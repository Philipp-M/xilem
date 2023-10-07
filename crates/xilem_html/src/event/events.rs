use super::{create_event_listener, EventListenerOptions, EventListenerState};
use crate::Hydrate;
use std::any::Any;

use xilem_core::{Id, MessageResult};

use crate::{
    interfaces::EventTarget, view::DomNode, ChangeFlags, Cx, OptionalAction, View, ViewMarker,
};

macro_rules! event_definitions {
    ($(($ty_name:ident, $event_name:literal, $web_sys_ty:ident)),*) => {
        $(
pub struct $ty_name<ET, C> {
    target: ET,
    callback: C,
    options: EventListenerOptions,
}

impl<ET, C> $ty_name<ET, C> {
    pub fn new(target: ET, callback: C) -> Self {
        Self {
            target,
            options: Default::default(),
            callback,
        }
    }

    /// Whether the event handler should be passive. (default = `true`)
    ///
    /// Passive event handlers can't prevent the browser's default action from
    /// running (otherwise possible with `event.prevent_default()`), which
    /// restricts what they can be used for, but reduces overhead.
    pub fn passive(mut self, value: bool) -> Self {
        self.options.passive = value;
        self
    }
}

impl<ET, C> ViewMarker for $ty_name<ET, C> {}

impl<T, A, C, ET, OA> View<T, A> for $ty_name<ET, C>
where
    OA: OptionalAction<A>,
    C: Fn(&mut T, web_sys::$web_sys_ty) -> OA,
    ET: EventTarget<T, A>,
{
    type State = EventListenerState<ET::State>;

    type Element = ET::Element;

    fn build(&self, cx: &mut Cx) -> (Id, Self::State, Self::Element) {
        let (id, (element, state)) = cx.with_new_id(|cx| {
            let (child_id, child_state, el) = self.target.build(cx);
            let listener = create_event_listener::<web_sys::$web_sys_ty>(el.as_node_ref(), $event_name, self.options, cx);
            (el, EventListenerState { child_state, child_id, listener })
        });
        (id, state, element)
    }

    fn rebuild(
        &self,
        cx: &mut Cx,
        prev: &Self,
        id: &mut Id,
        state: &mut Self::State,
        element: &mut Self::Element,
    ) -> ChangeFlags {
        cx.with_id(*id, |cx| {
            let mut changed = self.target.rebuild(cx, &prev.target, id, &mut state.child_state, element);
            // TODO check equality of prev and current element somehow
            if changed.contains(ChangeFlags::STRUCTURE) {
                state.listener = create_event_listener::<web_sys::$web_sys_ty>(element.as_node_ref(), $event_name, self.options, cx);
                changed |= ChangeFlags::OTHER_CHANGE;
            }
            changed
        })
    }

    fn message(
        &self,
        id_path: &[Id],
        state: &mut Self::State,
        message: Box<dyn Any>,
        app_state: &mut T,
    ) -> MessageResult<A> {
        match id_path {
            [] if message.downcast_ref::<web_sys::$web_sys_ty>().is_some() => {
                let event = message.downcast::<web_sys::$web_sys_ty>().unwrap();
                match (self.callback)(app_state, *event).action() {
                    Some(a) => MessageResult::Action(a),
                    None => MessageResult::Nop,
                }
            }
            [element_id, rest_path @ ..] if *element_id == state.child_id => {
                self.target.message(rest_path, &mut state.child_state, message, app_state)
            }
            _ => MessageResult::Stale(message),
        }
    }
}

impl<T, A, C, ET, OA> Hydrate<T, A> for $ty_name<ET, C>
where
    OA: OptionalAction<A>,
    C: Fn(&mut T, web_sys::$web_sys_ty) -> OA,
    ET: EventTarget<T, A> + Hydrate<T, A>,
{
    // TODO basically identical as View::build, but instead using hydrate, so maybe macro?
    fn hydrate(&self, cx: &mut Cx, element: web_sys::Node) -> (Id, Self::State, Self::Element) {
        let (id, (element, state)) = cx.with_new_id(|cx| {
            let (child_id, child_state, el) = self.target.hydrate(cx, element);
            let listener = create_event_listener::<web_sys::$web_sys_ty>(el.as_node_ref(), $event_name, self.options, cx);
            (el, EventListenerState { child_state, child_id, listener })
        });
        (id, state, element)
    }
}
        )*
    };
}

