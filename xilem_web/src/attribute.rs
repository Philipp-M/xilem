// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use xilem_core::{MessageResult, Mut, View, ViewElement, ViewId, ViewMarker};

use crate::{
    element_props::ElementScratch, vecmap::VecMap, AttributeValue, DomNode, DynMessage,
    ElementProps, Pod, PodMut, ViewCtx,
};

type CowStr = std::borrow::Cow<'static, str>;

/// This trait allows (modifying) HTML/SVG/MathML attributes on DOM [`Element`](`crate::interfaces::Element`)s.
///
/// Modifications have to be done on the up-traversal of [`View::rebuild`], i.e. after [`View::rebuild`] was invoked for descendent views.
/// See [`Attr::build`] and [`Attr::rebuild`], how to use this for [`ViewElement`]s that implement this trait.
/// When these methods are used, they have to be used in every reconciliation pass (i.e. [`View::rebuild`]).
pub trait WithAttributes {
    /// Needs to be invoked within a [`View::rebuild`] before traversing to descendent views, and before any modifications (with [`set_attribute`](`WithAttributes::set_attribute`)) are done in that view
    fn rebuild_attribute_modifier(&mut self);

    /// Needs to be invoked after any modifications are done
    fn mark_end_of_attribute_modifier(&mut self);

    /// Sets or removes (when value is `None`) an attribute from the underlying element.
    ///
    /// When in [`View::rebuild`] this has to be invoked *after* traversing the inner `View` with [`View::rebuild`]
    fn set_attribute(&mut self, name: CowStr, value: Option<AttributeValue>);

    // TODO first find a use-case for this...
    // fn get_attr(&self, name: &str) -> Option<&AttributeValue>;
}

#[derive(Debug, PartialEq)]
enum AttributeModifier {
    Remove(CowStr),
    Set(CowStr, AttributeValue),
    EndMarker(u16),
}

/// A shared storage, which is temporarily used for updating attributes on an element
#[derive(Debug, Default)]
pub struct AttributeScratch {
    updated: VecMap<CowStr, ()>,
}

/// This contains all the current attributes of an [`Element`](`crate::interfaces::Element`)
#[derive(Debug)]
pub struct Attributes {
    modifiers: Vec<AttributeModifier>,
    scratch: Rc<RefCell<ElementScratch>>,
    idx: u16,
    start_idx: u16,
}

#[cfg(feature = "hydration")]
impl Attributes {
    pub(crate) fn new(scratch: Rc<RefCell<ElementScratch>>, capacity: usize) -> Self {
        Self {
            scratch,
            modifiers: Vec::with_capacity(capacity),
            idx: 0,
            start_idx: 0,
        }
    }
}

fn set_attribute(element: &web_sys::Element, name: &str, value: &str) {
    debug_assert_ne!(
        name, "class",
        "Using `class` as attribute is not supported, use the `el.class()` modifier instead"
    );
    debug_assert_ne!(
        name, "style",
        "Using `style` as attribute is not supported, use the `el.style()` modifier instead"
    );

    // we have to special-case `value` because setting the value using `set_attribute`
    // doesn't work after the value has been changed.
    // TODO not sure, whether this is always a good idea, in case custom or other interfaces such as HtmlOptionElement elements are used that have "value" as an attribute name.
    // We likely want to use the DOM attributes instead.
    if name == "value" {
        if let Some(input_element) = element.dyn_ref::<web_sys::HtmlInputElement>() {
            input_element.set_value(value);
        } else {
            element.set_attribute("value", value).unwrap_throw();
        }
    } else if name == "checked" {
        if let Some(input_element) = element.dyn_ref::<web_sys::HtmlInputElement>() {
            input_element.set_checked(true);
        } else {
            element.set_attribute("checked", value).unwrap_throw();
        }
    } else {
        element.set_attribute(name, value).unwrap_throw();
    }
}

