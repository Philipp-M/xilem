use std::borrow::Cow;

use xilem_core::{Id, MessageResult};

use crate::{
    interfaces::{EventTarget, HtmlMediaElement, Node, HtmlVideoElement, HtmlElement},
    AttributeValue, ChangeFlags, Cx, View, ViewMarker,
};

use super::interfaces::Element;

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlMediaElementAttr {
    Play(bool),
}

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlVideoElementAttr {
    Width(u32),
    Height(u32),
}

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum DomAttr {
    HtmlMediaElement(HtmlMediaElementAttr),
    HtmlVideoElement(HtmlVideoElementAttr),
}

// TODO different less verbose name?
pub struct HtmlMediaElementPlay<E> {
    pub(crate) element: E,
    pub(crate) value: bool,
}

impl<E> HtmlMediaElementPlay<E> {
    pub fn new(element: E, value: bool) -> Self {
        Self { element, value }
    }
}

impl<E> ViewMarker for HtmlMediaElementPlay<E> {}

impl<T, A, E: HtmlMediaElement<T, A>> EventTarget<T, A> for HtmlMediaElementPlay<E> {}
impl<T, A, E: HtmlMediaElement<T, A>> Node<T, A> for HtmlMediaElementPlay<E> {
    fn node_name(&self) -> &str {
        self.element.node_name()
    }
}
// impl<T, A, E: HtmlMediaElement<T, A>> Element<T, A> for E {}
impl<T, A, E: HtmlMediaElement<T, A>> Element<T, A> for HtmlMediaElementPlay<E> {}
impl<T, A, E: HtmlMediaElement<T, A>> HtmlElement<T, A> for HtmlMediaElementPlay<E> {}
impl<T, A, E: HtmlMediaElement<T, A>> HtmlMediaElement<T, A> for HtmlMediaElementPlay<E> {}
impl<T, A, E: HtmlMediaElement<T, A>> HtmlVideoElement<T, A> for HtmlMediaElementPlay<E> {}
impl<T, A, E: HtmlMediaElement<T, A>> View<T, A> for HtmlMediaElementPlay<E> {
    type State = E::State;
    type Element = E::Element;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        cx.add_new_dom_attribute_to_current_element(
            |a| matches!(a, DomAttr::HtmlMediaElement(HtmlMediaElementAttr::Play(_))),
            &DomAttr::HtmlMediaElement(HtmlMediaElementAttr::Play(self.value)),
        );
        self.element.build(cx)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        cx.add_new_dom_attribute_to_current_element(
            |a| matches!(a, DomAttr::HtmlMediaElement(HtmlMediaElementAttr::Play(_))),
            &DomAttr::HtmlMediaElement(HtmlMediaElementAttr::Play(self.value)),
        );
        self.element.rebuild(cx, &prev.element, id, state, element)
    }

    fn message(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> MessageResult<A> {
        self.element.message(id_path, state, message, app_state)
    }
}

pub struct HtmlVideoElementWidth<E> {
    pub(crate) element: E,
    pub(crate) value: u32,
}

impl<E> HtmlVideoElementWidth<E> {
    pub fn new(element: E, value: u32) -> Self {
        Self { element, value }
    }
}

impl<E> ViewMarker for HtmlVideoElementWidth<E> {}

impl<T, A, E: HtmlVideoElement<T, A>> EventTarget<T, A> for HtmlVideoElementWidth<E> {}
impl<T, A, E: HtmlVideoElement<T, A>> Node<T, A> for HtmlVideoElementWidth<E> {
    fn node_name(&self) -> &str {
        self.element.node_name()
    }
}
impl<T, A, E: HtmlVideoElement<T, A>> Element<T, A> for HtmlVideoElementWidth<E> {}
impl<T, A, E: HtmlVideoElement<T, A>> HtmlElement<T, A> for HtmlVideoElementWidth<E> {}
impl<T, A, E: HtmlVideoElement<T, A>> HtmlMediaElement<T, A> for HtmlVideoElementWidth<E> {}
impl<T, A, E: HtmlVideoElement<T, A>> HtmlVideoElement<T, A> for HtmlVideoElementWidth<E> {}
impl<T, A, E: Element<T, A>> View<T, A> for HtmlVideoElementWidth<E> {
    type State = E::State;
    type Element = E::Element;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        cx.add_new_attribute_to_current_element(
            &Cow::from("width"),
            &Some(AttributeValue::U32(self.value)),
        );
        cx.add_new_dom_attribute_to_current_element(
            |a| matches!(a, DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Width(_))),
            &DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Width(self.value)),
        );
        self.element.build(cx)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        cx.add_new_attribute_to_current_element(
            &Cow::from("width"),
            &Some(AttributeValue::U32(self.value)),
        );
        cx.add_new_dom_attribute_to_current_element(
            |a| matches!(a, DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Width(_))),
            &DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Width(self.value)),
        );
        self.element.rebuild(cx, &prev.element, id, state, element)
    }

    fn message(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> MessageResult<A> {
        self.element.message(id_path, state, message, app_state)
    }
}

pub struct Attr<E> {
    pub(crate) element: E,
    pub(crate) name: Cow<'static, str>,
    pub(crate) value: Option<AttributeValue>,
}

impl<E> ViewMarker for Attr<E> {}

impl<T, A, E: Element<T, A>> View<T, A> for Attr<E> {
    type State = E::State;
    type Element = E::Element;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        cx.add_new_attribute_to_current_element(&self.name, &self.value);
        self.element.build(cx)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        cx.add_new_attribute_to_current_element(&self.name, &self.value);
        self.element.rebuild(cx, &prev.element, id, state, element)
    }

    fn message(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> MessageResult<A> {
        self.element.message(id_path, state, message, app_state)
    }
}
