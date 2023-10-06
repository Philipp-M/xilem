use std::borrow::Cow;

use xilem_core::{Id, MessageResult};

use crate::{AttributeValue, ChangeFlags, Cx, View, ViewMarker};

use super::interfaces::Element;

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlMediaElementAttr {
    Play(bool),
}

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum DomAttr {
    HtmlMediaElement(HtmlMediaElementAttr),
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

impl<T, A, E: Element<T, A>> View<T, A> for HtmlMediaElementPlay<E> {
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
