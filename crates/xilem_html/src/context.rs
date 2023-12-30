use std::any::Any;

use bitflags::bitflags;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::Document;

use xilem_core::{Id, IdPath, VecSplice};

use crate::{
    app::AppRunner,
    diff::{diff_kv_iterables, Diff},
    vecmap::VecMap,
    AttributeValue, Message, Pod, ViewSequence,
};

type CowStr = std::borrow::Cow<'static, str>;

fn set_attribute(element: &web_sys::Element, name: &str, value: &str) {
    // we have to special-case `value` because setting the value using `set_attribute`
    // doesn't work after the value has been changed.
    if name == "value" {
        let element: &web_sys::HtmlInputElement = element.dyn_ref().unwrap_throw();
        element.set_value(value)
    } else if name == "checked" {
        let element: &web_sys::HtmlInputElement = element.dyn_ref().unwrap_throw();
        element.set_checked(true)
    } else {
        element.set_attribute(name, value).unwrap_throw();
    }
}

fn remove_attribute(element: &web_sys::Element, name: &str) {
    // we have to special-case `checked` because setting the value using `set_attribute`
    // doesn't work after the value has been changed.
    if name == "checked" {
        let element: &web_sys::HtmlInputElement = element.dyn_ref().unwrap_throw();
        element.set_checked(false)
    } else {
        element.remove_attribute(name).unwrap_throw();
    }
}

#[derive(Debug)]
enum TreeMutation {
    // EnterChildrenMarker is necessary for an optimization to accumulate multiple tree mutations in one element (e.g. TreeMutation::Skip(10))
    // Otherwise parent and child skips/deletions would be merged
    // This could contain extra information such as the parent id
    EnterChildrenMarker,
    Delete(usize),
    Skip(usize),
    Insert(Id),
}

// Note: xilem has derive Clone here. Not sure.
pub struct Cx {
    id_path: IdPath,
    document: Document,
    // TODO There's likely a cleaner more robust way to propagate the attributes to an element
    pub(crate) current_element_attributes: VecMap<CowStr, AttributeValue>,
    // Tree mutations are accumulated while traversing view sequences via the following stack
    // The stack is flushed (partially, for each element scope) in Cx::build_element_children and Cx::rebuild_element_children
    mutations: Vec<TreeMutation>,
    app_ref: Option<Box<dyn AppRunner>>,
}

pub struct MessageThunk {
    id_path: IdPath,
    app_ref: Box<dyn AppRunner>,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ChangeFlags: u32 {
        const STRUCTURE = 1;
        const OTHER_CHANGE = 2;
    }
}

impl Cx {
    pub fn new() -> Self {
        Cx {
            id_path: Vec::new(),
            document: crate::document(),
            app_ref: None,
            current_element_attributes: Default::default(),
            mutations: Vec::new(),
        }
    }

    pub fn push(&mut self, id: Id) {
        self.id_path.push(id);
    }

    pub fn pop(&mut self) {
        self.id_path.pop();
    }

    #[allow(unused)]
    pub fn id_path(&self) -> &IdPath {
        &self.id_path
    }

    /// Run some logic with an id added to the id path.
    ///
    /// This is an ergonomic helper that ensures proper nesting of the id path.
    pub fn with_id<T, F: FnOnce(&mut Cx) -> T>(&mut self, id: Id, f: F) -> T {
        self.push(id);
        let result = f(self);
        self.pop();
        result
    }

