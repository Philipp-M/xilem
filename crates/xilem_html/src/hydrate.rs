use std::borrow::Cow;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use xilem_core::Id;

use crate::{view::DomNode, Cx, Pod, View, ViewMarker, ViewSequence};

pub trait HydrateSequence<T, A>: ViewSequence<T, A> {
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        cur_node: &mut Option<web_sys::Node>,
    ) -> Self::State;
}

pub trait Hydrate<T, A>: View<T, A> {
    fn hydrate(&self, cx: &mut Cx, node: web_sys::Node) -> (Id, Self::State, Self::Element);
}

impl<T, A, V: Hydrate<T, A> + ViewMarker> HydrateSequence<T, A> for V
where
    V::Element: 'static,
{
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        cur_node: &mut Option<web_sys::Node>,
    ) -> Self::State {
        let n = cur_node.take().unwrap_throw();
        *cur_node = n.next_sibling();
        let (id, state, element) = <V as Hydrate<T, A>>::hydrate(self, cx, n);
        elements.push(element.into_pod());
        (state, id)
    }
}

impl<T, A, VS: HydrateSequence<T, A>> HydrateSequence<T, A> for Vec<VS> {
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        cur_node: &mut Option<web_sys::Node>,
    ) -> Self::State {
        let mut states = Vec::new();
        for vs in self.iter() {
            let mut n = cur_node.take();
            *cur_node = n.as_ref().unwrap_throw().next_sibling();
            let state = vs.hydrate(cx, elements, &mut n);
            states.push(state);
        }
        states
    }
}

impl<T, A, VS: HydrateSequence<T, A>> HydrateSequence<T, A> for Option<VS> {
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        cur_node: &mut Option<web_sys::Node>,
    ) -> Self::State {
        match self {
            None => None,
            Some(vs) => {
                let mut n = cur_node.take();
                *cur_node = n.as_ref().unwrap_throw().next_sibling();
                let state = vs.hydrate(cx, elements, &mut n);
                Some(state)
            }
        }
    }
}

macro_rules! impl_hydrate_tuple {
    ($( $t:ident; $i:tt),*) => {
        impl<T, A, $( $t: HydrateSequence<T, A> ),* > HydrateSequence<T, A> for ( $( $t, )* ) {
            #[allow(unused)]
            #[allow(non_snake_case)]
            #[allow(clippy::all)]
            fn hydrate(
                &self,
                cx: &mut Cx,
                elements: &mut Vec<Pod>,
                cur_node: &mut Option<web_sys::Node>,
            ) -> Self::State {
                $(
                let mut n = cur_node.take();
                *cur_node = n.as_ref().unwrap_throw().next_sibling();
                let $t = self.$i.hydrate(cx, elements, &mut n);
                )*
                ($($t,)*)
            }
        }
    }
}

impl_hydrate_tuple!();
impl_hydrate_tuple!(V0;0);
impl_hydrate_tuple!(V0;0, V1;1);
impl_hydrate_tuple!(V0;0, V1;1, V2;2);
impl_hydrate_tuple!(V0;0, V1;1, V2;2, V3;3);
impl_hydrate_tuple!(V0;0, V1;1, V2;2, V3;3, V4;4);
impl_hydrate_tuple!(V0;0, V1;1, V2;2, V3;3, V4;4, V5;5);
impl_hydrate_tuple!(V0;0, V1;1, V2;2, V3;3, V4;4, V5;5, V6;6);
impl_hydrate_tuple!(V0;0, V1;1, V2;2, V3;3, V4;4, V5;5, V6;6, V7;7);
impl_hydrate_tuple!(V0;0, V1;1, V2;2, V3;3, V4;4, V5;5, V6;6, V7;7, V8;8);
impl_hydrate_tuple!(V0;0, V1;1, V2;2, V3;3, V4;4, V5;5, V6;6, V7;7, V8;8, V9;9);

impl<T, A> Hydrate<T, A> for &'static str {
    fn hydrate(&self, _cx: &mut Cx, element: web_sys::Node) -> (Id, Self::State, Self::Element) {
        let el: web_sys::Text = element.dyn_into().unwrap_throw();
        el.set_data(self);
        let id = Id::next();
        (id, (), el)
    }
}

impl<T, A> Hydrate<T, A> for String {
    fn hydrate(&self, _cx: &mut Cx, element: web_sys::Node) -> (Id, Self::State, Self::Element) {
        let el: web_sys::Text = element.dyn_into().unwrap_throw();
        el.set_data(self);
        let id = Id::next();
        (id, (), el)
    }
}

impl<T, A> Hydrate<T, A> for Cow<'static, str> {
    fn hydrate(&self, _cx: &mut Cx, element: web_sys::Node) -> (Id, Self::State, Self::Element) {
        let el: web_sys::Text = element.dyn_into().unwrap_throw();
        el.set_data(self);
        let id = Id::next();
        (id, (), el)
    }
}
