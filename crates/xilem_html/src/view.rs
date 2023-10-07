// Copyright 2023 the Druid Authors.
// SPDX-License-Identifier: Apache-2.0

//! Integration with xilem_core. This instantiates the View and related
//! traits for DOM node generation.

use std::{any::Any, borrow::Cow, ops::Deref, rc::Rc};

use wasm_bindgen::{
    convert::{FromWasmAbi, IntoWasmAbi},
    UnwrapThrowExt,
};
use xilem_core::{Id, MessageResult};

use crate::{context::Cx, ChangeFlags};

mod sealed {
    pub trait Sealed {}
}

// A possible refinement of xilem_core is to allow a single concrete type
// for a view element, rather than an associated type with a bound.
/// This trait is implemented for types that implement `AsRef<web_sys::Node>`.
/// It is an implementation detail.
pub trait DomNode: sealed::Sealed {
    fn into_pod(self, id: Id) -> Pod;
    fn as_node_ref(&self) -> &web_sys::Node;
}

impl<N: AsRef<web_sys::Node> + 'static> sealed::Sealed for N {}
impl<N: AsRef<web_sys::Node> + 'static> DomNode for N {
    fn into_pod(self, id: Id) -> Pod {
        Pod(Rc::new(self), id)
    }

    fn as_node_ref(&self) -> &web_sys::Node {
        self.as_ref()
    }
}

/// This trait is implemented for types that implement `AsRef<web_sys::Element>`.
/// It is an implementation detail.
pub trait DomElement: DomNode {
    fn as_element_ref(&self) -> &web_sys::Element;
}

impl<N: DomNode + AsRef<web_sys::Element>> DomElement for N {
    fn as_element_ref(&self) -> &web_sys::Element {
        self.as_ref()
    }
}

/// A trait for types that can be type-erased and impl `AsRef<Node>`. It is an
/// implementation detail.
pub trait AnyNode: sealed::Sealed {
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_node_ref(&self) -> &web_sys::Node;
}

impl<N: AsRef<web_sys::Node> + Any> AnyNode for N {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_node_ref(&self) -> &web_sys::Node {
        self.as_ref()
    }
}

impl sealed::Sealed for Box<dyn AnyNode> {}
impl DomNode for Box<dyn AnyNode> {
    fn into_pod(self, id: Id) -> Pod {
        Pod(self.into(), id)
    }

    fn as_node_ref(&self) -> &web_sys::Node {
        self.deref().as_node_ref()
    }
}

/// A container that holds a DOM element.
///
/// This implementation may be overkill (it's possibly enough that everything is
/// just a `web_sys::Element`), but does allow element types that contain other
/// data, if needed.
#[derive(Clone)]
pub struct Pod(pub Rc<dyn AnyNode>, Id);

impl Pod {
    fn new(node: impl DomNode, id: Id) -> Self {
        node.into_pod(id)
    }

    fn set_id(&mut self, id: Id) {
        self.1 = id;
    }

    pub fn id(&self) -> Id {
        self.1
    }

    fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        unsafe {
            Rc::get_mut_unchecked(&mut self.0)
                .as_any_mut()
                .downcast_mut()
        }
    }

    fn mark(&mut self, flags: ChangeFlags) -> ChangeFlags {
        flags
    }
}

// struct Pods<'a>(&'a [Pod]);

// impl<'a> Deref for Pods<'a> {
//     type Target = [Pod];

//     fn deref(&self) -> &Self::Target {
//         self.0
//     }
// }

// struct PodIdIter<'a>(usize, &'a [Pod]);

// struct PodId(usize, Id);

pub(crate) struct NodeIds(pub(crate) Vec<Pod>);

impl<'a> imara_diff::intern::TokenSource for &'a NodeIds {
    type Token = Id;

    type Tokenizer = std::iter::Map<std::slice::Iter<'a, Pod>, fn(&'a Pod) -> Id>;

    fn tokenize(&self) -> Self::Tokenizer {
        self.0.iter().map(|n| n.1)
    }

    fn estimate_tokens(&self) -> u32 {
        self.0.len() as u32
    }
}

pub(crate) struct UpdateElement<'a> {
    pub(crate) parent: web_sys::Element,
    pub(crate) before: &'a NodeIds,
    pub(crate) after: &'a NodeIds,
}

impl<'a> imara_diff::Sink for UpdateElement<'a> {
    type Out = ();

    fn process_change(&mut self, before: std::ops::Range<u32>, after: std::ops::Range<u32>) {
        for n in &self.before.0[(before.start as usize)..(before.end as usize)] {
            // let n = &unsafe { web_sys::Node::from_abi(*idx) };
            // web_sys::console::log_1(&"removing".into());
            // web_sys::console::log_1(&format!("removing: {}, index in arr: {}, end: {}", idx, before.start, before.end).into());
            // web_sys::console::log_1(n);
            self.parent.remove_child(n.0.as_node_ref()).unwrap_throw();
        }
        for n in &self.after.0[(after.start as usize)..(after.end as usize)] {
            // let n = &unsafe { web_sys::Node::from_abi(*idx) };
            // web_sys::console::log_1(&format!("adding: {}, index in arr: {}, end: {}", idx, after.start, after.end).into());
            // web_sys::console::log_1(&"adding".into());
            // web_sys::console::log_1(n);
            self.parent.append_child(n.0.as_node_ref()).unwrap_throw();
            // self.parent
            //     .append_child(&unsafe { web_sys::Node::from_abi(*idx) })
            //     .unwrap_throw();
        }
    }

