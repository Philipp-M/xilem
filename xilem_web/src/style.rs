// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    rc::Rc,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use xilem_core::{MessageResult, Mut, View, ViewElement, ViewId, ViewMarker};

use crate::{
    element_props::ElementScratch, vecmap::VecMap, DomNode, DynMessage, ElementProps, Pod, PodMut,
    ViewCtx,
};

type CowStr = std::borrow::Cow<'static, str>;

/// A trait to make the class adding functions generic over collection type
pub trait IntoStyles {
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>);
}

struct StyleTuple<T1, T2>(T1, T2);

/// Create a style from a style name and its value.
pub fn style<T1, T2>(name: T1, value: T2) -> impl IntoStyles
where
    T1: Into<CowStr>,
    T2: Into<CowStr>,
{
    StyleTuple(name, value)
}

impl<T1, T2> IntoStyles for StyleTuple<T1, T2>
where
    T1: Into<CowStr>,
    T2: Into<CowStr>,
{
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>) {
        let StyleTuple(key, value) = self;
        styles.push((key.into(), value.into()));
    }
}

impl<T> IntoStyles for Option<T>
where
    T: IntoStyles,
{
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>) {
        if let Some(t) = self {
            t.into_styles(styles);
        }
    }
}

impl<T> IntoStyles for Vec<T>
where
    T: IntoStyles,
{
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>) {
        for itm in self {
            itm.into_styles(styles);
        }
    }
}

impl<T: IntoStyles, const N: usize> IntoStyles for [T; N] {
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>) {
        for itm in self {
            itm.into_styles(styles);
        }
    }
}

impl<T1, T2, S> IntoStyles for HashMap<T1, T2, S>
where
    T1: Into<CowStr>,
    T2: Into<CowStr>,
{
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>) {
        for (key, value) in self {
            styles.push((key.into(), value.into()));
        }
    }
}

impl<T1, T2> IntoStyles for BTreeMap<T1, T2>
where
    T1: Into<CowStr>,
    T2: Into<CowStr>,
{
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>) {
        for (key, value) in self {
            styles.push((key.into(), value.into()));
        }
    }
}

impl<T1, T2> IntoStyles for VecMap<T1, T2>
where
    T1: Into<CowStr>,
    T2: Into<CowStr>,
{
    fn into_styles(self, styles: &mut Vec<(CowStr, CowStr)>) {
        for (key, value) in self {
            styles.push((key.into(), value.into()));
        }
    }
}

/// This trait allows (modifying) the `style` property of `HTMLElement`/`SVGElement`s, used in the DOM interface traits [`HtmlElement`](`crate::interfaces::HtmlElement`) and [`SvgElement`](`crate::interfaces::SvgElement`).
///
/// Modifications have to be done on the up-traversal of [`View::rebuild`], i.e. after [`View::rebuild`] was invoked for descendent views.
/// See [`Style::build`] and [`Style::rebuild`], how to use this for [`ViewElement`]s that implement this trait.
/// When these methods are used, they have to be used in every reconciliation pass (i.e. [`View::rebuild`]).
pub trait WithStyle {
    /// Needs to be invoked within a [`View::rebuild`] before traversing to descendent views, and before any modifications (with [`set_style`](`WithStyle::set_style`)) are done in that view
    fn rebuild_style_modifier(&mut self);

    /// Needs to be invoked after any modifications are done
    fn mark_end_of_style_modifier(&mut self);

    /// Sets or removes (when value is `None`) a style property from the underlying element.
    ///
    /// When in [`View::rebuild`] this has to be invoked *after* traversing the inner `View` with [`View::rebuild`]
    fn set_style(&mut self, name: CowStr, value: Option<CowStr>);

    // TODO first find a use-case for this...
    // fn get_style(&self, name: &str) -> Option<&CowStr>;
}

#[derive(Debug, PartialEq)]
enum StyleModifier {
    Remove(CowStr),
    Set(CowStr, CowStr),
    EndMarker(u16),
}

#[derive(Debug, Default)]
/// A shared storage, which is temporarily used for updating styles on an element
pub struct StyleScratch {
    updated: VecMap<CowStr, ()>,
}