fn remove_attribute(element: &web_sys::Element, name: &str) {
    debug_assert_ne!(
        name, "class",
        "Using `class` as attribute is not supported, use the `el.class()` modifier instead"
    );
    debug_assert_ne!(
        name, "style",
        "Using `style` as attribute is not supported, use the `el.style()` modifier instead"
    );
    // we have to special-case `checked` because setting the value using `set_attribute`
    // doesn't work after the value has been changed.
    if name == "checked" {
        if let Some(input_element) = element.dyn_ref::<web_sys::HtmlInputElement>() {
            input_element.set_checked(false);
        } else {
            element.remove_attribute("checked").unwrap_throw();
        }
    } else {
        element.remove_attribute(name).unwrap_throw();
    }
}

impl Attributes {
    /// applies potential changes of the attributes of an element to the underlying DOM node
    pub fn apply_attribute_changes(&mut self, element: &web_sys::Element) {
        let mut scratch = self.scratch.borrow_mut();
        #[cfg(feature = "hydration")]
        if scratch.in_hydration {
            debug_assert!(scratch.attributes.updated.is_empty());
            return;
        }

        // optimization to avoid unnecessary work
        if scratch.was_created {
            debug_assert!(scratch.attributes.updated.is_empty());
            for modifier in self.modifiers.iter() {
                match modifier {
                    AttributeModifier::Remove(name) => {
                        remove_attribute(element, name);
                    }
                    AttributeModifier::Set(name, value) => {
                        set_attribute(element, name, &value.serialize());
                    }
                    AttributeModifier::EndMarker(_) => (),
                }
            }
        } else if !scratch.attributes.updated.is_empty() {
            for modifier in self.modifiers.iter().rev() {
                match modifier {
                    AttributeModifier::Remove(name) => {
                        if scratch.attributes.updated.remove(name).is_some() {
                            remove_attribute(element, name);
                        }
                    }
                    AttributeModifier::Set(name, value) => {
                        if scratch.attributes.updated.remove(name).is_some() {
                            set_attribute(element, name, &value.serialize());
                        }
                    }
                    AttributeModifier::EndMarker(_) => (),
                }
            }
            debug_assert!(scratch.attributes.updated.is_empty());
        }
    }
}

impl WithAttributes for Attributes {
    fn set_attribute(&mut self, name: CowStr, value: Option<AttributeValue>) {
        let new_modifier = if let Some(value) = value {
            AttributeModifier::Set(name.clone(), value)
        } else {
            AttributeModifier::Remove(name.clone())
        };

        let mut scratch = self.scratch.borrow_mut();
        if scratch.was_initialized() {
            self.modifiers.push(new_modifier);
        } else if let Some(modifier) = self.modifiers.get_mut(self.idx as usize) {
            if modifier != &new_modifier {
                if let AttributeModifier::Remove(previous_name)
                | AttributeModifier::Set(previous_name, _) = modifier
                {
                    if &name != previous_name {
                        scratch.attributes.updated.insert(previous_name.clone(), ());
                    }
                }
                scratch.attributes.updated.insert(name, ());
                *modifier = new_modifier;
            }
            // else remove it out of scratch.attributes.updated? (because previous attributes are overwritten) not sure if worth it because potentially worse perf
        } else {
            scratch.attributes.updated.insert(name, ());
            self.modifiers.push(new_modifier);
        }
        self.idx += 1;
    }

    fn rebuild_attribute_modifier(&mut self) {
        if self.idx == 0 {
            self.start_idx = 0;
        } else {
            let AttributeModifier::EndMarker(start_idx) = self.modifiers[(self.idx - 1) as usize]
            else {
                unreachable!("this should not happen, as either `rebuild_attribute_modifier` happens first, or follows an `mark_end_of_attribute_modifier`")
            };
            self.idx = start_idx;
            self.start_idx = start_idx;
        }
    }

