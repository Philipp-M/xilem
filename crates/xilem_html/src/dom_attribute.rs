#[cfg(feature = "HtmlMediaElement")]
#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlMediaElementAttr {
    Play(bool),
    PlaybackRate(f64),
}

#[cfg(feature = "HtmlVideoElement")]
#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlVideoElementAttr {
    Width(u32),
    Height(u32),
}

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum DomAttr {
    #[cfg(feature = "HtmlMediaElement")]
    HtmlMediaElement(HtmlMediaElementAttr),
    #[cfg(feature = "HtmlVideoElement")]
    HtmlVideoElement(HtmlVideoElementAttr),
}

// not having the descendant dom interface parameter, would make this macro vastly more complex (needs probably proc-macros)
#[allow(unused)]
macro_rules! create_dom_attribute_view {
    ($attribute:ident, $value_type:ty, $dom_interface:ident) => {
        create_dom_attribute_view!($attribute, $value_type, $dom_interface: {});
    };
    ($attribute:ident, $value_type:ty, $dom_interface:ident : {$($descendant_dom_interface:ident),*}) => {
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

        macro_rules! [<generate_dom_interface_impl_for_ $dom_interface:snake _ $attribute:snake>] {
            ($inner_dom_interface:ident) => {
                impl<T, A, E: $crate::interfaces::$dom_interface<T, A>> $crate::interfaces::$inner_dom_interface<T, A> for
                    [<$dom_interface $attribute:camel>]<E> {}
            };
        }

        $crate::interfaces::for_all_dom_interface_relatives!($dom_interface, [<generate_dom_interface_impl_for_ $dom_interface:snake _ $attribute:snake>]);

        $(
        paste::paste! {
        #[cfg(feature = "" $descendant_dom_interface "")]
        impl<T, A, E: $crate::interfaces::$descendant_dom_interface<T, A>> $crate::interfaces::$descendant_dom_interface<T, A> for
            [<$dom_interface $attribute:camel>]<E> {}
        }
        )*

        impl<E> $crate::sealed::Sealed for [<$dom_interface $attribute:camel>]<E> {}
        impl<E> $crate::ViewMarker for [<$dom_interface $attribute:camel>]<E> {}

        impl<T, A, E: $crate::interfaces::$dom_interface<T, A>> $crate::View<T, A> for [<$dom_interface $attribute:camel>]<E> {
            type State = E::State;
            type Element = E::Element;

            fn build(&self, cx: &mut $crate::Cx) -> (xilem_core::Id, Self::State, Self::Element) {
                // TODO only relevant in SSR contexts
                // cx.add_new_attribute_to_current_element(
                //     &Cow::from("width"),
                //     &Some(AttributeValue::U32(self.value)),
                // );
                cx.add_new_dom_attribute_to_current_element(
                    |a| matches!(a, $crate::dom_attribute::DomAttr::$dom_interface($crate::dom_attribute::[<$dom_interface Attr>]::[<$attribute:camel>](_))),
                    &$crate::dom_attribute::DomAttr::$dom_interface($crate::dom_attribute::[<$dom_interface Attr>]::[<$attribute:camel>](self.value)),
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
                // TODO only relevant in SSR contexts
                // cx.add_new_attribute_to_current_element(
                //     &Cow::from("width"),
                //     &Some(AttributeValue::U32(self.value)),
                // );
                cx.add_new_dom_attribute_to_current_element(
                    |a| matches!(a, $crate::dom_attribute::DomAttr::$dom_interface($crate::dom_attribute::[<$dom_interface Attr>]::[<$attribute:camel>](_))),
                    &$crate::dom_attribute::DomAttr::$dom_interface($crate::dom_attribute::[<$dom_interface Attr>]::[<$attribute:camel>](self.value)),
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
}

#[allow(unused)]
pub(crate) use create_dom_attribute_view;
