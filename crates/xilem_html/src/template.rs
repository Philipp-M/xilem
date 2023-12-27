use std::{any::TypeId, rc::Rc};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use xilem_core::{Id, MessageResult};

use crate::{view::DomNode, ChangeFlags, Cx, Hydrate, View, ViewMarker};

pub struct Templated<E>(Rc<E>);

impl<E> ViewMarker for Templated<E> {}

impl<T, A, E> View<T, A> for Templated<E>
where
    E: View<T, A> + Hydrate<T, A> + 'static,
    E::Element: JsCast,
{
    type State = E::State;
    type Element = E::Element;

    fn build(&self, cx: &mut Cx) -> (Id, E::State, E::Element) {
        let type_id = TypeId::of::<Self>();
        if let Some((element, view)) = cx.templates.get(&type_id) {
            let element = element.clone_node_with_deep(true).unwrap_throw();
            let prev = view.clone();
            let prev = prev.downcast_ref::<E>().unwrap_throw();
            let (mut id, mut state, mut element) = prev.hydrate(cx, element);
            self.0.rebuild(cx, prev, &mut id, &mut state, &mut element);

            (id, state, element)
        } else {
            let (id, state, element) = self.0.build(cx);

            let template: web_sys::Node = element
                .as_node_ref()
                .clone_node_with_deep(true)
                .unwrap_throw();

            cx.templates.insert(type_id, (template, self.0.clone()));
            (id, state, element)
        }
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        self.0.rebuild(cx, &prev.0, id, state, element)
    }

    fn message(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> MessageResult<A> {
        self.0.message(id_path, state, message, app_state)
    }
}

// TODO is it somehow possible to eliminate the Rc everytime this is called?
pub fn t<E>(view: E) -> Templated<E> {
    Templated(Rc::new(view))
}