    fn mark_end_of_attribute_modifier(&mut self) {
        match self.modifiers.get_mut(self.idx as usize) {
            Some(AttributeModifier::EndMarker(prev_start_idx))
                if *prev_start_idx == self.start_idx => {} // attribute modifier hasn't changed
            Some(modifier) => {
                *modifier = AttributeModifier::EndMarker(self.start_idx);
            }
            None => {
                self.modifiers
                    .push(AttributeModifier::EndMarker(self.start_idx));
            }
        }
        self.idx += 1;
        self.start_idx = self.idx;
    }
}

impl WithAttributes for ElementProps {
    fn rebuild_attribute_modifier(&mut self) {
        self.attributes().rebuild_attribute_modifier();
    }

    fn mark_end_of_attribute_modifier(&mut self) {
        self.attributes().mark_end_of_attribute_modifier();
    }

    fn set_attribute(&mut self, name: CowStr, value: Option<AttributeValue>) {
        self.attributes().set_attribute(name, value);
    }
}

impl<E: DomNode<P>, P: WithAttributes> WithAttributes for Pod<E, P> {
    fn rebuild_attribute_modifier(&mut self) {
        self.props.rebuild_attribute_modifier();
    }

    fn mark_end_of_attribute_modifier(&mut self) {
        self.props.mark_end_of_attribute_modifier();
    }

    fn set_attribute(&mut self, name: CowStr, value: Option<AttributeValue>) {
        self.props.set_attribute(name, value);
    }
}

impl<E: DomNode<P>, P: WithAttributes> WithAttributes for PodMut<'_, E, P> {
    fn rebuild_attribute_modifier(&mut self) {
        self.props.rebuild_attribute_modifier();
    }

    fn mark_end_of_attribute_modifier(&mut self) {
        self.props.mark_end_of_attribute_modifier();
    }

    fn set_attribute(&mut self, name: CowStr, value: Option<AttributeValue>) {
        self.props.set_attribute(name, value);
    }
}

/// Syntax sugar for adding a type bound on the `ViewElement` of a view, such that both, [`ViewElement`] and [`ViewElement::Mut`] are bound to [`WithAttributes`]
pub trait ElementWithAttributes:
    for<'a> ViewElement<Mut<'a>: WithAttributes> + WithAttributes
{
}

impl<T> ElementWithAttributes for T
where
    T: ViewElement + WithAttributes,
    for<'a> T::Mut<'a>: WithAttributes,
{
}

/// A view to add or remove an attribute to/from an element, see [`Element::attr`](`crate::interfaces::Element::attr`) for how it's usually used.
#[derive(Clone, Debug)]
pub struct Attr<E, T, A> {
    el: E,
    name: CowStr,
    value: Option<AttributeValue>,
    phantom: PhantomData<fn() -> (T, A)>,
}

impl<E, T, A> Attr<E, T, A> {
    pub fn new(el: E, name: CowStr, value: Option<AttributeValue>) -> Self {
        Attr {
            el,
            name,
            value,
            phantom: PhantomData,
        }
    }
}

impl<E, T, A> ViewMarker for Attr<E, T, A> {}
impl<E, T, A> View<T, A, ViewCtx, DynMessage> for Attr<E, T, A>
where
    T: 'static,
    A: 'static,
    E: View<T, A, ViewCtx, DynMessage, Element: ElementWithAttributes>,
{
    type Element = E::Element;

    type ViewState = E::ViewState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        ctx.add_modifier_size_hint::<Attributes>(1);
        let (mut element, state) = self.el.build(ctx);
        element.set_attribute(self.name.clone(), self.value.clone());
        element.mark_end_of_attribute_modifier();
        (element, state)
    }

    fn rebuild<'e>(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'e, Self::Element>,
    ) -> Mut<'e, Self::Element> {
        element.rebuild_attribute_modifier();
        let mut element = self.el.rebuild(&prev.el, view_state, ctx, element);
        element.set_attribute(self.name.clone(), self.value.clone());
        element.mark_end_of_attribute_modifier();
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
