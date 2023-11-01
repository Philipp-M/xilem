#[cfg(feature = "HtmlCanvasElement")]
pub mod html_canvas_element;
#[cfg(feature = "HtmlMediaElement")]
pub mod html_media_element;
#[cfg(feature = "HtmlVideoElement")]
pub mod html_video_element;

pub mod element;

use crate::ChangeFlags;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum DomAttr {
    Element(element::ElementAttr),
    #[cfg(feature = "HtmlMediaElement")]
    HtmlMediaElement(html_media_element::HtmlMediaElementAttr),
    #[cfg(feature = "HtmlVideoElement")]
    HtmlVideoElement(html_video_element::HtmlVideoElementAttr),
    #[cfg(feature = "HtmlCanvasElement")]
    HtmlCanvasElement(html_canvas_element::HtmlCanvasElementAttr),
}

pub(crate) fn build_dom_attribute(el: &web_sys::Element, attr: &DomAttr) {
    match attr {
        DomAttr::Element(attr) => element::build_dom_attribute(el, attr),
        #[cfg(feature = "HtmlVideoElement")]
        DomAttr::HtmlMediaElement(attr) => {
            let el = el.dyn_ref::<web_sys::HtmlMediaElement>().unwrap_throw();
            html_media_element::build_dom_attribute(el, attr)
        }
        #[cfg(feature = "HtmlVideoElement")]
        DomAttr::HtmlVideoElement(attr) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            html_video_element::build_dom_attribute(el, attr)
        }
        #[cfg(feature = "HtmlCanvasElement")]
        DomAttr::HtmlCanvasElement(attr) => {
            let el = el.dyn_ref::<web_sys::HtmlCanvasElement>().unwrap_throw();
            html_canvas_element::build_dom_attribute(el, attr)
        }
    }
}

pub(crate) fn rebuild_dom_attribute(
    el: &web_sys::Element,
    old: &DomAttr,
    new: &DomAttr,
) -> ChangeFlags {
    match (old, new) {
        (DomAttr::Element(old), DomAttr::Element(new)) => {
            element::rebuild_dom_attribute(el, old, new)
        }
        (DomAttr::HtmlMediaElement(old), DomAttr::HtmlMediaElement(new)) => {
            let el = el.dyn_ref::<web_sys::HtmlMediaElement>().unwrap_throw();
            html_media_element::rebuild_dom_attribute(el, old, new)
        }
        (DomAttr::HtmlVideoElement(old), DomAttr::HtmlVideoElement(new)) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            html_video_element::rebuild_dom_attribute(el, old, new)
        }
        (DomAttr::HtmlCanvasElement(old), DomAttr::HtmlCanvasElement(new)) => {
            let el = el.dyn_ref::<web_sys::HtmlCanvasElement>().unwrap_throw();
            html_canvas_element::rebuild_dom_attribute(el, old, new)
        }
        _ => ChangeFlags::empty(),
    }
}

#[allow(unused)]
macro_rules! create_dom_attribute_view {
    // This match arm doesn't implement any dom interface for the view
    ($attribute:ident, $value_type:ty, $dom_interface:ident) => {
        paste::paste! {
        // TODO different less verbose name?
        pub struct [<$dom_interface $attribute:camel>]<E> {
            pub(crate) element: E,
            pub(crate) value: $value_type,
        }

        impl<E> [<$dom_interface $attribute:camel>]<E> {
            pub fn new(element: E, value: $value_type) -> Self {
                Self { element, value }
            }
        }

        impl<E> $crate::sealed::Sealed for [<$dom_interface $attribute:camel>]<E> {}
        impl<E> $crate::ViewMarker for [<$dom_interface $attribute:camel>]<E> {}

        impl<T, A, E> $crate::View<T, A> for [<$dom_interface $attribute:camel>]<E>
        where
            E: $crate::interfaces::$dom_interface<T, A>,
        {
            type State = E::State;
            type Element = E::Element;

            fn build(&self, cx: &mut $crate::Cx) -> (xilem_core::Id, Self::State, Self::Element) {
                use $crate::dom_attributes::{DomAttr, [<$dom_interface:snake>]::[<$dom_interface Attr>]};
                cx.add_new_dom_attribute_to_current_element(
                    |a| matches!(a, DomAttr::$dom_interface([<$dom_interface Attr>]::[<$attribute:camel>](_))),
                    &DomAttr::$dom_interface([<$dom_interface Attr>]::[<$attribute:camel>](self.value.clone())),
                );
                self.element.build(cx)
            }

            fn rebuild(
                &self,
                cx: &mut $crate::Cx,
                prev: &Self,
                id: &mut xilem_core::Id,
                state: &mut Self::State,
                element: &mut Self::Element,
            ) -> $crate::ChangeFlags {
                use $crate::dom_attributes::{DomAttr, [<$dom_interface:snake>]::[<$dom_interface Attr>]};
                cx.add_new_dom_attribute_to_current_element(
                    |a| matches!(a, DomAttr::$dom_interface([<$dom_interface Attr>]::[<$attribute:camel>](_))),
                    &DomAttr::$dom_interface([<$dom_interface Attr>]::[<$attribute:camel>](self.value.clone())),
                );
                self.element.rebuild(cx, &prev.element, id, state, element)
            }

            fn message(
                &self,
                id_path: &[xilem_core::Id],
                state: &mut Self::State,
                message: Box<dyn std::any::Any>,
                app_state: &mut T,
            ) -> xilem_core::MessageResult<A> {
                self.element.message(id_path, state, message, app_state)
            }
        }
        }
    };
    // This match arm implements all ancestor dom interfaces, and the explicit descendent interfaces.
    // Not having the descendant dom interface parameter, would make this macro vastly more complex (probably needs something like proc-macros)
    ($attribute:ident, $value_type:ty, $dom_interface:ident : {$($descendant_dom_interface:ident),*}) => {
        create_dom_attribute_view!($attribute, $value_type, $dom_interface);

        paste::paste! {
        macro_rules! [<generate_dom_interface_impl_for_ $dom_interface:snake _ $attribute:snake>] {
            ($inner_dom_interface:ident) => {
                impl<T, A, E> $crate::interfaces::$inner_dom_interface<T, A> for [<$dom_interface $attribute:camel>]<E>
                where
                    E: $crate::interfaces::$dom_interface<T, A>,
                {}
            };
        }

        $crate::interfaces::for_all_dom_interface_relatives!(
            $dom_interface,
            [<generate_dom_interface_impl_for_ $dom_interface:snake _ $attribute:snake>]
        );

        $(
        #[cfg(feature = "" $descendant_dom_interface "")]
        impl<T, A, E> $crate::interfaces::$descendant_dom_interface<T, A> for [<$dom_interface $attribute:camel>]<E>
        where
            E: $crate::interfaces::$descendant_dom_interface<T, A>,
        {}
        )*
        }
    };
}

#[allow(unused)]
pub(crate) use create_dom_attribute_view;