#[derive(Debug, Default)]
/// This contains all the current style properties of an [`HtmlElement`](`crate::interfaces::Element`) or [`SvgElement`](`crate::interfaces::SvgElement`).
pub struct Styles {
    modifiers: Vec<StyleModifier>,
    scratch: Rc<RefCell<ElementScratch>>,
    idx: u16,
    start_idx: u16,
}

#[cfg(feature = "hydration")]
impl Styles {
    pub(crate) fn new(scratch: Rc<RefCell<ElementScratch>>, capacity: usize) -> Self {
        Self {
            scratch,
            modifiers: Vec::with_capacity(capacity),
            idx: 0,
            start_idx: 0,
        }
    }
}

fn set_style(element: &web_sys::Element, name: &str, value: &str) {
    if let Some(el) = element.dyn_ref::<web_sys::HtmlElement>() {
        el.style().set_property(name, value).unwrap_throw();
    } else if let Some(el) = element.dyn_ref::<web_sys::SvgElement>() {
        el.style().set_property(name, value).unwrap_throw();
    }
}

fn remove_style(element: &web_sys::Element, name: &str) {
    if let Some(el) = element.dyn_ref::<web_sys::HtmlElement>() {
        el.style().remove_property(name).unwrap_throw();
    } else if let Some(el) = element.dyn_ref::<web_sys::SvgElement>() {
        el.style().remove_property(name).unwrap_throw();
    }
}

impl Styles {
    pub fn apply_style_changes(&mut self, element: &web_sys::Element) {
        let mut scratch = self.scratch.borrow_mut();
        #[cfg(feature = "hydration")]
        if scratch.in_hydration {
            debug_assert!(scratch.styles.updated.is_empty());
            return;
        }

        // optimization to avoid unnecessary work
        if scratch.was_created {
            debug_assert!(scratch.styles.updated.is_empty());
            for modifier in self.modifiers.iter() {
                match modifier {
                    StyleModifier::Remove(name) => {
                        remove_style(element, name);
                    }
                    StyleModifier::Set(name, value) => {
                        set_style(element, name, value);
                    }
                    StyleModifier::EndMarker(_) => (),
                }
            }
        } else if !scratch.styles.updated.is_empty() {
            for modifier in self.modifiers.iter().rev() {
                match modifier {
                    StyleModifier::Remove(name) => {
                        if scratch.styles.updated.remove(name).is_some() {
                            remove_style(element, name);
                        }
                    }
                    StyleModifier::Set(name, value) => {
                        if scratch.styles.updated.remove(name).is_some() {
                            set_style(element, name, value);
                        }
                    }
                    StyleModifier::EndMarker(_) => (),
                }
            }
            debug_assert!(scratch.styles.updated.is_empty());
        }
    }
}

impl WithStyle for Styles {
    fn set_style(&mut self, name: CowStr, value: Option<CowStr>) {
        let new_modifier = if let Some(value) = value {
            StyleModifier::Set(name.clone(), value)
        } else {
            StyleModifier::Remove(name.clone())
        };

        let mut scratch = self.scratch.borrow_mut();
        if scratch.was_initialized() {
            self.modifiers.push(new_modifier);
        } else if let Some(modifier) = self.modifiers.get_mut(self.idx as usize) {
            if modifier != &new_modifier {
                if let StyleModifier::Remove(previous_name) | StyleModifier::Set(previous_name, _) =
                    modifier
                {
                    if &name != previous_name {
                        scratch.styles.updated.insert(previous_name.clone(), ());
                    }
                }
                scratch.styles.updated.insert(name, ());
                *modifier = new_modifier;
            }
            // else remove it out of updated_styles? (because previous styles are overwritten) not sure if worth it because potentially worse perf
        } else {
            scratch.styles.updated.insert(name, ());
            self.modifiers.push(new_modifier);
        }
        self.idx += 1;
    }

