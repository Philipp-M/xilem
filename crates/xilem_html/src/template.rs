use std::any::TypeId;

use wasm_bindgen::UnwrapThrowExt;
use xilem_core::{Id, MessageResult, VecSplice};

use crate::{view::DomNode, ChangeFlags, Cx, HydrateSequence, Pod, ViewSequence};

pub struct Templated<E>(E);

impl<T, A, E: HydrateSequence<T, A> + 'static> ViewSequence<T, A> for Templated<E> {
    type State = E::State;

    fn build(&self, cx: &mut Cx, elements: &mut Vec<Pod>) -> Self::State {
        let type_id = TypeId::of::<Self>();
        if let Some(template) = cx.templates.get(&type_id) {
            let template = template.clone_node_with_deep(true).unwrap();
            self.0.hydrate(cx, elements, &template.child_nodes(), 0)
        } else {
            let state = self.0.build(cx, elements);

            let fragment = cx.document().create_document_fragment();
            for el in elements {
                let el = el.0.as_node_ref().clone_node_with_deep(true).unwrap();
                fragment.append_child(&el).unwrap_throw();
            }
            cx.templates.insert(type_id, fragment);
            state
        }
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        state: &mut Self::State,
        elements: &mut VecSplice<Pod>,
    ) -> ChangeFlags {
        self.0.rebuild(cx, &prev.0, state, elements)
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

    fn count(&self, state: &Self::State) -> usize {
        self.0.count(state)
    }
}

pub fn t<E>(view: E) -> Templated<E> {
    Templated(view)
}
