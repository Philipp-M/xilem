// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use xilem_core::{MessageResult, Mut, View, ViewElement, ViewId, ViewMarker};

use crate::{
    element_props::ElementScratch, vecmap::VecMap, DomNode, DynMessage, ElementProps, Pod, PodMut,
    ViewCtx,
};

type CowStr = std::borrow::Cow<'static, str>;

/// Types implementing this trait can be used in the [`Class`] view, see also [`Element::class`](`crate::interfaces::Element::class`)
pub trait AsClassIter {
    fn class_iter(&self) -> impl Iterator<Item = CowStr>;
}

impl<C: AsClassIter> AsClassIter for Option<C> {
    fn class_iter(&self) -> impl Iterator<Item = CowStr> {
        self.iter().flat_map(|c| c.class_iter())
    }
}

impl AsClassIter for String {
    fn class_iter(&self) -> impl Iterator<Item = CowStr> {
        std::iter::once(self.clone().into())
    }
}

impl AsClassIter for &'static str {
    fn class_iter(&self) -> impl Iterator<Item = CowStr> {
        std::iter::once(CowStr::from(*self))
    }
}

impl AsClassIter for CowStr {
    fn class_iter(&self) -> impl Iterator<Item = CowStr> {
        std::iter::once(self.clone())
    }
}

impl<T> AsClassIter for Vec<T>
where
    T: AsClassIter,
{
    fn class_iter(&self) -> impl Iterator<Item = CowStr> {
        self.iter().flat_map(|c| c.class_iter())
    }
}

impl<T: AsClassIter, const N: usize> AsClassIter for [T; N] {
    fn class_iter(&self) -> impl Iterator<Item = CowStr> {
        self.iter().flat_map(|c| c.class_iter())
    }
}

/// This trait enables having classes (via `className`) on DOM [`Element`](`crate::interfaces::Element`)s. It is used within [`View`]s that modify the classes of an element.
///
/// Modifications have to be done on the up-traversal of [`View::rebuild`], i.e. after [`View::rebuild`] was invoked for descendent views.
/// See [`Class::build`] and [`Class::rebuild`], how to use this for [`ViewElement`]s that implement this trait.
/// When these methods are used, they have to be used in every reconciliation pass (i.e. [`View::rebuild`]).
pub trait WithClasses {
    /// Needs to be invoked within a [`View::rebuild`] before traversing to descendent views, and before any modifications (with [`add_class`](`WithClasses::add_class`) or [`remove_class`](`WithClasses::remove_class`)) are done in that view
    fn rebuild_class_modifier(&mut self);

    /// Needs to be invoked after any modifications are done
    fn mark_end_of_class_modifier(&mut self);

    /// Adds a class to the element
    ///
    /// When in [`View::rebuild`] this has to be invoked *after* traversing the inner `View` with [`View::rebuild`]
    fn add_class(&mut self, class_name: CowStr);

    /// Removes a possibly previously added class from the element
    ///
    /// When in [`View::rebuild`] this has to be invoked *after* traversing the inner `View` with [`View::rebuild`]
    fn remove_class(&mut self, class_name: CowStr);

    // TODO something like the following, but I'm not yet sure how to support that efficiently (and without much binary bloat)
    // The modifiers possibly have to be applied then...
    // fn classes(&self) -> impl Iterator<CowStr>;
    // maybe also something like:
    // fn has_class(&self, class_name: &str) -> bool
    // Need to find a use-case for this first though (i.e. a modifier needs to read previously added classes)
}

#[derive(Debug)]
enum ClassModifier {
    Remove(CowStr),
    Add(CowStr),
    EndMarker(u16),
}

#[derive(Debug, Default)]
/// A shared storage, which is temporarily used for updating classes on an element
pub struct ClassScratch {
    // TODO maybe this attribute is redundant and can be formed just from the class_modifiers attribute
    classes: VecMap<CowStr, ()>,
    class_name: String,
    dirty: bool,
}

/// This contains all the current classes of an [`Element`](`crate::interfaces::Element`)
#[derive(Debug)]
pub struct Classes {
    modifiers: Vec<ClassModifier>,
    scratch: Rc<RefCell<ElementScratch>>,
    idx: u16,
    start_idx: u16,
}

