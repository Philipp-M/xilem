use super::create_dom_attribute_view;
use crate::{ChangeFlags, interfaces::for_all_dom_interfaces};
use std::borrow::Cow;

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum ElementAttr {
    Class(Cow<'static, str>),
}

// TODO currently something like el.class("class1").class("class2") will result in "class2" (i.e. overwrite previous uses of class()) which is maybe not what we want.
// There should probably be a way to add/remove classes when composing the element.
create_dom_attribute_view!(class, Cow<'static, str>, Element);

macro_rules! impl_dom_interface_for_element_dom_attributes {
    ($dom_interface:ident) => {
        impl<T, A, E: $crate::interfaces::$dom_interface<T, A>>
            $crate::interfaces::$dom_interface<T, A> for ElementClass<E>
        {
        }
    };
}

// necessary to be different?
for_all_dom_interfaces!(impl_dom_interface_for_element_dom_attributes);

pub(crate) fn build_dom_attribute(el: &web_sys::Element, attr: &ElementAttr) {
    match attr {
        ElementAttr::Class(class) => {
            // benches show, that className is the fastest way to set the class: (https://www.measurethat.net/Benchmarks/Show/5918/0/classname-vs-setattribute-vs-classlist)
            el.set_class_name(class);
        }
    }
}

pub(crate) fn rebuild_dom_attribute(
    el: &web_sys::Element,
    old: &ElementAttr,
    new: &ElementAttr,
) -> ChangeFlags {
    match (old, new) {
        (ElementAttr::Class(old_class), ElementAttr::Class(new_class))
            if old_class != new_class =>
        {
            el.set_class_name(new_class);
            ChangeFlags::OTHER_CHANGE
        }
        _ => ChangeFlags::empty(),
    }
}
