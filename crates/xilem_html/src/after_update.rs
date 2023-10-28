use xilem_core::{Id, MessageResult};

use crate::{sealed::Sealed, ChangeFlags, Cx, View, ViewMarker};

use super::interfaces::{for_all_dom_interfaces, Element};

pub struct AfterUpdate<E, F> {
    pub(crate) element: E,
    pub(crate) callback: F,
}

impl<E, F> AfterUpdate<E, F> {
    pub fn new(element: E, callback: F) -> AfterUpdate<E, F> {
        AfterUpdate { element, callback }
    }
}

pub struct AfterUpdateState<E, S> {
    element: E,
    child_state: S,
    child_id: Id,
}

impl<E, F> ViewMarker for AfterUpdate<E, F> {}
impl<E, F> Sealed for AfterUpdate<E, F> {}

impl<T, A, E, F> View<T, A> for AfterUpdate<E, F>
where
    E: Element<T, A>,
    E::Element: Clone + PartialEq,
    F: Fn(&mut T, &E::Element),
{
    type State = AfterUpdateState<Self::Element, E::State>;
    type Element = E::Element;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, (element, state)) = cx.with_new_id(|cx| {
            let (child_id, child_state, el) = self.element.build(cx);
            let state = AfterUpdateState {
                child_state,
                child_id,
                element: el.clone(),
            };
            let id_path = cx.id_path().clone();
            cx.after_update
                .insert(*id_path.last().unwrap(), (true, id_path));
            (el, state)
        });
        (id, state, element)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        cx.with_id(*id, |cx| {
            let changeflags = self.element.rebuild(
                cx,
                &prev.element,
                &mut state.child_id,
                &mut state.child_state,
                element,
            );
            if *element != state.element {
                state.element = element.clone();
            }
            // entry() not possible due to borrow-checker issues
            // TODO there are likely cleaner ways to achieve this...
            if !cx.after_update.contains_key(id) {
                cx.after_update.insert(*id, (true, cx.id_path().clone()));
            } else {
                cx.after_update.get_mut(id).unwrap().0 = true;
            }
            changeflags
        })
    }

    fn message(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn std::any::Any>,
        app_state: &mut T,
    ) -> MessageResult<A> {
        match id_path {
            [] => {
                (self.callback)(app_state, &state.element);
                MessageResult::Nop
            }
            [element_id, rest_path @ ..] if *element_id == state.child_id => {
                self.element
                    .message(rest_path, &mut state.child_state, message, app_state)
            }
            _ => MessageResult::Stale(message),
        }
    }
}

macro_rules! impl_dom_interface_for_attr {
    ($dom_interface:ident) => {
        impl<T, A, E, F> $crate::interfaces::$dom_interface<T, A> for AfterUpdate<E, F>
        where
            E: $crate::interfaces::$dom_interface<T, A>,
            E::Element: Clone + PartialEq,
            F: Fn(&mut T, &E::Element),
        {
        }
    };
}

for_all_dom_interfaces!(impl_dom_interface_for_attr);