#[cfg(feature = "hydration")]
impl Classes {
    pub(crate) fn new(scratch: Rc<RefCell<ElementScratch>>, capacity: usize) -> Self {
        Self {
            scratch,
            modifiers: Vec::with_capacity(capacity),
            idx: 0,
            start_idx: 0,
        }
    }
}

impl Classes {
    pub fn apply_class_changes(&mut self, element: &web_sys::Element) {
        let mut scratch = self.scratch.borrow_mut();
        #[cfg(feature = "hydration")]
        if scratch.in_hydration {
            scratch.class.dirty = false;
            return;
        }

        if scratch.class.dirty {
            scratch.class.dirty = false;
            scratch.class.classes.clear();
            for modifier in &self.modifiers {
                match modifier {
                    ClassModifier::Remove(class_name) => {
                        scratch.class.classes.remove(class_name);
                    }
                    ClassModifier::Add(class_name) => {
                        scratch.class.classes.insert(class_name.clone(), ());
                    }
                    ClassModifier::EndMarker(_) => (),
                }
            }
            // intersperse would be the right way to do this, but avoid extra dependencies just for this (and otherwise it's unstable in std)...
            scratch.class.class_name.clear();
            let last_idx = scratch.class.classes.len().saturating_sub(1);
            let scratch = &mut *scratch; // help the borrow-checker
            for (idx, class) in scratch.class.classes.keys().enumerate() {
                scratch.class.class_name += class;
                if idx != last_idx {
                    scratch.class.class_name += " ";
                }
            }
            // Svg elements do have issues with className, see https://developer.mozilla.org/en-US/docs/Web/API/Element/className
            if element.dyn_ref::<web_sys::SvgElement>().is_some() {
                element
                    .set_attribute("class", &scratch.class.class_name)
                    .unwrap_throw();
            } else {
                element.set_class_name(&scratch.class.class_name);
            }
        }
    }
}

impl WithClasses for Classes {
    fn rebuild_class_modifier(&mut self) {
        if self.idx == 0 {
            self.start_idx = 0;
        } else {
            let ClassModifier::EndMarker(start_idx) = self.modifiers[(self.idx - 1) as usize]
            else {
                unreachable!("this should not happen, as either `rebuild_class_modifier` is happens first, or follows an `mark_end_of_class_modifier`")
            };
            self.idx = start_idx;
            self.start_idx = start_idx;
        }
    }

    fn mark_end_of_class_modifier(&mut self) {
        match self.modifiers.get_mut(self.idx as usize) {
            Some(ClassModifier::EndMarker(idx)) if *idx == self.start_idx => (),
            Some(modifier) => {
                self.scratch.borrow_mut().class.dirty = true;
                *modifier = ClassModifier::EndMarker(self.start_idx);
            }
            None => {
                self.scratch.borrow_mut().class.dirty = true;
                self.modifiers
                    .push(ClassModifier::EndMarker(self.start_idx));
            }
        }
        self.idx += 1;
        self.start_idx = self.idx;
    }

    fn add_class(&mut self, class_name: CowStr) {
        match self.modifiers.get_mut(self.idx as usize) {
            Some(ClassModifier::Add(class)) if class == &class_name => (), // class modifier hasn't changed
            Some(modifier) => {
                self.scratch.borrow_mut().class.dirty = true;
                *modifier = ClassModifier::Add(class_name);
            }
            None => {
                self.scratch.borrow_mut().class.dirty = true;
                self.modifiers.push(ClassModifier::Add(class_name));
            }
        }
        self.idx += 1;
    }

    fn remove_class(&mut self, class_name: CowStr) {
        // Same code as add_class but with remove...
        match self.modifiers.get_mut(self.idx as usize) {
            Some(ClassModifier::Remove(class)) if class == &class_name => (), // class modifier hasn't changed
            Some(modifier) => {
                self.scratch.borrow_mut().class.dirty = true;
                *modifier = ClassModifier::Remove(class_name);
            }
            None => {
                self.scratch.borrow_mut().class.dirty = true;
                self.modifiers.push(ClassModifier::Remove(class_name));
            }
        }
        self.idx += 1;
    }
}