macro_rules! impl_dom_interface_for_event_ty {
    ($dom_interface:ident, $event_ty:ident, $web_sys_ty: ident) => {
        impl<T, A, E, C, OA> $dom_interface<T, A> for $crate::events::$event_ty<E, C>
        where
            E: $crate::interfaces::$dom_interface<T, A>,
            OA: OptionalAction<A>,
            C: Fn(&mut T, web_sys::$web_sys_ty) -> OA,
        {
        }
    };
}

pub(crate) use impl_dom_interface_for_event_ty;

macro_rules! impl_dom_interface_for_all_event_tys {
    ($dom_interface: ident) => {
        impl_dom_interface_for_all_event_tys!(
            ($dom_interface, OnAbort, Event),
            ($dom_interface, OnAuxClick, MouseEvent),
            ($dom_interface, OnBeforeInput, InputEvent),
            ($dom_interface, OnBeforeMatch, Event),
            ($dom_interface, OnBeforeToggle, Event),
            ($dom_interface, OnBlur, FocusEvent),
            ($dom_interface, OnCancel, Event),
            ($dom_interface, OnCanPlay, Event),
            ($dom_interface, OnCanPlayThrough, Event),
            ($dom_interface, OnChange, Event),
            ($dom_interface, OnClick, MouseEvent),
            ($dom_interface, OnClose, Event),
            ($dom_interface, OnContextLost, Event),
            ($dom_interface, OnContextMenu, MouseEvent),
            ($dom_interface, OnContextRestored, Event),
            ($dom_interface, OnCopy, Event),
            ($dom_interface, OnCueChange, Event),
            ($dom_interface, OnCut, Event),
            ($dom_interface, OnDblClick, MouseEvent),
            ($dom_interface, OnDrag, Event),
            ($dom_interface, OnDragEnd, Event),
            ($dom_interface, OnDragEnter, Event),
            ($dom_interface, OnDragLeave, Event),
            ($dom_interface, OnDragOver, Event),
            ($dom_interface, OnDragStart, Event),
            ($dom_interface, OnDrop, Event),
            ($dom_interface, OnDurationChange, Event),
            ($dom_interface, OnEmptied, Event),
            ($dom_interface, OnEnded, Event),
            ($dom_interface, OnError, Event),
            ($dom_interface, OnFocus, FocusEvent),
            ($dom_interface, OnFocusIn, FocusEvent),
            ($dom_interface, OnFocusOut, FocusEvent),
            ($dom_interface, OnFormData, Event),
            ($dom_interface, OnInput, InputEvent),
            ($dom_interface, OnInvalid, Event),
            ($dom_interface, OnKeyDown, KeyboardEvent),
            ($dom_interface, OnKeyUp, KeyboardEvent),
            ($dom_interface, OnLoad, Event),
            ($dom_interface, OnLoadedData, Event),
            ($dom_interface, OnLoadedMetadata, Event),
            ($dom_interface, OnLoadStart, Event),
            ($dom_interface, OnMouseDown, MouseEvent),
            ($dom_interface, OnMouseEnter, MouseEvent),
            ($dom_interface, OnMouseLeave, MouseEvent),
            ($dom_interface, OnMouseMove, MouseEvent),
            ($dom_interface, OnMouseOut, MouseEvent),
            ($dom_interface, OnMouseOver, MouseEvent),
            ($dom_interface, OnMouseUp, MouseEvent),
            ($dom_interface, OnPaste, Event),
            ($dom_interface, OnPause, Event),
            ($dom_interface, OnPlay, Event),
            ($dom_interface, OnPlaying, Event),
            ($dom_interface, OnProgress, Event),
            ($dom_interface, OnRateChange, Event),
            ($dom_interface, OnReset, Event),
            ($dom_interface, OnResize, Event),
            ($dom_interface, OnScroll, Event),
            ($dom_interface, OnScrollEnd, Event),
            ($dom_interface, OnSecurityPolicyViolation, Event),
            ($dom_interface, OnSeeked, Event),
            ($dom_interface, OnSeeking, Event),
            ($dom_interface, OnSelect, Event),
            ($dom_interface, OnSlotChange, Event),
            ($dom_interface, OnStalled, Event),
            ($dom_interface, OnSubmit, Event),
            ($dom_interface, OnSuspend, Event),
            ($dom_interface, OnTimeUpdate, Event),
            ($dom_interface, OnToggle, Event),
            ($dom_interface, OnVolumeChange, Event),
            ($dom_interface, OnWaiting, Event),
            ($dom_interface, OnWheel, WheelEvent)
        );
    };
    ($(($dom_interface: ident, $ty_name:ident, $web_sys_ty:ident)),*) => {
        $(
            impl_dom_interface_for_event_ty!($dom_interface, $ty_name, $web_sys_ty);
        )*
    };
}

