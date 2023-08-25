use std::borrow::Cow;

use gloo::events::EventListenerOptions;
use wasm_bindgen::JsCast;

use crate::{
    dom::{attribute::Attr, event::EventListener},
    OptionalAction,
};

pub trait EventTarget<T, A> {
    fn on<E, EH, OA>(
        self,
        event: impl Into<Cow<'static, str>>,
        handler: EH,
    ) -> EventListener<Self, E, EH>
    where
        E: JsCast + 'static,
        OA: OptionalAction<A>,
        EH: Fn(&mut T, E) -> OA,
        Self: Sized,
    {
        EventListener::new(self, event, handler)
    }

    fn on_with_options<E, EH, OA>(
        self,
        event: impl Into<Cow<'static, str>>,
        handler: EH,
        options: EventListenerOptions,
    ) -> EventListener<Self, E, EH>
    where
        E: JsCast + 'static,
        OA: OptionalAction<A>,
        EH: Fn(&mut T, E) -> OA,
        Self: Sized,
    {
        EventListener::new_with_options(self, event, handler, options)
    }
}

impl<T, A, E: EventTarget<T, A>> EventTarget<T, A> for Attr<E> {}
impl<T, A, E: EventTarget<T, A>, Ev, F> EventTarget<T, A> for EventListener<E, Ev, F> {}