    /// Allocate a new id and run logic with the new id added to the id path.
    ///
    /// Also an ergonomic helper.
    pub fn with_new_id<T, F: FnOnce(&mut Cx) -> T>(&mut self, f: F) -> (Id, T) {
        let id = Id::next();
        self.push(id);
        let result = f(self);
        self.pop();
        (id, result)
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub(crate) fn build_element(
        &mut self,
        ns: &str,
        name: &str,
    ) -> (web_sys::Element, VecMap<CowStr, AttributeValue>) {
        let el = self
            .document
            .create_element_ns(Some(ns), name)
            .expect("could not create element");
        let attributes = self.apply_attributes(&el);
        (el, attributes)
    }

    pub fn build_element_children<T, A, Children: ViewSequence<T, A>>(
        &mut self,
        el: &web_sys::Element,
        children: &Children,
        child_elements: &mut Vec<Pod>,
    ) -> (Id, Children::State) {
        let mutation_start_idx = self.mutations.len();
        self.mutations.push(TreeMutation::EnterChildrenMarker);
        let id = Id::next();
        self.push(id);
        let state = children.build(self, child_elements);
        self.pop();
        // TODO do something with the extra information probably tree-structure tracking (should only be inserts)?
        self.mutations.truncate(mutation_start_idx);

        for child in child_elements {
            el.append_child(child.0.as_node_ref()).unwrap_throw();
        }

        (id, state)
    }

    pub(crate) fn rebuild_element(
        &mut self,
        element: &web_sys::Element,
        attributes: &mut VecMap<CowStr, AttributeValue>,
    ) -> ChangeFlags {
        self.apply_attribute_changes(element, attributes)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rebuild_element_children<T, A, Children: ViewSequence<T, A>>(
        &mut self,
        el: &web_sys::Element,
        id: Id,
        children: &Children,
        prev_children: &Children,
        children_states: &mut Children::State,
        child_elements: &mut Vec<Pod>,
        scratch: &mut Vec<Pod>,
    ) -> ChangeFlags {
        let mutation_start_idx = self.mutations.len();
        self.mutations.push(TreeMutation::EnterChildrenMarker);
        self.push(id);
        let mut splice = VecSplice::new(child_elements, scratch);
        let mut changeflags = children.rebuild(self, prev_children, children_states, &mut splice);
        self.pop();

        if changeflags.contains(ChangeFlags::STRUCTURE) {
            changeflags.remove(ChangeFlags::STRUCTURE);
            self.apply_tree_mutations(el, child_elements, mutation_start_idx);
        }
        self.mutations.truncate(mutation_start_idx);
        changeflags
    }

    fn apply_tree_mutations(
        &mut self,
        el: &web_sys::Element,
        child_elements: &[Pod],
        mutation_start_idx: usize,
    ) {
        let mut child_idx = 0;
        let node_list = el.child_nodes();
        let old_len = node_list.length() as usize;
        for mutation in &self.mutations[mutation_start_idx..] {
            match mutation {
                TreeMutation::Delete(count) => {
                    // Optimization in case all elements are deleted at once
                    if *count == old_len {
                        el.set_text_content(None);
                    } else {
                        for _ in 0..*count {
                            let child = node_list.get(child_idx).unwrap_throw();
                            el.remove_child(&child).unwrap_throw();
                        }
                    }
                }
                TreeMutation::Skip(count) => child_idx += *count as u32,
                TreeMutation::Insert(_) => {
                    let child = node_list.get(child_idx);
                    let child = child.as_deref().and_then(JsCast::dyn_ref);
                    el.insert_before(child_elements[child_idx as usize].0.as_node_ref(), child)
                        .unwrap_throw();
                    child_idx += 1;
                }
                TreeMutation::EnterChildrenMarker => (),
            }
        }
    }

    // TODO Not sure how multiple attribute definitions with the same name should be handled (e.g. `e.attr("class", "a").attr("class", "b")`)
    // Currently the outer most (in the example above "b") defines the attribute (when it isn't `None`, in that case the inner attr defines the value)
    pub(crate) fn add_new_attribute_to_current_element(
        &mut self,
        name: &CowStr,
        value: &Option<AttributeValue>,
    ) {
        if let Some(value) = value {
            // could be slightly optimized via something like this: `new_attrs.entry(name).or_insert_with(|| value)`
            if !self.current_element_attributes.contains_key(name) {
                self.current_element_attributes
                    .insert(name.clone(), value.clone());
            }
        }
    }

    pub(crate) fn apply_attributes(
        &mut self,
        element: &web_sys::Element,
    ) -> VecMap<CowStr, AttributeValue> {
        let mut attributes = VecMap::default();
        std::mem::swap(&mut attributes, &mut self.current_element_attributes);
        for (name, value) in attributes.iter() {
            set_attribute(element, name, &value.serialize());
        }
        attributes
    }

    pub(crate) fn apply_attribute_changes(
        &mut self,
        element: &web_sys::Element,
        attributes: &mut VecMap<CowStr, AttributeValue>,
    ) -> ChangeFlags {
        let mut changed = ChangeFlags::empty();
        // update attributes
        for itm in diff_kv_iterables(&*attributes, &self.current_element_attributes) {
            match itm {
                Diff::Add(name, value) | Diff::Change(name, value) => {
                    set_attribute(element, name, &value.serialize());
                    changed |= ChangeFlags::OTHER_CHANGE;
                }
                Diff::Remove(name) => {
                    remove_attribute(element, name);
                    changed |= ChangeFlags::OTHER_CHANGE;
                }
            }
        }
        std::mem::swap(attributes, &mut self.current_element_attributes);
        self.current_element_attributes.clear();
        changed
    }

    pub fn message_thunk(&self) -> MessageThunk {
        MessageThunk {
            id_path: self.id_path.clone(),
            app_ref: self.app_ref.as_ref().unwrap().clone_box(),
        }
    }
    pub(crate) fn set_runner(&mut self, runner: impl AppRunner + 'static) {
        self.app_ref = Some(Box::new(runner));
    }

    pub fn skip_child(&mut self) {
        if let Some(TreeMutation::Skip(n)) = self.mutations.last_mut() {
            *n += 1;
        } else {
            self.mutations.push(TreeMutation::Skip(1));
        }
    }

    pub fn add_child(&mut self, id: Id) {
        self.mutations.push(TreeMutation::Insert(id));
    }

    pub fn delete_children(&mut self, count: usize) {
        if let Some(TreeMutation::Delete(n)) = self.mutations.last_mut() {
            *n += count;
        } else {
            self.mutations.push(TreeMutation::Delete(count));
        }
    }

    // TODO separate variant for id/element change?
    pub fn child_changed(&mut self, id_before: Id, new_id: Id, _changeflags: ChangeFlags) {
        if id_before != new_id {
            self.delete_children(1);
            self.add_child(new_id);
        } else {
            self.skip_child();
        }
    }
}

impl Default for Cx {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageThunk {
    pub fn push_message(&self, message_body: impl Any + 'static) {
        let message = Message {
            id_path: self.id_path.clone(),
            body: Box::new(message_body),
        };
        self.app_ref.handle_message(message);
    }
}

impl ChangeFlags {
    pub fn tree_structure() -> Self {
        Self::STRUCTURE
    }
}