pub(crate) use impl_dom_interface_for_all_event_tys;

macro_rules! impl_node_for_all_event_tys {
    () => {
        impl_node_for_all_event_tys!(
            (OnAbort, Event),
            (OnAuxClick, MouseEvent),
            (OnBeforeInput, InputEvent),
            (OnBeforeMatch, Event),
            (OnBeforeToggle, Event),
            (OnBlur, FocusEvent),
            (OnCancel, Event),
            (OnCanPlay, Event),
            (OnCanPlayThrough, Event),
            (OnChange, Event),
            (OnClick, MouseEvent),
            (OnClose, Event),
            (OnContextLost, Event),
            (OnContextMenu, MouseEvent),
            (OnContextRestored, Event),
            (OnCopy, Event),
            (OnCueChange, Event),
            (OnCut, Event),
            (OnDblClick, MouseEvent),
            (OnDrag, Event),
            (OnDragEnd, Event),
            (OnDragEnter, Event),
            (OnDragLeave, Event),
            (OnDragOver, Event),
            (OnDragStart, Event),
            (OnDrop, Event),
            (OnDurationChange, Event),
            (OnEmptied, Event),
            (OnEnded, Event),
            (OnError, Event),
            (OnFocus, FocusEvent),
            (OnFocusIn, FocusEvent),
            (OnFocusOut, FocusEvent),
            (OnFormData, Event),
            (OnInput, InputEvent),
            (OnInvalid, Event),
            (OnKeyDown, KeyboardEvent),
            (OnKeyUp, KeyboardEvent),
            (OnLoad, Event),
            (OnLoadedData, Event),
            (OnLoadedMetadata, Event),
            (OnLoadStart, Event),
            (OnMouseDown, MouseEvent),
            (OnMouseEnter, MouseEvent),
            (OnMouseLeave, MouseEvent),
            (OnMouseMove, MouseEvent),
            (OnMouseOut, MouseEvent),
            (OnMouseOver, MouseEvent),
            (OnMouseUp, MouseEvent),
            (OnPaste, Event),
            (OnPause, Event),
            (OnPlay, Event),
            (OnPlaying, Event),
            (OnProgress, Event),
            (OnRateChange, Event),
            (OnReset, Event),
            (OnResize, Event),
            (OnScroll, Event),
            (OnScrollEnd, Event),
            (OnSecurityPolicyViolation, Event),
            (OnSeeked, Event),
            (OnSeeking, Event),
            (OnSelect, Event),
            (OnSlotChange, Event),
            (OnStalled, Event),
            (OnSubmit, Event),
            (OnSuspend, Event),
            (OnTimeUpdate, Event),
            (OnToggle, Event),
            (OnVolumeChange, Event),
            (OnWaiting, Event),
            (OnWheel, WheelEvent)
        );
    };
    ($(($ty_name:ident, $web_sys_ty:ident)),*) => {
        $(
            impl<T, A, E, C, OA> crate::interfaces::Node<T, A> for $ty_name<E, C>
            where
                E: crate::interfaces::Node<T, A>,
                OA: OptionalAction<A>,
                C: Fn(&mut T, web_sys::$web_sys_ty) -> OA,
            {
                fn node_name(&self) -> &str {
                    self.target.node_name()
                }
            }
        )*
    };
}

