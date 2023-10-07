use std::borrow::Cow;

use xilem_core::{Id, MessageResult};

use crate::{AttributeValue, ChangeFlags, Cx, Hydrate, View, ViewMarker};

use super::interfaces::Element;

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

impl<T, A, E: Element<T, A> + Hydrate<T, A>> Hydrate<T, A> for Attr<E> {
    fn hydrate(&self, cx: &mut Cx, node: web_sys::Node) -> (Id, Self::State, Self::Element) {
        cx.add_new_attribute_to_current_element(&self.name, &self.value);
        self.element.hydrate(cx, node)
    }
}