    fn rebuild_style_modifier(&mut self) {
        if self.idx == 0 {
            self.start_idx = 0;
        } else {
            let StyleModifier::EndMarker(start_idx) = self.modifiers[(self.idx - 1) as usize]
            else {
                unreachable!("this should not happen, as either `rebuild_style_modifier` happens first, or follows an `mark_end_of_style_modifier`")
            };
            self.idx = start_idx;
            self.start_idx = start_idx;
        }
    }

    fn mark_end_of_style_modifier(&mut self) {
        match self.modifiers.get_mut(self.idx as usize) {
            Some(StyleModifier::EndMarker(prev_start_idx)) if *prev_start_idx == self.start_idx => {
            } // class modifier hasn't changed
            Some(modifier) => {
                *modifier = StyleModifier::EndMarker(self.start_idx);
            }
            None => {
                self.modifiers
                    .push(StyleModifier::EndMarker(self.start_idx));
            }
        }
        self.idx += 1;
        self.start_idx = self.idx;
    }
}

impl WithStyle for ElementProps {
    fn rebuild_style_modifier(&mut self) {
        self.styles().rebuild_style_modifier();
    }

    fn mark_end_of_style_modifier(&mut self) {
        self.styles().mark_end_of_style_modifier();
    }

    fn set_style(&mut self, name: CowStr, value: Option<CowStr>) {
        self.styles().set_style(name, value);
    }
}

impl<E: DomNode<P>, P: WithStyle> WithStyle for Pod<E, P> {
    fn rebuild_style_modifier(&mut self) {
        self.props.rebuild_style_modifier();
    }

    fn mark_end_of_style_modifier(&mut self) {
        self.props.mark_end_of_style_modifier();
    }

    fn set_style(&mut self, name: CowStr, value: Option<CowStr>) {
        self.props.set_style(name, value);
    }
}

impl<E: DomNode<P>, P: WithStyle> WithStyle for PodMut<'_, E, P> {
    fn rebuild_style_modifier(&mut self) {
        self.props.rebuild_style_modifier();
    }

    fn mark_end_of_style_modifier(&mut self) {
        self.props.mark_end_of_style_modifier();
    }

    fn set_style(&mut self, name: CowStr, value: Option<CowStr>) {
        self.props.set_style(name, value);
    }
}

/// Syntax sugar for adding a type bound on the `ViewElement` of a view, such that both, [`ViewElement`] and [`ViewElement::Mut`] are bound to [`WithStyle`]
pub trait ElementWithStyle: for<'a> ViewElement<Mut<'a>: WithStyle> + WithStyle {}

impl<T> ElementWithStyle for T
where
    T: ViewElement + WithStyle,
    for<'a> T::Mut<'a>: WithStyle,
{
}

#[derive(Clone, Debug)]
/// A view to add `style` properties of `HTMLElement` and `SVGElement` derived elements,
pub struct Style<E, T, A> {
    el: E,
    styles: Vec<(CowStr, CowStr)>,
    phantom: PhantomData<fn() -> (T, A)>,
}

impl<E, T, A> Style<E, T, A> {
    pub fn new(el: E, styles: Vec<(CowStr, CowStr)>) -> Self {
        Style {
            el,
            styles,
            phantom: PhantomData,
        }
    }
}

impl<E, T, A> ViewMarker for Style<E, T, A> {}
impl<T, A, E> View<T, A, ViewCtx, DynMessage> for Style<E, T, A>
where
    T: 'static,
    A: 'static,
    E: View<T, A, ViewCtx, DynMessage, Element: ElementWithStyle>,
{
    type Element = E::Element;

    type ViewState = E::ViewState;

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        ctx.add_modifier_size_hint::<Styles>(self.styles.len());
        let (mut element, state) = self.el.build(ctx);
        for (key, value) in &self.styles {
            element.set_style(key.clone(), Some(value.clone()));
        }
        element.mark_end_of_style_modifier();
        (element, state)
    }

    fn rebuild<'e>(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'e, Self::Element>,
    ) -> Mut<'e, Self::Element> {
        element.rebuild_style_modifier();
        let mut element = self.el.rebuild(&prev.el, view_state, ctx, element);
        for (key, value) in &self.styles {
            element.set_style(key.clone(), Some(value.clone()));
        }
        element.mark_end_of_style_modifier();
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
