use crate::{View, ViewMarker};
use std::borrow::Cow;

use gloo::events::EventListenerOptions;
use wasm_bindgen::JsCast;

use crate::{
    events::{self, OnEvent},
    Attr, IntoAttributeValue, OptionalAction,
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

// TODO should the options be its own function `on_event_with_options`,
// or should that be done via the builder pattern: `el.on_event().passive(false)`?
macro_rules! event_handler_mixin {
    ($(($event_ty: ident, $fn_name:ident, $event:expr, $web_sys_event_type:ident),)*) => {
    $(
        fn $fn_name<EH, OA>(self, handler: EH) -> events::$event_ty<T, A, Self, EH>
        where
            OA: OptionalAction<A>,
            EH: Fn(&mut T, web_sys::$web_sys_event_type) -> OA,
        {
            $crate::events::$event_ty::new(self, handler)
        }
    )*
    };
}

pub trait Element<T, A = ()>: View<T, A> + ViewMarker + sealed::Sealed
where
    Self: Sized,
{
    fn on<E, EH, OA>(self, event: impl Into<Cow<'static, str>>, handler: EH) -> OnEvent<Self, E, EH>
    where
        E: JsCast + 'static,
        OA: OptionalAction<A>,
        EH: Fn(&mut T, E) -> OA,
        Self: Sized,
    {
        OnEvent::new(self, event, handler)
    }

    fn on_with_options<E, EH, OA>(
        self,
        event: impl Into<Cow<'static, str>>,
        handler: EH,
        options: EventListenerOptions,
    ) -> OnEvent<Self, E, EH>
    where
        E: JsCast + 'static,
        OA: OptionalAction<A>,
        EH: Fn(&mut T, E) -> OA,
        Self: Sized,
    {
        OnEvent::new_with_options(self, event, handler, options)
    }

    // TODO should the API be "functional" in the sense, that new attributes are wrappers around the type,
    // or should they modify the underlying instance (e.g. via the following methods)?
    // The disadvantage that "functional" brings in, is that elements are not modifiable (i.e. attributes can't be simply added etc.)
    // fn attrs(&self) -> &Attributes;
    // fn attrs_mut(&mut self) -> &mut Attributes;

    /// Set an attribute on this element.
    ///
    /// # Panics
    ///
    /// If the name contains characters that are not valid in an attribute name,
    /// then the `View::build`/`View::rebuild` functions will panic for this view.
    fn attr(
        self,
        name: impl Into<Cow<'static, str>>,
        value: impl IntoAttributeValue,
    ) -> Attr<T, A, Self> {
        Attr {
            element: self,
            name: name.into(),
            value: value.into_attribute_value(),
            phantom: std::marker::PhantomData,
        }
    }

    // TODO should some methods extend some properties automatically,
    // instead of overwriting the (possibly set) inner value
    // or should there be (extra) "modifier" methods like `add_class` and/or `remove_class`
    fn class(self, class: impl Into<Cow<'static, str>>) -> Attr<T, A, Self> {
        self.attr("class", class.into())
    }

    // event list from
    // https://html.spec.whatwg.org/multipage/webappapis.html#idl-definitions
    //
    // I didn't include the events on the window, since we aren't attaching
    // any events to the window in xilem_html
    event_handler_mixin!(
        (OnAbort, on_abort, "abort", Event),
        (OnAuxClick, on_auxclick, "auxclick", PointerEvent),
        (OnBeforeInput, on_beforeinput, "beforeinput", InputEvent),
        (OnBeforeMatch, on_beforematch, "beforematch", Event),
        (OnBeforeToggle, on_beforetoggle, "beforetoggle", Event),
        (OnBlur, on_blur, "blur", FocusEvent),
        (OnCancel, on_cancel, "cancel", Event),
        (OnCanPlay, on_canplay, "canplay", Event),
        (OnCanPlayThrough, on_canplaythrough, "canplaythrough", Event),
        (OnChange, on_change, "change", Event),
        (OnClick, on_click, "click", MouseEvent),
        (OnClose, on_close, "close", Event),
        (OnContextLost, on_contextlost, "contextlost", Event),
        (OnContextMenu, on_contextmenu, "contextmenu", PointerEvent),
        (
            OnContextRestored,
            on_contextrestored,
            "contextrestored",
            Event
        ),
        (OnCopy, on_copy, "copy", Event),
        (OnCueChange, on_cuechange, "cuechange", Event),
        (OnCut, on_cut, "cut", Event),
        (OnDblClick, on_dblclick, "dblclick", MouseEvent),
        (OnDrag, on_drag, "drag", Event),
        (OnDragEnd, on_dragend, "dragend", Event),
        (OnDragEnter, on_dragenter, "dragenter", Event),
        (OnDragLeave, on_dragleave, "dragleave", Event),
        (OnDragOver, on_dragover, "dragover", Event),
        (OnDragStart, on_dragstart, "dragstart", Event),
        (OnDrop, on_drop, "drop", Event),
        (OnDurationChange, on_durationchange, "durationchange", Event),
        (OnEmptied, on_emptied, "emptied", Event),
        (OnEnded, on_ended, "ended", Event),
        (OnError, on_error, "error", Event),
        (OnFocus, on_focus, "focus", FocusEvent),
        (OnFocusIn, on_focusin, "focusin", FocusEvent),
        (OnFocusOut, on_focusout, "focusout", FocusEvent),
        (OnFormData, on_formdata, "formdata", Event),
        (OnInput, on_input, "input", InputEvent),
        (OnInvalid, on_invalid, "invalid", Event),
        (OnKeyDown, on_keydown, "keydown", KeyboardEvent),
        (OnKeyUp, on_keyup, "keyup", KeyboardEvent),
        (OnLoad, on_load, "load", Event),
        (OnLoadedData, on_loadeddata, "loadeddata", Event),
        (OnLoadedMetadata, on_loadedmetadata, "loadedmetadata", Event),
        (OnLoadStart, on_loadstart, "loadstart", Event),
        (OnMouseDown, on_mousedown, "mousedown", MouseEvent),
        (OnMouseEnter, on_mouseenter, "mouseenter", MouseEvent),
        (OnMouseLeave, on_mouseleave, "mouseleave", MouseEvent),
        (OnMouseMove, on_mousemove, "mousemove", MouseEvent),
        (OnMouseOut, on_mouseout, "mouseout", MouseEvent),
        (OnMouseOver, on_mouseover, "mouseover", MouseEvent),
        (OnMouseUp, on_mouseup, "mouseup", MouseEvent),
        (OnPaste, on_paste, "paste", Event),
        (OnPause, on_pause, "pause", Event),
        (OnPlay, on_play, "play", Event),
        (OnPlaying, on_playing, "playing", Event),
        (OnProgress, on_progress, "progress", Event),
        (OnRateChange, on_ratechange, "ratechange", Event),
        (OnReset, on_reset, "reset", Event),
        (OnResize, on_resize, "resize", Event),
        (OnScroll, on_scroll, "scroll", Event),
        (OnScrollEnd, on_scrollend, "scrollend", Event),
        (
            OnSecurityPolicyViolation,
            on_securitypolicyviolation,
            "securitypolicyviolation",
            Event
        ),
        (OnSeeked, on_seeked, "seeked", Event),
        (OnSeeking, on_seeking, "seeking", Event),
        (OnSelect, on_select, "select", Event),
        (OnSlotChange, on_slotchange, "slotchange", Event),
        (OnStalled, on_stalled, "stalled", Event),
        (OnSubmit, on_submit, "submit", Event),
        (OnSuspend, on_suspend, "suspend", Event),
        (OnTimeUpdate, on_timeupdate, "timeupdate", Event),
        (OnToggle, on_toggle, "toggle", Event),
        (OnVolumeChange, on_volumechange, "volumechange", Event),
        (OnWaiting, on_waiting, "waiting", Event),
        (OnWheel, on_wheel, "wheel", WheelEvent),
    );
}

// base case for ancestor macros, do nothing, because the body is in all the child interface macros...
#[allow(unused_macros)]
macro_rules! for_all_element_ancestors {
    ($($_:tt)*) => {};
}
#[allow(unused_imports)]
pub(crate) use for_all_element_ancestors;

macro_rules! dom_interface_macro_and_trait_definitions_helper {
    ($interface:ident {
        methods: $_methods_body:tt,
        child_interfaces: {
            $($child_interface:ident {
                methods: $child_methods_body:tt,
                child_interfaces: $child_interface_body: tt
            },)*
        }
    }) => {
        paste::paste! {
            $(
                pub trait $child_interface<T, A = ()>: $interface<T, A> $child_methods_body

                /// Execute $mac which is a macro, that takes $dom_interface:ident (<optional macro parameters>) as match arm for all interfaces that
                #[doc = concat!("`", stringify!($child_interface), "`")]
                /// inherits from
                #[allow(unused_macros)]
                macro_rules! [<for_all_ $child_interface:snake _ancestors>] {
                    ($mac:path, $extra_params:tt) => {
                        $mac!($interface, $extra_params);
                        $crate::interfaces::[<for_all_ $interface:snake _ancestors>]!($mac, $extra_params);
                    };
                }
                #[allow(unused_imports)]
                pub(crate) use [<for_all_ $child_interface:snake _ancestors>];
            )*
        }
        paste::paste! {
            /// Execute $mac which is a macro, that takes $dom_interface:ident (<optional macro parameters>) as match arm for all interfaces that inherit from
            #[doc = concat!("`", stringify!($interface), "`")]
            #[allow(unused_macros)]
            macro_rules! [<for_all_ $interface:snake _descendents>] {
                ($mac:path, $extra_params:tt) => {
                    $(
                        $mac!($child_interface, $extra_params);
                        $crate::interfaces::[<for_all_ $child_interface:snake _ descendents>]!($mac, $extra_params);
                    )*
                };
            }
            #[allow(unused_imports)]
            pub(crate) use [<for_all_ $interface:snake _descendents>];
        }

        $(
            $crate::interfaces::dom_interface_macro_and_trait_definitions_helper!(
                $child_interface {
                    methods: $child_methods_body,
                    child_interfaces: $child_interface_body
                }
            );
        )*
    };
}
pub(crate) use dom_interface_macro_and_trait_definitions_helper;

/// Recursively generates trait and macro definitions for all interfaces, defined below
/// The macros that are defined with this macro are functionally composing a macro which is invoked for all ancestor and descendent interfaces of a given interface
/// There's also a macro `for_all_dom_interfaces` which is run for all interfaces defined below
/// For example `for_all_video_element_ancestors` is run for the interfaces `HtmlMediaElement`, `HtmlElement` and `Element`
/// And `for_all_html_media_element_descendents` is run for the interfaces `HtmlAudioElement` and `HtmlVideoElement`
macro_rules! dom_interface_macro_and_trait_definitions {
    ($($interface:ident $interface_body:tt,)*) => {
        $crate::interfaces::dom_interface_macro_and_trait_definitions_helper!(
            Element {
                methods: {},
                child_interfaces: {$($interface $interface_body,)*}
            }
        );

        /// Execute $mac which is a macro, that takes $dom_interface:ident (<optional macro parameters>), for all dom interfaces.
        /// It optionally passes arguments given to for_all_dom_interfaces! to $mac!
        macro_rules! for_all_dom_interfaces {
            ($mac:path, $extra_params:tt) => {
                $mac!(Element, $extra_params);
                paste::paste! {$(
                    $mac!($interface, $extra_params);
                    $crate::interfaces::[<for_all_ $interface:snake _descendents>]!($mac, $extra_params);
                )*}
            }
        }
        pub(crate) use for_all_dom_interfaces;
    }
}

dom_interface_macro_and_trait_definitions!(
    HtmlElement {
        methods: {},
        child_interfaces: {
            HtmlAnchorElement { methods: {}, child_interfaces: {} },
            HtmlAreaElement { methods: {}, child_interfaces: {} },
            HtmlBaseElement { methods: {}, child_interfaces: {} },
            HtmlBodyElement { methods: {}, child_interfaces: {} },
            HtmlBrElement { methods: {}, child_interfaces: {} },
            HtmlButtonElement { methods: {}, child_interfaces: {} },
            HtmlCanvasElement {
                methods: {
                    fn width(self, value: u32) -> Attr<T, A, Self> {
                        self.attr("width", value)
                    }
                    fn height(self, value: u32) -> Attr<T, A, Self> {
                        self.attr("height", value)
                    }
                },
                child_interfaces: {}
            },
            HtmlDataElement { methods: {}, child_interfaces: {} },
            HtmlDataListElement { methods: {}, child_interfaces: {} },
            HtmlDetailsElement { methods: {}, child_interfaces: {} },
            HtmlDialogElement { methods: {}, child_interfaces: {} },
            HtmlDirectoryElement { methods: {}, child_interfaces: {} },
            HtmlDivElement { methods: {}, child_interfaces: {} },
            HtmlDListElement { methods: {}, child_interfaces: {} },
            HtmlUnknownElement { methods: {}, child_interfaces: {} },
            HtmlEmbedElement { methods: {}, child_interfaces: {} },
            HtmlFieldSetElement { methods: {}, child_interfaces: {} },
            HtmlFontElement { methods: {}, child_interfaces: {} },
            HtmlFormElement { methods: {}, child_interfaces: {} },
            HtmlFrameElement { methods: {}, child_interfaces: {} },
            HtmlFrameSetElement { methods: {}, child_interfaces: {} },
            HtmlHeadElement { methods: {}, child_interfaces: {} },
            HtmlHeadingElement { methods: {}, child_interfaces: {} },
            HtmlHrElement { methods: {}, child_interfaces: {} },
            HtmlHtmlElement { methods: {}, child_interfaces: {} },
            HtmlIFrameElement { methods: {}, child_interfaces: {} },
            HtmlImageElement { methods: {}, child_interfaces: {} },
            HtmlInputElement { methods: {}, child_interfaces: {} },
            HtmlLabelElement { methods: {}, child_interfaces: {} },
            HtmlLegendElement { methods: {}, child_interfaces: {} },
            HtmlLiElement { methods: {}, child_interfaces: {} },
            HtmlLinkElement { methods: {}, child_interfaces: {} },
            HtmlMapElement { methods: {}, child_interfaces: {} },
            HtmlMediaElement {
                methods: {},
                child_interfaces: {
                    HtmlAudioElement { methods: {}, child_interfaces: {} },
                    HtmlVideoElement {
                        methods: {
                            fn width(self, value: u32) -> Attr<T, A, Self> {
                                self.attr("width", value)
                            }
                            fn height(self, value: u32) -> Attr<T, A, Self> {
                                self.attr("height", value)
                            }
                        },
                        child_interfaces: {}
                    },
                }
            },
            HtmlMenuElement { methods: {}, child_interfaces: {} },
            HtmlMenuItemElement { methods: {}, child_interfaces: {} },
            HtmlMetaElement { methods: {}, child_interfaces: {} },
            HtmlMeterElement { methods: {}, child_interfaces: {} },
            HtmlModElement { methods: {}, child_interfaces: {} },
            HtmlObjectElement { methods: {}, child_interfaces: {} },
            HtmlOListElement { methods: {}, child_interfaces: {} },
            HtmlOptGroupElement { methods: {}, child_interfaces: {} },
            HtmlOptionElement { methods: {}, child_interfaces: {} },
            HtmlOutputElement { methods: {}, child_interfaces: {} },
            HtmlParagraphElement { methods: {}, child_interfaces: {} },
            HtmlParamElement { methods: {}, child_interfaces: {} },
            HtmlPictureElement { methods: {}, child_interfaces: {} },
            HtmlPreElement { methods: {}, child_interfaces: {} },
            HtmlProgressElement { methods: {}, child_interfaces: {} },
            HtmlQuoteElement { methods: {}, child_interfaces: {} },
            HtmlScriptElement { methods: {}, child_interfaces: {} },
            HtmlSelectElement { methods: {}, child_interfaces: {} },
            HtmlSlotElement { methods: {}, child_interfaces: {} },
            HtmlSourceElement { methods: {}, child_interfaces: {} },
            HtmlSpanElement { methods: {}, child_interfaces: {} },
            HtmlStyleElement { methods: {}, child_interfaces: {} },
            HtmlTableCaptionElement { methods: {}, child_interfaces: {} },
            HtmlTableCellElement { methods: {}, child_interfaces: {} },
            HtmlTableColElement { methods: {}, child_interfaces: {} },
            HtmlTableElement { methods: {}, child_interfaces: {} },
            HtmlTableRowElement { methods: {}, child_interfaces: {} },
            HtmlTableSectionElement { methods: {}, child_interfaces: {} },
            HtmlTemplateElement { methods: {}, child_interfaces: {} },
            HtmlTimeElement { methods: {}, child_interfaces: {} },
            HtmlTextAreaElement { methods: {}, child_interfaces: {} },
            HtmlTitleElement { methods: {}, child_interfaces: {} },
            HtmlTrackElement { methods: {}, child_interfaces: {} },
            HtmlUListElement { methods: {}, child_interfaces: {} },
        }
    },
);
