use wasm_bindgen::JsCast;

use crate::{attribute::Attr, event::EventListener, OptionalAction, HtmlMediaElementPlay};

use super::{Element, EventTarget};

pub trait Node<T, A>: EventTarget<T, A> {
    fn node_name(&self) -> &str;
}

impl<T, A, E: Element<T, A>> Node<T, A> for Attr<E> {
    fn node_name(&self) -> &str {
        self.element.node_name()
    }
}

impl<T, A, E: Element<T, A>> Node<T, A> for HtmlMediaElementPlay<E> {
    fn node_name(&self) -> &str {
        self.element.node_name()
    }
}

impl<T, A, E: Node<T, A>, Ev, F, OA> Node<T, A> for EventListener<E, Ev, F>
where
    F: Fn(&mut T, Ev) -> OA,
    E: Node<T, A>,
    Ev: JsCast + 'static,
    OA: OptionalAction<A>,
{
    fn node_name(&self) -> &str {
        self.element.node_name()
    }
}