impl_node_for_all_event_tys!();
// impl_dom_interface_for_event_ty!(EventTarget, OnClick, MouseEvent);

// click/auxclick/contextmenu are still mouse events in either Safari as well as Firefox,
// see: https://stackoverflow.com/questions/70626381/why-chrome-emits-pointerevents-and-firefox-mouseevents-and-which-type-definition/76900433#76900433
event_definitions!(
    (OnAbort, "abort", Event),
    (OnAuxClick, "auxclick", MouseEvent),
    (OnBeforeInput, "beforeinput", InputEvent),
    (OnBeforeMatch, "beforematch", Event),
    (OnBeforeToggle, "beforetoggle", Event),
    (OnBlur, "blur", FocusEvent),
    (OnCancel, "cancel", Event),
    (OnCanPlay, "canplay", Event),
    (OnCanPlayThrough, "canplaythrough", Event),
    (OnChange, "change", Event),
    (OnClick, "click", MouseEvent),
    (OnClose, "close", Event),
    (OnContextLost, "contextlost", Event),
    (OnContextMenu, "contextmenu", MouseEvent),
    (OnContextRestored, "contextrestored", Event),
    (OnCopy, "copy", Event),
    (OnCueChange, "cuechange", Event),
    (OnCut, "cut", Event),
    (OnDblClick, "dblclick", MouseEvent),
    (OnDrag, "drag", Event),
    (OnDragEnd, "dragend", Event),
    (OnDragEnter, "dragenter", Event),
    (OnDragLeave, "dragleave", Event),
    (OnDragOver, "dragover", Event),
    (OnDragStart, "dragstart", Event),
    (OnDrop, "drop", Event),
    (OnDurationChange, "durationchange", Event),
    (OnEmptied, "emptied", Event),
    (OnEnded, "ended", Event),
    (OnError, "error", Event),
    (OnFocus, "focus", FocusEvent),
    (OnFocusIn, "focusin", FocusEvent),
    (OnFocusOut, "focusout", FocusEvent),
    (OnFormData, "formdata", Event),
    (OnInput, "input", InputEvent),
    (OnInvalid, "invalid", Event),
    (OnKeyDown, "keydown", KeyboardEvent),
    (OnKeyUp, "keyup", KeyboardEvent),
    (OnLoad, "load", Event),
    (OnLoadedData, "loadeddata", Event),
    (OnLoadedMetadata, "loadedmetadata", Event),
    (OnLoadStart, "loadstart", Event),
    (OnMouseDown, "mousedown", MouseEvent),
    (OnMouseEnter, "mouseenter", MouseEvent),
    (OnMouseLeave, "mouseleave", MouseEvent),
    (OnMouseMove, "mousemove", MouseEvent),
    (OnMouseOut, "mouseout", MouseEvent),
    (OnMouseOver, "mouseover", MouseEvent),
    (OnMouseUp, "mouseup", MouseEvent),
    (OnPaste, "paste", Event),
    (OnPause, "pause", Event),
    (OnPlay, "play", Event),
    (OnPlaying, "playing", Event),
    (OnProgress, "progress", Event),
    (OnRateChange, "ratechange", Event),
    (OnReset, "reset", Event),
    (OnResize, "resize", Event),
    (OnScroll, "scroll", Event),
    (OnScrollEnd, "scrollend", Event),
    (OnSecurityPolicyViolation, "securitypolicyviolation", Event),
    (OnSeeked, "seeked", Event),
    (OnSeeking, "seeking", Event),
    (OnSelect, "select", Event),
    (OnSlotChange, "slotchange", Event),
    (OnStalled, "stalled", Event),
    (OnSubmit, "submit", Event),
    (OnSuspend, "suspend", Event),
    (OnTimeUpdate, "timeupdate", Event),
    (OnToggle, "toggle", Event),
    (OnVolumeChange, "volumechange", Event),
    (OnWaiting, "waiting", Event),
    (OnWheel, "wheel", WheelEvent)
);
