use crate::{events::impl_dom_interface_for_event_ty, HtmlMediaElementPlay};
use std::borrow::Cow;

use wasm_bindgen::JsCast;

use crate::{
    events::{self, impl_dom_interface_for_all_event_tys},
    Attr, AttributeValue, EventListener, IntoAttributeValue, OptionalAction,
};

// TODO should the options be its own function `on_event_with_options`,
// or should that be done via the builder pattern: `el.on_event().passive(false)`?
macro_rules! event_handler_mixin {
    ($(($event_ty: ident, $fn_name:ident, $event:expr, $web_sys_event_type:ident),)*) => {
    $(
        fn $fn_name<EH, OA>(self, handler: EH) -> events::$event_ty<Self, EH>
        where
            OA: OptionalAction<A>,
            EH: Fn(&mut T, web_sys::$web_sys_event_type) -> OA,
        {
            events::$event_ty::new(self, handler)
        }
    )*
    };
}

use super::Node;
// TODO should Node or even EventTarget have the super trait View instead?
pub trait Element<T, A = ()>: Node<T, A>
where
    Self: Sized,
{
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
    ) -> Attr<Self> {
        Attr {
            element: self,
            name: name.into(),
            value: value.into_attribute_value(),
        }
    }

    // TODO should some methods extend some properties automatically,
    // instead of overwriting the (possibly set) inner value
    // or should there be (extra) "modifier" methods like `add_class` and/or `remove_class`
    fn class(self, class: impl Into<Cow<'static, str>>) -> Attr<Self> {
        self.attr("class", AttributeValue::String(class.into()))
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

impl<T, A, E: Element<T, A>> Element<T, A> for Attr<E> {}
impl<T, A, E: Element<T, A>> Element<T, A> for HtmlMediaElementPlay<E> {}

impl<T, A, E, Ev, F, OA> Element<T, A> for EventListener<E, Ev, F>
where
    F: Fn(&mut T, Ev) -> OA,
    E: Element<T, A>,
    Ev: JsCast + 'static,
    OA: OptionalAction<A>,
{
}

impl_dom_interface_for_all_event_tys!(Element);
