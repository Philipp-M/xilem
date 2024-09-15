use crate::AnyPod;
use crate::{core::AppendVec, vec_splice::VecSplice};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::UnwrapThrowExt as _;

#[allow(variant_size_differences)]
enum ChildMutation {
    Delete(u32),
    Insert(AnyPod),
    Skip(u32),
}

// #[derive(Clone, PartialEq, Debug)]
struct ElementStackIdx {
    dirty: bool,
    idx: usize,
    last_mutation: Option<ChildMutation>,
    hydrate: bool,
}

pub struct TreeMutations {
    mutation_stack: Vec<ChildMutation>,
    element_stack_indices: Vec<ElementStackIdx>,
    last_mutation: Option<ChildMutation>,
    dirty: bool,
    start_idx: usize,
    hydrate: bool,
    pub(crate) append_scratches: Vec<Rc<RefCell<AppendVec<AnyPod>>>>,
    pub(crate) scratch: Vec<AnyPod>,
    document_fragment: web_sys::DocumentFragment,
    range: web_sys::Range,
    current_append_scratch_idx: usize,
}

impl Default for TreeMutations {
    fn default() -> Self {
        Self {
            mutation_stack: Default::default(),
            element_stack_indices: Default::default(),
            scratch: Default::default(),
            last_mutation: None,
            start_idx: 0,
            dirty: false,
            hydrate: false,
            document_fragment: crate::document().create_document_fragment(),
            range: crate::document().create_range().unwrap_throw(),
            append_scratches: Default::default(),
            current_append_scratch_idx: 0,
        }
    }
}

impl TreeMutations {
    pub fn get_append_scratch(&mut self) -> Rc<RefCell<AppendVec<AnyPod>>> {
        if let Some(append_scratch) = self.append_scratches.get(self.current_append_scratch_idx) {
            self.current_append_scratch_idx += 1;
            Rc::clone(append_scratch)
        } else {
            self.current_append_scratch_idx += 1;
            let append_scratch = Default::default();
            self.append_scratches.push(Rc::clone(&append_scratch));
            append_scratch
        }
    }
    pub fn return_append_scratch(&mut self, scratch: Rc<RefCell<AppendVec<AnyPod>>>) {
        self.current_append_scratch_idx -= 1;
        debug_assert!(Rc::ptr_eq(
            &self.append_scratches[self.current_append_scratch_idx],
            &scratch
        ));
    }

    pub fn push(&mut self, hydrate: bool) {
        self.element_stack_indices.push(ElementStackIdx {
            idx: self.start_idx,
            dirty: self.dirty,
            last_mutation: self.last_mutation.take(),
            hydrate: self.hydrate,
        });
        self.hydrate = hydrate;
        self.start_idx = self.mutation_stack.len();
    }

    fn apply_mutation(&mut self, mutation: ChildMutation) {
        match (&mut self.last_mutation, mutation) {
            (Some(ChildMutation::Delete(current_count)), ChildMutation::Delete(count))
            | (Some(ChildMutation::Skip(current_count)), ChildMutation::Skip(count)) => {
                *current_count += count;
            }
            (_, mutation) => {
                if let Some(mutation) = self.last_mutation.take() {
                    self.mutation_stack.push(mutation);
                    self.dirty = true;
                }
                self.last_mutation = Some(mutation);
            }
        }
    }

    pub fn delete(&mut self, count: u32) {
        self.apply_mutation(ChildMutation::Delete(count));
        self.dirty = true;
    }

    pub fn skip(&mut self, count: u32) {
        self.apply_mutation(ChildMutation::Skip(count));
    }

    pub fn insert(&mut self, element: AnyPod) {
        self.apply_mutation(ChildMutation::Insert(element));
        self.dirty = true;
    }

    pub fn pop_and_apply_mutations(&mut self, node: &web_sys::Node, children: &mut Vec<AnyPod>) {
        if self.dirty {
            if let Some(mutation) = self.last_mutation.take() {
                self.mutation_stack.push(mutation);
            }
            let previous_len = children.len() as u32;
            let mut splice = VecSplice::new(children, &mut self.scratch);
            let mut idx = 0;
            let mut insertion_start = idx;
            let mut changes = self.mutation_stack.drain(self.start_idx..).peekable();
            while let Some(change) = changes.next() {
                match change {
                    ChildMutation::Delete(count) => {
                        // console::log_2(&"delete".into(), &count.into());
                        if previous_len == count {
                            node.set_text_content(None);
                            children.clear();
                            return; // nothing else to be done here...
                        } else {
                            splice.delete(count as usize);
                            let nodes = node.child_nodes();
                            self.range
                                .set_start_before(&nodes.get(idx).unwrap_throw())
                                .unwrap_throw();
                            self.range
                                .set_end_after(&nodes.get(idx + count - 1).unwrap_throw())
                                .unwrap_throw();
                            self.range.delete_contents().unwrap_throw();
                        }
                    }
                    ChildMutation::Insert(child) => {
                        // console::log_2(&"insert".into(), &insertion_start.into());
                        if !self.hydrate {
                            self.document_fragment
                                .append_child(child.as_ref())
                                .unwrap_throw();
                            if !matches!(changes.peek(), Some(ChildMutation::Insert(_next))) {
                                // console::log_1(&insertion_start.into());
                                let begin_child = node.child_nodes().get(insertion_start);
                                node.insert_before(&self.document_fragment, begin_child.as_ref())
                                    .unwrap_throw();
                                insertion_start = idx;
                            }
                        }
                        splice.insert(child);
                        idx += 1;
                    }
                    ChildMutation::Skip(count) => {
                        // console::log_2(&"skip".into(), &count.to_string().into());
                        idx += count;
                        insertion_start = idx;
                        splice.skip(count as usize);
                    }
                }
            }
        }
        if let Some(ElementStackIdx {
            dirty,
            idx,
            last_mutation,
            hydrate,
        }) = self.element_stack_indices.pop()
        {
            self.dirty = dirty;
            self.start_idx = idx;
            self.hydrate = hydrate;
            self.last_mutation = last_mutation;
        }

        debug_assert_eq!(self.document_fragment.child_element_count(), 0);
        debug_assert_eq!(self.scratch.len(), 0);
    }
}
