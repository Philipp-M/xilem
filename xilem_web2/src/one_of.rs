use wasm_bindgen::UnwrapThrowExt;
use xilem_core::{Mut, Noop, NoopCtx, OneOf, OneOf2, OneOfCtx};

use crate::{
    attribute::WithAttributes, class::WithClasses, elements::html, interfaces::Element,
    AttributeValue, DomNode, DomView, Pod, PodMut, ViewCtx,
};

type CowStr = std::borrow::Cow<'static, str>;

impl<P1, P2, P3, P4, N1, N2, N3, N4> OneOfCtx<Pod<N1, P1>, Pod<N2, P2>, Pod<N3, P3>, Pod<N4, P4>>
    for ViewCtx
where
    P1: 'static,
    P2: 'static,
    P3: 'static,
    P4: 'static,
    N1: DomNode<P1>,
    N2: DomNode<P2>,
    N3: DomNode<P3>,
    N4: DomNode<P4>,
{
    type OneOfElement = Pod<OneOf<N1, N2, N3, N4>, OneOf<P1, P2, P3, P4>>;

    fn upcast_one_of_two_element(
        elem: OneOf<Pod<N1, P1>, Pod<N2, P2>, Pod<N3, P3>, Pod<N4, P4>>,
    ) -> Self::OneOfElement {
        match elem {
            OneOf::A(e) => Pod {
                node: OneOf::A(e.node),
                props: OneOf::A(e.props),
            },
            OneOf::B(e) => Pod {
                node: OneOf::B(e.node),
                props: OneOf::B(e.props),
            },
            OneOf::C(e) => Pod {
                node: OneOf::C(e.node),
                props: OneOf::C(e.props),
            },
            OneOf::D(e) => Pod {
                node: OneOf::D(e.node),
                props: OneOf::D(e.props),
            },
        }
    }

    fn update_one_of_two_element_mut(
        elem_mut: &mut Mut<'_, Self::OneOfElement>,
        new_elem: OneOf<Pod<N1, P1>, Pod<N2, P2>, Pod<N3, P3>, Pod<N4, P4>>,
    ) {
        let old_node: &web_sys::Node = elem_mut.node.as_ref();
        let new_node: &web_sys::Node = new_elem.as_ref();
        if old_node != new_node {
            elem_mut
                .parent
                .replace_child(new_node, old_node)
                .unwrap_throw();
        }
        (*elem_mut.node, *elem_mut.props) = match new_elem {
            OneOf::A(e) => (OneOf::A(e.node), OneOf::A(e.props)),
            OneOf::B(e) => (OneOf::B(e.node), OneOf::B(e.props)),
            OneOf::C(e) => (OneOf::C(e.node), OneOf::C(e.props)),
            OneOf::D(e) => (OneOf::D(e.node), OneOf::D(e.props)),
        };
    }

    fn with_downcast_a(
        elem: &mut Mut<'_, Self::OneOfElement>,
        f: impl FnOnce(Mut<'_, Pod<N1, P1>>),
    ) {
        let (OneOf::A(node), OneOf::A(props)) = (&mut elem.node, &mut elem.props) else {
            unreachable!()
        };
        f(PodMut::new(node, props, elem.parent, elem.was_removed));
    }

    fn with_downcast_b(
        elem: &mut Mut<'_, Self::OneOfElement>,
        f: impl FnOnce(Mut<'_, Pod<N2, P2>>),
    ) {
        let (OneOf::B(node), OneOf::B(props)) = (&mut elem.node, &mut elem.props) else {
            unreachable!()
        };
        f(PodMut::new(node, props, elem.parent, elem.was_removed));
    }

    fn with_downcast_c(
        elem: &mut Mut<'_, Self::OneOfElement>,
        f: impl FnOnce(Mut<'_, Pod<N3, P3>>),
    ) {
        let (OneOf::C(node), OneOf::C(props)) = (&mut elem.node, &mut elem.props) else {
            unreachable!()
        };
        f(PodMut::new(node, props, elem.parent, elem.was_removed));
    }

    fn with_downcast_d(
        elem: &mut Mut<'_, Self::OneOfElement>,
        f: impl FnOnce(Mut<'_, Pod<N4, P4>>),
    ) {
        let (OneOf::D(node), OneOf::D(props)) = (&mut elem.node, &mut elem.props) else {
            unreachable!()
        };
        f(PodMut::new(node, props, elem.parent, elem.was_removed));
    }
}

impl NoopCtx for ViewCtx {
    type NoopElement = Pod<Noop, Noop>;
}

impl WithAttributes for Noop {
    fn start_attribute_modifier(&mut self) {
        unreachable!()
    }

    fn end_attribute_modifier(&mut self) {
        unreachable!()
    }

    fn set_attribute(&mut self, _name: CowStr, _value: Option<AttributeValue>) {
        unreachable!()
    }
}

impl WithClasses for Noop {
    fn start_class_modifier(&mut self) {
        unreachable!()
    }

    fn add_class(&mut self, _class_name: CowStr) {
        unreachable!()
    }

    fn remove_class(&mut self, _class_name: CowStr) {
        unreachable!()
    }

    fn end_class_modifier(&mut self) {
        todo!()
    }
}

impl<P> DomNode<P> for Noop {
    fn apply_props(&self, _props: &mut P) {
        unreachable!()
    }
}

impl<E1: WithAttributes, E2: WithAttributes> WithAttributes for OneOf<E1, E2> {
    fn start_attribute_modifier(&mut self) {
        match self {
            OneOf::A(e) => e.start_attribute_modifier(),
            OneOf::B(e) => e.start_attribute_modifier(),
            OneOf::C(e) => e.start_attribute_modifier(),
            OneOf::D(e) => e.start_attribute_modifier(),
        }
    }

    fn end_attribute_modifier(&mut self) {
        match self {
            OneOf::A(e) => e.end_attribute_modifier(),
            OneOf::B(e) => e.end_attribute_modifier(),
            OneOf::C(e) => e.end_attribute_modifier(),
            OneOf::D(e) => e.end_attribute_modifier(),
        }
    }

    fn set_attribute(&mut self, name: CowStr, value: Option<AttributeValue>) {
        match self {
            OneOf::A(e) => e.set_attribute(name, value),
            OneOf::B(e) => e.set_attribute(name, value),
            OneOf::C(e) => e.set_attribute(name, value),
            OneOf::D(e) => e.set_attribute(name, value),
        }
    }
}

impl<E1: WithClasses, E2: WithClasses> WithClasses for OneOf<E1, E2> {
    fn start_class_modifier(&mut self) {
        match self {
            OneOf::A(e) => e.start_class_modifier(),
            OneOf::B(e) => e.start_class_modifier(),
            OneOf::C(e) => e.start_class_modifier(),
            OneOf::D(e) => e.start_class_modifier(),
        }
    }

    fn add_class(&mut self, class_name: CowStr) {
        match self {
            OneOf::A(e) => e.add_class(class_name),
            OneOf::B(e) => e.add_class(class_name),
            OneOf::C(e) => e.add_class(class_name),
            OneOf::D(e) => e.add_class(class_name),
        }
    }

    fn remove_class(&mut self, class_name: CowStr) {
        match self {
            OneOf::A(e) => e.remove_class(class_name),
            OneOf::B(e) => e.remove_class(class_name),
            OneOf::C(e) => e.remove_class(class_name),
            OneOf::D(e) => e.remove_class(class_name),
        }
    }

    fn end_class_modifier(&mut self) {
        match self {
            OneOf::A(e) => e.end_class_modifier(),
            OneOf::B(e) => e.end_class_modifier(),
            OneOf::C(e) => e.end_class_modifier(),
            OneOf::D(e) => e.end_class_modifier(),
        }
    }
}

impl<P1, P2, P3, P4, E1: DomNode<P1>, E2: DomNode<P2>, E3: DomNode<P3>, E4: DomNode<P4>>
    DomNode<OneOf<P1, P2, P3, P4>> for OneOf<E1, E2, E3, E4>
{
    fn apply_props(&self, props: &mut OneOf<P1, P2, P3, P4>) {
        match (self, props) {
            (OneOf::A(el), OneOf::A(props)) => el.apply_props(props),
            (OneOf::B(el), OneOf::B(props)) => el.apply_props(props),
            _ => unreachable!(),
        }
    }
}

fn one_of_view() -> impl DomView<()> {
    // This works
    OneOf2::<html::Div<(), ()>, Noop>::A(html::div(()))
    // This doesn't
    // OneOf2::A(html::div(()))
}

fn one_of_element() -> impl Element<()> {
    OneOf2::<html::Div<(), ()>, Noop>::A(html::div(()))
    // OneOf2::A(html::div(()))
}

// This one works, when we don't have an impl ViewSequence for OneOf2
fn one_of_ambiguous_seq() -> impl DomView<()> {
    html::div(if true {
        OneOf2::A(html::div(()))
    } else {
        OneOf2::B(html::span(()))
    })
}