impl WithClasses for ElementProps {
    fn rebuild_class_modifier(&mut self) {
        self.classes().rebuild_class_modifier();
    }

    fn mark_end_of_class_modifier(&mut self) {
        self.classes().mark_end_of_class_modifier();
    }

    fn add_class(&mut self, class_name: CowStr) {
        self.classes().add_class(class_name);
    }

    fn remove_class(&mut self, class_name: CowStr) {
        self.classes().remove_class(class_name);
    }
}

impl<E: DomNode<P>, P: WithClasses> WithClasses for Pod<E, P> {
    fn rebuild_class_modifier(&mut self) {
        self.props.rebuild_class_modifier();
    }

    fn mark_end_of_class_modifier(&mut self) {
        self.props.mark_end_of_class_modifier();
    }

    fn add_class(&mut self, class_name: CowStr) {
        self.props.add_class(class_name);
    }

    fn remove_class(&mut self, class_name: CowStr) {
        self.props.remove_class(class_name);
    }
}

impl<E: DomNode<P>, P: WithClasses> WithClasses for PodMut<'_, E, P> {
    fn rebuild_class_modifier(&mut self) {
        self.props.rebuild_class_modifier();
    }

    fn mark_end_of_class_modifier(&mut self) {
        self.props.mark_end_of_class_modifier();
    }

    fn add_class(&mut self, class_name: CowStr) {
        self.props.add_class(class_name);
    }

    fn remove_class(&mut self, class_name: CowStr) {
        self.props.remove_class(class_name);
    }
}

/// Syntax sugar for adding a type bound on the `ViewElement` of a view, such that both, [`ViewElement`] and [`ViewElement::Mut`] are bound to [`WithClasses`]
pub trait ElementWithClasses: for<'a> ViewElement<Mut<'a>: WithClasses> + WithClasses {}

impl<T> ElementWithClasses for T
where
    T: ViewElement + WithClasses,
    for<'a> T::Mut<'a>: WithClasses,
{
}

/// A view to add classes to elements
#[derive(Clone, Debug)]
pub struct Class<E, C, T, A> {
    el: E,
    classes: C,
    phantom: PhantomData<fn() -> (T, A)>,
}

impl<E, C, T, A> Class<E, C, T, A> {
    pub fn new(el: E, classes: C) -> Self {
        Class {
            el,
            classes,
            phantom: PhantomData,
        }
    }
}

impl<E, C, T, A> ViewMarker for Class<E, C, T, A> {}
impl<E, C, T, A> View<T, A, ViewCtx, DynMessage> for Class<E, C, T, A>
where
    T: 'static,
    A: 'static,
    C: AsClassIter + 'static,
    E: View<T, A, ViewCtx, DynMessage, Element: ElementWithClasses>,
{
    type Element = E::Element;

    type ViewState = E::ViewState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        let class_iter = self.classes.class_iter();
        ctx.add_modifier_size_hint::<Classes>(class_iter.size_hint().0);
        let (mut e, s) = self.el.build(ctx);
        for class in class_iter {
            e.add_class(class);
        }
        e.mark_end_of_class_modifier();
        (e, s)
    }

    fn rebuild<'e>(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'e, Self::Element>,
    ) -> Mut<'e, Self::Element> {
        // This has to happen, before any children are rebuilt, otherwise this state machine breaks...
        // The actual modifiers also have to happen after the children are rebuilt, see `add_class` below.
        element.rebuild_class_modifier();
        let mut element = self.el.rebuild(&prev.el, view_state, ctx, element);
        for class in self.classes.class_iter() {
            element.add_class(class);
        }
        element.mark_end_of_class_modifier();
        element
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: Mut<'_, Self::Element>,
    ) {
        self.el.teardown(view_state, ctx, element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[ViewId],
        message: DynMessage,
        app_state: &mut T,
    ) -> MessageResult<A, DynMessage> {
        self.el.message(view_state, id_path, message, app_state)
    }
}