    fn finish(self) -> Self::Out {}
}

// // Well this is kinda hacky (workaround to get imara-diff working)...
// impl Eq for PodId {}
// impl PartialEq for PodId {
//     fn eq(&self, other: &Self) -> bool {
//         self.1 == other.1
//     }
// }

// impl std::hash::Hash for PodId {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.1.hash(state)
//     }
// }

// impl<'a> PodIdIter<'a> {
//     pub fn new(pods: &'a [Pod]) -> Self {
//         Self(0, pods)
//     }
// }

// impl<'a> Iterator for PodIdIter<'a> {
//     type Item = PodId;

//     fn next(&mut self) -> Option<Self::Item> {
//         let item = self.1.get(self.0).map(|p| PodId(self.0, p.1));
//         self.0 += 1;
//         item
//     }
// }

// impl<'a> imara_diff::intern::TokenSource for Pods<'a> {
//     type Token = PodId;

//     type Tokenizer = PodIdIter<'a>;

//     fn tokenize(&self) -> Self::Tokenizer {
//         // PodIdIter::new(self.0)
//     }

//     fn estimate_tokens(&self) -> u32 {
//         self.0.len() as u32
//     }
// }

// impl<'a> imara_diff::intern::TokenSource for PodIds {
//     type Token = &'a PodId;

//     type Tokenizer = std::slice::Iter<'a, PodId>;

//     fn tokenize(&self) -> Self::Tokenizer {
//         self.0.iter()
//         // PodIdIter::new(self.0)
//     }

//     fn estimate_tokens(&self) -> u32 {
//         self.0.len() as u32
//     }
// }

// impl<'a> imara_diff::intern::TokenSource for &'a[ {
//     type Token = u32;

//     type Tokenizer = std::slice::Iter<'a, PodId>;

//     fn tokenize(&self) -> Self::Tokenizer {
//         self.0.iter()
//         // PodIdIter::new(self.0)
//     }

//     fn estimate_tokens(&self) -> u32 {
//         self.0.len() as u32
//     }
// }

xilem_core::generate_view_trait! {View, DomNode, Cx, ChangeFlags;}
xilem_core::generate_viewsequence_trait! {ViewSequence, View, ViewMarker, DomNode, Cx, ChangeFlags, Pod;}
xilem_core::generate_anyview_trait! {AnyView, View, ViewMarker, Cx, ChangeFlags, AnyNode, BoxedView;}
xilem_core::generate_memoize_view! {Memoize, MemoizeState, View, ViewMarker, Cx, ChangeFlags, s, memoize;}
xilem_core::generate_adapt_view! {View, Cx, ChangeFlags;}
xilem_core::generate_adapt_state_view! {View, Cx, ChangeFlags;}

// strings -> text nodes

impl ViewMarker for &'static str {}
impl<T, A> View<T, A> for &'static str {
    type State = ();
    type Element = web_sys::Text;

    fn build(&self, _cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let el = new_text(self);
        let id = Id::next();
        (id, (), el)
    }

    fn rebuild(
        &self,
        _cx: &mut Cx,
        prev: &Self,
        _id: &mut Id,
        _state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut is_changed = ChangeFlags::empty();
        if prev != self {
            element.set_data(self);
            is_changed |= ChangeFlags::OTHER_CHANGE;
        }
        is_changed
    }

    fn message(
        &self,
        _id_path: &[Id],
        _state: &mut Self::State,
        _message: Box<dyn std::any::Any>,
        _app_state: &mut T,
    ) -> MessageResult<A> {
        MessageResult::Nop
    }
}

impl ViewMarker for String {}
impl<T, A> View<T, A> for String {
    type State = ();
    type Element = web_sys::Text;

    fn build(&self, _cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let el = new_text(self);
        let id = Id::next();
        (id, (), el)
    }

    fn rebuild(
        &self,
        _cx: &mut Cx,
        prev: &Self,
        _id: &mut Id,
        _state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut is_changed = ChangeFlags::empty();
        if prev != self {
            element.set_data(self);
            is_changed |= ChangeFlags::OTHER_CHANGE;
        }
        is_changed
    }

    fn message(
        &self,
        _id_path: &[Id],
        _state: &mut Self::State,
        _message: Box<dyn std::any::Any>,
        _app_state: &mut T,
    ) -> MessageResult<A> {
        MessageResult::Nop
    }
}

impl ViewMarker for Cow<'static, str> {}
impl<T, A> View<T, A> for Cow<'static, str> {
    type State = ();
    type Element = web_sys::Text;

    fn build(&self, _cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let el = new_text(self);
        let id = Id::next();
        (id, (), el)
    }

    fn rebuild(
        &self,
        _cx: &mut Cx,
        prev: &Self,
        _id: &mut Id,
        _state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        let mut is_changed = ChangeFlags::empty();
        if prev != self {
            element.set_data(self);
            is_changed |= ChangeFlags::OTHER_CHANGE;
        }
        is_changed
    }

    fn message(
        &self,
        _id_path: &[Id],
        _state: &mut Self::State,
        _message: Box<dyn std::any::Any>,
        _app_state: &mut T,
    ) -> MessageResult<A> {
        MessageResult::Nop
    }
}

fn new_text(text: &str) -> web_sys::Text {
    web_sys::Text::new_with_data(text).unwrap()
}
