use std::borrow::Cow;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use xilem_core::Id;

use crate::{view::DomNode, Cx, Pod, View, ViewMarker, ViewSequence};

pub trait HydrateSequence<T, A>: ViewSequence<T, A> {
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        node_list: &web_sys::NodeList,
        cur_index: u32,
    ) -> Self::State;
}

pub trait Hydrate<T, A>: View<T, A> {
    fn hydrate(&self, cx: &mut Cx, element: &web_sys::Node) -> (Id, Self::State, Self::Element);
}

impl<T, A, V: Hydrate<T, A> + ViewMarker> HydrateSequence<T, A> for V
where
    V::Element: 'static,
{
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        node_list: &web_sys::NodeList,
        cur_index: u32,
    ) -> Self::State {
        let n = node_list.get(cur_index).unwrap_throw();
        let (id, state, element) = <V as Hydrate<T, A>>::hydrate(self, cx, &n);
        elements.push(element.into_pod());
        (state, id)
    }
}

impl<T, A, VS: HydrateSequence<T, A>> HydrateSequence<T, A> for Vec<VS> {
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        node_list: &web_sys::NodeList,
        cur_index: u32,
    ) -> Self::State {
        let (_, state) = self.iter().fold(
            (cur_index, Vec::new()),
            |(mut cur_index, mut states), vs| {
                let state = vs.hydrate(cx, elements, node_list, cur_index);
                cur_index += vs.count(&state) as u32;
                states.push(state);
                (cur_index, states)
            },
        );
        state
    }
}

impl<T, A, VS: HydrateSequence<T, A>> HydrateSequence<T, A> for Option<VS> {
    fn hydrate(
        &self,
        cx: &mut Cx,
        elements: &mut Vec<Pod>,
        node_list: &web_sys::NodeList,
        cur_index: u32,
    ) -> Self::State {
        match self {
            None => None,
            Some(vt) => {
                let state = vt.hydrate(cx, elements, node_list, cur_index);
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
                node_list: &web_sys::NodeList,
                cur_index: u32,
            ) -> Self::State {
                $(
                let $t = self.$i.hydrate(cx, elements, node_list, cur_index);
                let cur_index = cur_index + self.$i.count(&$t) as u32;
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
    fn hydrate(&self, _cx: &mut Cx, element: &web_sys::Node) -> (Id, Self::State, Self::Element) {
        let el: web_sys::Text = element.clone().dyn_into().unwrap_throw();
        el.set_data(self);
        let id = Id::next();
        (id, (), el)
    }
}

impl<T, A> Hydrate<T, A> for String {
    fn hydrate(&self, _cx: &mut Cx, element: &web_sys::Node) -> (Id, Self::State, Self::Element) {
        let el: web_sys::Text = element.clone().dyn_into().unwrap_throw();
        el.set_data(self);
        let id = Id::next();
        (id, (), el)
    }
}

impl<T, A> Hydrate<T, A> for Cow<'static, str> {
    fn hydrate(&self, _cx: &mut Cx, element: &web_sys::Node) -> (Id, Self::State, Self::Element) {
        let el: web_sys::Text = element.clone().dyn_into().unwrap_throw();
        el.set_data(self);
        let id = Id::next();
        (id, (), el)
    }
}
