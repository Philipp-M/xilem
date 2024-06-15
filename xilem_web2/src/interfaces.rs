use crate::{
    events,
    style::{IntoStyles, Style, WithStyle},
    AsClassIter, Attr, Class, DomView, IntoAttributeValue, OptionalAction, WithAttributes,
    WithClasses,
};
use wasm_bindgen::JsCast;

type CowStr = std::borrow::Cow<'static, str>;

macro_rules! event_handler_mixin {
    ($(($event_ty: ident, $fn_name:ident, $event:expr, $web_sys_event_type:ident),)*) => {
    $(
        fn $fn_name<Callback, OA>(
            self,
            handler: Callback,
        ) -> events::$event_ty<Self, State, Action, Callback>
        where
            Self: Sized,
            Self::Element: AsRef<web_sys::Element>,
            OA: OptionalAction<Action>,
            Callback: Fn(&mut State, web_sys::$web_sys_event_type) -> OA,
        {
            events::$event_ty::new(self, handler)
        }
    )*
    };
}

pub trait Element<State, Action = ()>:
    Sized
    + DomView<State, Action, Props: WithAttributes + WithClasses, DomNode: AsRef<web_sys::Element>>
{
    fn attr(
        self,
        name: impl Into<CowStr>,
        value: impl IntoAttributeValue,
    ) -> Attr<Self, State, Action> {
        Attr::new(self, name.into(), value.into_attr_value())
    }

    fn class<AsClasses: AsClassIter>(
        self,
        as_classes: AsClasses,
    ) -> Class<Self, AsClasses, State, Action> {
        Class::new(self, as_classes)
    }

    fn on<Event, Callback, OA>(
        self,
        event: impl Into<CowStr>,
        handler: Callback,
    ) -> events::OnEvent<Self, State, Action, Event, Callback>
    where
        Self::Element: AsRef<web_sys::Element>,
        Event: JsCast + 'static,
        OA: OptionalAction<Action>,
        Callback: Fn(&mut State, Event) -> OA,
        Self: Sized,
    {
        events::OnEvent::new(self, event, handler)
    }

    // event list from
    // https://html.spec.whatwg.org/multipage/webappapis.html#idl-definitions
    //
    // I didn't include the events on the window, since we aren't attaching
    // any events to the window in xilem_web
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
        (OnInput, on_input, "input", Event),
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

impl<State, Action, T> Element<State, Action> for T
where
    T: DomView<State, Action>,
    T::Props: WithAttributes + WithClasses,
    T::DomNode: AsRef<web_sys::Element>,
{
}

// #[cfg(feature = "HtmlAnchorElement")]
pub trait HtmlAnchorElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlAnchorElement>>
{
}

// #[cfg(feature = "HtmlAnchorElement")]
impl<State, Action, T> HtmlAnchorElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlAnchorElement>,
{
}

// #[cfg(feature = "HtmlAreaElement")]
pub trait HtmlAreaElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlAreaElement>>
{
}

// #[cfg(feature = "HtmlAreaElement")]
impl<State, Action, T> HtmlAreaElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlAreaElement>,
{
}

// #[cfg(feature = "HtmlAudioElement")]
pub trait HtmlAudioElement<State, Action = ()>:
    HtmlMediaElement<State, Action, DomNode: AsRef<web_sys::HtmlAudioElement>>
{
}

// #[cfg(feature = "HtmlAudioElement")]
impl<State, Action, T> HtmlAudioElement<State, Action> for T
where
    T: HtmlMediaElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlAudioElement>,
{
}

// #[cfg(feature = "HtmlBaseElement")]
// pub trait HtmlBaseElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlBaseElement>>
// {
// }

// #[cfg(feature = "HtmlBaseElement")]
// impl<State, Action, T> HtmlBaseElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlBaseElement>,
// {
// }

// #[cfg(feature = "HtmlBodyElement")]
// pub trait HtmlBodyElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlBodyElement>>
// {
// }

// #[cfg(feature = "HtmlBodyElement")]
// impl<State, Action, T> HtmlBodyElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlBodyElement>,
// {
// }

// #[cfg(feature = "HtmlBrElement")]
pub trait HtmlBrElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlBrElement>>
{
}

// #[cfg(feature = "HtmlBrElement")]
impl<State, Action, T> HtmlBrElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlBrElement>,
{
}

// #[cfg(feature = "HtmlButtonElement")]
pub trait HtmlButtonElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlButtonElement>>
{
}

// #[cfg(feature = "HtmlButtonElement")]
impl<State, Action, T> HtmlButtonElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlButtonElement>,
{
}

// #[cfg(feature = "HtmlCanvasElement")]
pub trait HtmlCanvasElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlCanvasElement>>
{
    fn width(self, value: u32) -> Attr<Self, State, Action>
    where
        Self: Sized,
        Self::Element: WithAttributes,
        // The following bound would be more correct, but the current trait solver is not capable enough
        // (the new trait solver is able to do handle this though...)
        // but the bound above is enough for the API for now
        // Self::Element: ElementWithAttributes,
    {
        Attr::new(self, "width".into(), value.into_attr_value())
    }
}

// #[cfg(feature = "HtmlCanvasElement")]
impl<State, Action, T> HtmlCanvasElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlCanvasElement>,
{
}

// #[cfg(feature = "HtmlDataElement")]
pub trait HtmlDataElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlDataElement>>
{
}

// #[cfg(feature = "HtmlDataElement")]
impl<State, Action, T> HtmlDataElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlDataElement>,
{
}

// #[cfg(feature = "HtmlDataListElement")]
pub trait HtmlDataListElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlDataListElement>>
{
}

// #[cfg(feature = "HtmlDataListElement")]
impl<State, Action, T> HtmlDataListElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlDataListElement>,
{
}

// #[cfg(feature = "HtmlDetailsElement")]
pub trait HtmlDetailsElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlDetailsElement>>
{
}

// #[cfg(feature = "HtmlDetailsElement")]
impl<State, Action, T> HtmlDetailsElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlDetailsElement>,
{
}

// #[cfg(feature = "HtmlDialogElement")]
pub trait HtmlDialogElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlDialogElement>>
{
}

// #[cfg(feature = "HtmlDialogElement")]
impl<State, Action, T> HtmlDialogElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlDialogElement>,
{
}

// #[cfg(feature = "HtmlDirectoryElement")]
// pub trait HtmlDirectoryElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlDirectoryElement>>
// {
// }

// #[cfg(feature = "HtmlDirectoryElement")]
// impl<State, Action, T> HtmlDirectoryElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlDirectoryElement>,
// {
// }

// #[cfg(feature = "HtmlDivElement")]
pub trait HtmlDivElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlDivElement>>
{
}

// #[cfg(feature = "HtmlDivElement")]
impl<State, Action, T> HtmlDivElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlDivElement>,
{
}

// #[cfg(feature = "HtmlDListElement")]
pub trait HtmlDListElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlDListElement>>
{
}

// #[cfg(feature = "HtmlDListElement")]
impl<State, Action, T> HtmlDListElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlDListElement>,
{
}

// #[cfg(feature = "HtmlElement")]
pub trait HtmlElement<State, Action = ()>:
    Element<State, Action, Props: WithStyle, DomNode: AsRef<web_sys::HtmlElement>>
{
    /// Set a style attribute
    fn style(self, style: impl IntoStyles) -> Style<Self, State, Action> {
        let mut styles = vec![];
        style.into_styles(&mut styles);
        Style::new(self, styles)
    }
}

// #[cfg(feature = "HtmlElement")]
impl<State, Action, T> HtmlElement<State, Action> for T
where
    T: Element<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlElement>,
    T::Props: WithStyle,
{
}

// #[cfg(feature = "HtmlUnknownElement")]
// pub trait HtmlUnknownElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlUnknownElement>>
// {
// }

// #[cfg(feature = "HtmlUnknownElement")]
// impl<State, Action, T> HtmlUnknownElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlUnknownElement>,
// {
// }

// #[cfg(feature = "HtmlEmbedElement")]
pub trait HtmlEmbedElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlEmbedElement>>
{
}

// #[cfg(feature = "HtmlEmbedElement")]
impl<State, Action, T> HtmlEmbedElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlEmbedElement>,
{
}

// #[cfg(feature = "HtmlFieldSetElement")]
pub trait HtmlFieldSetElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlFieldSetElement>>
{
}

// #[cfg(feature = "HtmlFieldSetElement")]
impl<State, Action, T> HtmlFieldSetElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlFieldSetElement>,
{
}

// #[cfg(feature = "HtmlFontElement")]
// pub trait HtmlFontElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlFontElement>>
// {
// }

// #[cfg(feature = "HtmlFontElement")]
// impl<State, Action, T> HtmlFontElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlFontElement>,
// {
// }

// #[cfg(feature = "HtmlFormElement")]
pub trait HtmlFormElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlFormElement>>
{
}

// #[cfg(feature = "HtmlFormElement")]
impl<State, Action, T> HtmlFormElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlFormElement>,
{
}

// #[cfg(feature = "HtmlFrameElement")]
// pub trait HtmlFrameElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlFrameElement>>
// {
// }

// #[cfg(feature = "HtmlFrameElement")]
// impl<State, Action, T> HtmlFrameElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlFrameElement>,
// {
// }

// #[cfg(feature = "HtmlFrameSetElement")]
// pub trait HtmlFrameSetElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlFrameSetElement>>
// {
// }

// #[cfg(feature = "HtmlFrameSetElement")]
// impl<State, Action, T> HtmlFrameSetElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlFrameSetElement>,
// {
// }

// #[cfg(feature = "HtmlHeadElement")]
// pub trait HtmlHeadElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlHeadElement>>
// {
// }

// #[cfg(feature = "HtmlHeadElement")]
// impl<State, Action, T> HtmlHeadElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlHeadElement>,
// {
// }

// #[cfg(feature = "HtmlHeadingElement")]
pub trait HtmlHeadingElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlHeadingElement>>
{
}

// #[cfg(feature = "HtmlHeadingElement")]
impl<State, Action, T> HtmlHeadingElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlHeadingElement>,
{
}

// #[cfg(feature = "HtmlHrElement")]
pub trait HtmlHrElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlHrElement>>
{
}

// #[cfg(feature = "HtmlHrElement")]
impl<State, Action, T> HtmlHrElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlHrElement>,
{
}

// #[cfg(feature = "HtmlHtmlElement")]
// pub trait HtmlHtmlElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlHtmlElement>>
// {
// }

// #[cfg(feature = "HtmlHtmlElement")]
// impl<State, Action, T> HtmlHtmlElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlHtmlElement>,
// {
// }

// #[cfg(feature = "HtmlIFrameElement")]
pub trait HtmlIFrameElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlIFrameElement>>
{
}

// #[cfg(feature = "HtmlIFrameElement")]
impl<State, Action, T> HtmlIFrameElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlIFrameElement>,
{
}

// #[cfg(feature = "HtmlImageElement")]
pub trait HtmlImageElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlImageElement>>
{
}

// #[cfg(feature = "HtmlImageElement")]
impl<State, Action, T> HtmlImageElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlImageElement>,
{
}

// #[cfg(feature = "HtmlInputElement")]
pub trait HtmlInputElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlInputElement>>
{
}

// #[cfg(feature = "HtmlInputElement")]
impl<State, Action, T> HtmlInputElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlInputElement>,
{
}

// #[cfg(feature = "HtmlLabelElement")]
pub trait HtmlLabelElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlLabelElement>>
{
}

// #[cfg(feature = "HtmlLabelElement")]
impl<State, Action, T> HtmlLabelElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlLabelElement>,
{
}

// #[cfg(feature = "HtmlLegendElement")]
pub trait HtmlLegendElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlLegendElement>>
{
}

// #[cfg(feature = "HtmlLegendElement")]
impl<State, Action, T> HtmlLegendElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlLegendElement>,
{
}

// #[cfg(feature = "HtmlLiElement")]
pub trait HtmlLiElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlLiElement>>
{
}

// #[cfg(feature = "HtmlLiElement")]
impl<State, Action, T> HtmlLiElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlLiElement>,
{
}

// #[cfg(feature = "HtmlLinkElement")]
pub trait HtmlLinkElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlLinkElement>>
{
}

// #[cfg(feature = "HtmlLinkElement")]
impl<State, Action, T> HtmlLinkElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlLinkElement>,
{
}

// #[cfg(feature = "HtmlMapElement")]
pub trait HtmlMapElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlMapElement>>
{
}

// #[cfg(feature = "HtmlMapElement")]
impl<State, Action, T> HtmlMapElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlMapElement>,
{
}

// #[cfg(feature = "HtmlMediaElement")]
pub trait HtmlMediaElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlMediaElement>>
{
}

// #[cfg(feature = "HtmlMediaElement")]
impl<State, Action, T> HtmlMediaElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlMediaElement>,
{
}

// #[cfg(feature = "HtmlMenuElement")]
pub trait HtmlMenuElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlMenuElement>>
{
}

// #[cfg(feature = "HtmlMenuElement")]
impl<State, Action, T> HtmlMenuElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlMenuElement>,
{
}

// #[cfg(feature = "HtmlMenuItemElement")]
// pub trait HtmlMenuItemElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlMenuItemElement>>
// {
// }

// #[cfg(feature = "HtmlMenuItemElement")]
// impl<State, Action, T> HtmlMenuItemElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlMenuItemElement>,
// {
// }

// #[cfg(feature = "HtmlMetaElement")]
// pub trait HtmlMetaElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlMetaElement>>
// {
// }

// #[cfg(feature = "HtmlMetaElement")]
// impl<State, Action, T> HtmlMetaElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlMetaElement>,
// {
// }

// #[cfg(feature = "HtmlMeterElement")]
pub trait HtmlMeterElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlMeterElement>>
{
}

// #[cfg(feature = "HtmlMeterElement")]
impl<State, Action, T> HtmlMeterElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlMeterElement>,
{
}

// #[cfg(feature = "HtmlModElement")]
pub trait HtmlModElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlModElement>>
{
}

// #[cfg(feature = "HtmlModElement")]
impl<State, Action, T> HtmlModElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlModElement>,
{
}

// #[cfg(feature = "HtmlObjectElement")]
pub trait HtmlObjectElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlObjectElement>>
{
}

// #[cfg(feature = "HtmlObjectElement")]
impl<State, Action, T> HtmlObjectElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlObjectElement>,
{
}

// #[cfg(feature = "HtmlOListElement")]
pub trait HtmlOListElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlOListElement>>
{
}

// #[cfg(feature = "HtmlOListElement")]
impl<State, Action, T> HtmlOListElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlOListElement>,
{
}

// #[cfg(feature = "HtmlOptGroupElement")]
pub trait HtmlOptGroupElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlOptGroupElement>>
{
}

// #[cfg(feature = "HtmlOptGroupElement")]
impl<State, Action, T> HtmlOptGroupElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlOptGroupElement>,
{
}

// #[cfg(feature = "HtmlOptionElement")]
pub trait HtmlOptionElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlOptionElement>>
{
}

// #[cfg(feature = "HtmlOptionElement")]
impl<State, Action, T> HtmlOptionElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlOptionElement>,
{
}

// #[cfg(feature = "HtmlOutputElement")]
pub trait HtmlOutputElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlOutputElement>>
{
}

// #[cfg(feature = "HtmlOutputElement")]
impl<State, Action, T> HtmlOutputElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlOutputElement>,
{
}

// #[cfg(feature = "HtmlParagraphElement")]
pub trait HtmlParagraphElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlParagraphElement>>
{
}

// #[cfg(feature = "HtmlParagraphElement")]
impl<State, Action, T> HtmlParagraphElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlParagraphElement>,
{
}

// #[cfg(feature = "HtmlParamElement")]
// pub trait HtmlParamElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlParamElement>>
// {
// }

// #[cfg(feature = "HtmlParamElement")]
// impl<State, Action, T> HtmlParamElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlParamElement>,
// {
// }

// #[cfg(feature = "HtmlPictureElement")]
pub trait HtmlPictureElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlPictureElement>>
{
}

// #[cfg(feature = "HtmlPictureElement")]
impl<State, Action, T> HtmlPictureElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlPictureElement>,
{
}

// #[cfg(feature = "HtmlPreElement")]
pub trait HtmlPreElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlPreElement>>
{
}

// #[cfg(feature = "HtmlPreElement")]
impl<State, Action, T> HtmlPreElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlPreElement>,
{
}

// #[cfg(feature = "HtmlProgressElement")]
pub trait HtmlProgressElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlProgressElement>>
{
}

// #[cfg(feature = "HtmlProgressElement")]
impl<State, Action, T> HtmlProgressElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlProgressElement>,
{
}

// #[cfg(feature = "HtmlQuoteElement")]
pub trait HtmlQuoteElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlQuoteElement>>
{
}

// #[cfg(feature = "HtmlQuoteElement")]
impl<State, Action, T> HtmlQuoteElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlQuoteElement>,
{
}

// #[cfg(feature = "HtmlScriptElement")]
pub trait HtmlScriptElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlScriptElement>>
{
}

// #[cfg(feature = "HtmlScriptElement")]
impl<State, Action, T> HtmlScriptElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlScriptElement>,
{
}

// #[cfg(feature = "HtmlSelectElement")]
pub trait HtmlSelectElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlSelectElement>>
{
}

// #[cfg(feature = "HtmlSelectElement")]
impl<State, Action, T> HtmlSelectElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlSelectElement>,
{
}

// #[cfg(feature = "HtmlSlotElement")]
pub trait HtmlSlotElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlSlotElement>>
{
}

// #[cfg(feature = "HtmlSlotElement")]
impl<State, Action, T> HtmlSlotElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlSlotElement>,
{
}

// #[cfg(feature = "HtmlSourceElement")]
pub trait HtmlSourceElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlSourceElement>>
{
}

// #[cfg(feature = "HtmlSourceElement")]
impl<State, Action, T> HtmlSourceElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlSourceElement>,
{
}

// #[cfg(feature = "HtmlSpanElement")]
pub trait HtmlSpanElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlSpanElement>>
{
}

// #[cfg(feature = "HtmlSpanElement")]
impl<State, Action, T> HtmlSpanElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlSpanElement>,
{
}

// #[cfg(feature = "HtmlStyleElement")]
// pub trait HtmlStyleElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlStyleElement>>
// {
// }

// #[cfg(feature = "HtmlStyleElement")]
// impl<State, Action, T> HtmlStyleElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlStyleElement>,
// {
// }

// #[cfg(feature = "HtmlTableCaptionElement")]
pub trait HtmlTableCaptionElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTableCaptionElement>>
{
}

// #[cfg(feature = "HtmlTableCaptionElement")]
impl<State, Action, T> HtmlTableCaptionElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTableCaptionElement>,
{
}

// #[cfg(feature = "HtmlTableCellElement")]
pub trait HtmlTableCellElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTableCellElement>>
{
}

// #[cfg(feature = "HtmlTableCellElement")]
impl<State, Action, T> HtmlTableCellElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTableCellElement>,
{
}

// #[cfg(feature = "HtmlTableColElement")]
pub trait HtmlTableColElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTableColElement>>
{
}

// #[cfg(feature = "HtmlTableColElement")]
impl<State, Action, T> HtmlTableColElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTableColElement>,
{
}

// #[cfg(feature = "HtmlTableElement")]
pub trait HtmlTableElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTableElement>>
{
}

// #[cfg(feature = "HtmlTableElement")]
impl<State, Action, T> HtmlTableElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTableElement>,
{
}

// #[cfg(feature = "HtmlTableRowElement")]
pub trait HtmlTableRowElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTableRowElement>>
{
}

// #[cfg(feature = "HtmlTableRowElement")]
impl<State, Action, T> HtmlTableRowElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTableRowElement>,
{
}

// #[cfg(feature = "HtmlTableSectionElement")]
pub trait HtmlTableSectionElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTableSectionElement>>
{
}

// #[cfg(feature = "HtmlTableSectionElement")]
impl<State, Action, T> HtmlTableSectionElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTableSectionElement>,
{
}

// #[cfg(feature = "HtmlTemplateElement")]
pub trait HtmlTemplateElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTemplateElement>>
{
}

// #[cfg(feature = "HtmlTemplateElement")]
impl<State, Action, T> HtmlTemplateElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTemplateElement>,
{
}

// #[cfg(feature = "HtmlTimeElement")]
pub trait HtmlTimeElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTimeElement>>
{
}

// #[cfg(feature = "HtmlTimeElement")]
impl<State, Action, T> HtmlTimeElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTimeElement>,
{
}

// #[cfg(feature = "HtmlTextAreaElement")]
pub trait HtmlTextAreaElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTextAreaElement>>
{
}

// #[cfg(feature = "HtmlTextAreaElement")]
impl<State, Action, T> HtmlTextAreaElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTextAreaElement>,
{
}

// #[cfg(feature = "HtmlTitleElement")]
// pub trait HtmlTitleElement<State, Action = ()>:
//     HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTitleElement>>
// {
// }

// #[cfg(feature = "HtmlTitleElement")]
// impl<State, Action, T> HtmlTitleElement<State, Action> for T
// where
//     T: HtmlElement<State, Action>,
//     T::DomNode: AsRef<web_sys::HtmlTitleElement>,
// {
// }

// #[cfg(feature = "HtmlTrackElement")]
pub trait HtmlTrackElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlTrackElement>>
{
}

// #[cfg(feature = "HtmlTrackElement")]
impl<State, Action, T> HtmlTrackElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlTrackElement>,
{
}

// #[cfg(feature = "HtmlUListElement")]
pub trait HtmlUListElement<State, Action = ()>:
    HtmlElement<State, Action, DomNode: AsRef<web_sys::HtmlUListElement>>
{
}

// #[cfg(feature = "HtmlUListElement")]
impl<State, Action, T> HtmlUListElement<State, Action> for T
where
    T: HtmlElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlUListElement>,
{
}

// #[cfg(feature = "HtmlVideoElement")]
pub trait HtmlVideoElement<State, Action = ()>:
    HtmlMediaElement<State, Action, DomNode: AsRef<web_sys::HtmlVideoElement>>
{
}

// #[cfg(feature = "HtmlVideoElement")]
impl<State, Action, T> HtmlVideoElement<State, Action> for T
where
    T: HtmlMediaElement<State, Action>,
    T::DomNode: AsRef<web_sys::HtmlVideoElement>,
{
}

// #[cfg(feature = "SvgaElement")]
pub trait SvgaElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgaElement>>
{
}

// #[cfg(feature = "SvgaElement")]
impl<State, Action, T> SvgaElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgaElement>,
{
}

// #[cfg(feature = "SvgAnimateElement")]
pub trait SvgAnimateElement<State, Action = ()>:
    SvgAnimationElement<State, Action, DomNode: AsRef<web_sys::SvgAnimateElement>>
{
}

// #[cfg(feature = "SvgAnimateElement")]
impl<State, Action, T> SvgAnimateElement<State, Action> for T
where
    T: SvgAnimationElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgAnimateElement>,
{
}

// #[cfg(feature = "SvgAnimateMotionElement")]
pub trait SvgAnimateMotionElement<State, Action = ()>:
    SvgAnimationElement<State, Action, DomNode: AsRef<web_sys::SvgAnimateMotionElement>>
{
}

// #[cfg(feature = "SvgAnimateMotionElement")]
impl<State, Action, T> SvgAnimateMotionElement<State, Action> for T
where
    T: SvgAnimationElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgAnimateMotionElement>,
{
}

// #[cfg(feature = "SvgAnimateTransformElement")]
pub trait SvgAnimateTransformElement<State, Action = ()>:
    SvgAnimationElement<State, Action, DomNode: AsRef<web_sys::SvgAnimateTransformElement>>
{
}

// #[cfg(feature = "SvgAnimateTransformElement")]
impl<State, Action, T> SvgAnimateTransformElement<State, Action> for T
where
    T: SvgAnimationElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgAnimateTransformElement>,
{
}

// #[cfg(feature = "SvgAnimationElement")]
pub trait SvgAnimationElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgAnimationElement>>
{
}

// #[cfg(feature = "SvgAnimationElement")]
impl<State, Action, T> SvgAnimationElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgAnimationElement>,
{
}

// #[cfg(feature = "SvgCircleElement")]
pub trait SvgCircleElement<State, Action = ()>:
    SvgGeometryElement<State, Action, DomNode: AsRef<web_sys::SvgCircleElement>>
{
}

// #[cfg(feature = "SvgCircleElement")]
impl<State, Action, T> SvgCircleElement<State, Action> for T
where
    T: SvgGeometryElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgCircleElement>,
{
}

// #[cfg(feature = "SvgClipPathElement")]
pub trait SvgClipPathElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgClipPathElement>>
{
}

// #[cfg(feature = "SvgClipPathElement")]
impl<State, Action, T> SvgClipPathElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgClipPathElement>,
{
}

// #[cfg(feature = "SvgComponentTransferFunctionElement")]
pub trait SvgComponentTransferFunctionElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgComponentTransferFunctionElement>>
{
}

// #[cfg(feature = "SvgComponentTransferFunctionElement")]
impl<State, Action, T> SvgComponentTransferFunctionElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgComponentTransferFunctionElement>,
{
}

// #[cfg(feature = "SvgDefsElement")]
pub trait SvgDefsElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgDefsElement>>
{
}

// #[cfg(feature = "SvgDefsElement")]
impl<State, Action, T> SvgDefsElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgDefsElement>,
{
}

// #[cfg(feature = "SvgDescElement")]
pub trait SvgDescElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgDescElement>>
{
}

// #[cfg(feature = "SvgDescElement")]
impl<State, Action, T> SvgDescElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgDescElement>,
{
}

// #[cfg(feature = "SvgElement")]
pub trait SvgElement<State, Action = ()>:
    Element<State, Action, Props: WithStyle, DomNode: AsRef<web_sys::SvgElement>>
{
    /// Set a style attribute
    fn style(self, style: impl IntoStyles) -> Style<Self, State, Action> {
        let mut styles = vec![];
        style.into_styles(&mut styles);
        Style::new(self, styles)
    }
}

// #[cfg(feature = "SvgElement")]
impl<State, Action, T> SvgElement<State, Action> for T
where
    T: Element<State, Action>,
    T::DomNode: AsRef<web_sys::SvgElement>,
    T::Props: WithStyle,
{
}

// #[cfg(feature = "SvgEllipseElement")]
pub trait SvgEllipseElement<State, Action = ()>:
    SvgGeometryElement<State, Action, DomNode: AsRef<web_sys::SvgEllipseElement>>
{
}

// #[cfg(feature = "SvgEllipseElement")]
impl<State, Action, T> SvgEllipseElement<State, Action> for T
where
    T: SvgGeometryElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgEllipseElement>,
{
}

// #[cfg(feature = "SvgfeBlendElement")]
pub trait SvgfeBlendElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeBlendElement>>
{
}

// #[cfg(feature = "SvgfeBlendElement")]
impl<State, Action, T> SvgfeBlendElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeBlendElement>,
{
}

// #[cfg(feature = "SvgfeColorMatrixElement")]
pub trait SvgfeColorMatrixElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeColorMatrixElement>>
{
}

// #[cfg(feature = "SvgfeColorMatrixElement")]
impl<State, Action, T> SvgfeColorMatrixElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeColorMatrixElement>,
{
}

// #[cfg(feature = "SvgfeComponentTransferElement")]
pub trait SvgfeComponentTransferElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeComponentTransferElement>>
{
}

// #[cfg(feature = "SvgfeComponentTransferElement")]
impl<State, Action, T> SvgfeComponentTransferElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeComponentTransferElement>,
{
}

// #[cfg(feature = "SvgfeCompositeElement")]
pub trait SvgfeCompositeElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeCompositeElement>>
{
}

// #[cfg(feature = "SvgfeCompositeElement")]
impl<State, Action, T> SvgfeCompositeElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeCompositeElement>,
{
}

// #[cfg(feature = "SvgfeConvolveMatrixElement")]
pub trait SvgfeConvolveMatrixElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeConvolveMatrixElement>>
{
}

// #[cfg(feature = "SvgfeConvolveMatrixElement")]
impl<State, Action, T> SvgfeConvolveMatrixElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeConvolveMatrixElement>,
{
}

// #[cfg(feature = "SvgfeDiffuseLightingElement")]
pub trait SvgfeDiffuseLightingElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeDiffuseLightingElement>>
{
}

// #[cfg(feature = "SvgfeDiffuseLightingElement")]
impl<State, Action, T> SvgfeDiffuseLightingElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeDiffuseLightingElement>,
{
}

// #[cfg(feature = "SvgfeDisplacementMapElement")]
pub trait SvgfeDisplacementMapElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeDisplacementMapElement>>
{
}

// #[cfg(feature = "SvgfeDisplacementMapElement")]
impl<State, Action, T> SvgfeDisplacementMapElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeDisplacementMapElement>,
{
}

// #[cfg(feature = "SvgfeDistantLightElement")]
pub trait SvgfeDistantLightElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeDistantLightElement>>
{
}

// #[cfg(feature = "SvgfeDistantLightElement")]
impl<State, Action, T> SvgfeDistantLightElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeDistantLightElement>,
{
}

// #[cfg(feature = "SvgfeDropShadowElement")]
pub trait SvgfeDropShadowElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeDropShadowElement>>
{
}

// #[cfg(feature = "SvgfeDropShadowElement")]
impl<State, Action, T> SvgfeDropShadowElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeDropShadowElement>,
{
}

// #[cfg(feature = "SvgfeFloodElement")]
pub trait SvgfeFloodElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeFloodElement>>
{
}

// #[cfg(feature = "SvgfeFloodElement")]
impl<State, Action, T> SvgfeFloodElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeFloodElement>,
{
}

// #[cfg(feature = "SvgfeFuncAElement")]
pub trait SvgfeFuncAElement<State, Action = ()>:
    SvgComponentTransferFunctionElement<State, Action, DomNode: AsRef<web_sys::SvgfeFuncAElement>>
{
}

// #[cfg(feature = "SvgfeFuncAElement")]
impl<State, Action, T> SvgfeFuncAElement<State, Action> for T
where
    T: SvgComponentTransferFunctionElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeFuncAElement>,
{
}

// #[cfg(feature = "SvgfeFuncBElement")]
pub trait SvgfeFuncBElement<State, Action = ()>:
    SvgComponentTransferFunctionElement<State, Action, DomNode: AsRef<web_sys::SvgfeFuncBElement>>
{
}

// #[cfg(feature = "SvgfeFuncBElement")]
impl<State, Action, T> SvgfeFuncBElement<State, Action> for T
where
    T: SvgComponentTransferFunctionElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeFuncBElement>,
{
}

// #[cfg(feature = "SvgfeFuncGElement")]
pub trait SvgfeFuncGElement<State, Action = ()>:
    SvgComponentTransferFunctionElement<State, Action, DomNode: AsRef<web_sys::SvgfeFuncGElement>>
{
}

// #[cfg(feature = "SvgfeFuncGElement")]
impl<State, Action, T> SvgfeFuncGElement<State, Action> for T
where
    T: SvgComponentTransferFunctionElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeFuncGElement>,
{
}

// #[cfg(feature = "SvgfeFuncRElement")]
pub trait SvgfeFuncRElement<State, Action = ()>:
    SvgComponentTransferFunctionElement<State, Action, DomNode: AsRef<web_sys::SvgfeFuncRElement>>
{
}

// #[cfg(feature = "SvgfeFuncRElement")]
impl<State, Action, T> SvgfeFuncRElement<State, Action> for T
where
    T: SvgComponentTransferFunctionElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeFuncRElement>,
{
}

// #[cfg(feature = "SvgfeGaussianBlurElement")]
pub trait SvgfeGaussianBlurElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeGaussianBlurElement>>
{
}

// #[cfg(feature = "SvgfeGaussianBlurElement")]
impl<State, Action, T> SvgfeGaussianBlurElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeGaussianBlurElement>,
{
}

// #[cfg(feature = "SvgfeImageElement")]
pub trait SvgfeImageElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeImageElement>>
{
}

// #[cfg(feature = "SvgfeImageElement")]
impl<State, Action, T> SvgfeImageElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeImageElement>,
{
}

// #[cfg(feature = "SvgfeMergeElement")]
pub trait SvgfeMergeElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeMergeElement>>
{
}

// #[cfg(feature = "SvgfeMergeElement")]
impl<State, Action, T> SvgfeMergeElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeMergeElement>,
{
}

// #[cfg(feature = "SvgfeMergeNodeElement")]
pub trait SvgfeMergeNodeElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeMergeNodeElement>>
{
}

// #[cfg(feature = "SvgfeMergeNodeElement")]
impl<State, Action, T> SvgfeMergeNodeElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeMergeNodeElement>,
{
}

// #[cfg(feature = "SvgfeMorphologyElement")]
pub trait SvgfeMorphologyElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeMorphologyElement>>
{
}

// #[cfg(feature = "SvgfeMorphologyElement")]
impl<State, Action, T> SvgfeMorphologyElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeMorphologyElement>,
{
}

// #[cfg(feature = "SvgfeOffsetElement")]
pub trait SvgfeOffsetElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeOffsetElement>>
{
}

// #[cfg(feature = "SvgfeOffsetElement")]
impl<State, Action, T> SvgfeOffsetElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeOffsetElement>,
{
}

// #[cfg(feature = "SvgfePointLightElement")]
pub trait SvgfePointLightElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfePointLightElement>>
{
}

// #[cfg(feature = "SvgfePointLightElement")]
impl<State, Action, T> SvgfePointLightElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfePointLightElement>,
{
}

// #[cfg(feature = "SvgfeSpecularLightingElement")]
pub trait SvgfeSpecularLightingElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeSpecularLightingElement>>
{
}

// #[cfg(feature = "SvgfeSpecularLightingElement")]
impl<State, Action, T> SvgfeSpecularLightingElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeSpecularLightingElement>,
{
}

// #[cfg(feature = "SvgfeSpotLightElement")]
pub trait SvgfeSpotLightElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeSpotLightElement>>
{
}

// #[cfg(feature = "SvgfeSpotLightElement")]
impl<State, Action, T> SvgfeSpotLightElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeSpotLightElement>,
{
}

// #[cfg(feature = "SvgfeTileElement")]
pub trait SvgfeTileElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeTileElement>>
{
}

// #[cfg(feature = "SvgfeTileElement")]
impl<State, Action, T> SvgfeTileElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeTileElement>,
{
}

// #[cfg(feature = "SvgfeTurbulenceElement")]
pub trait SvgfeTurbulenceElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgfeTurbulenceElement>>
{
}

// #[cfg(feature = "SvgfeTurbulenceElement")]
impl<State, Action, T> SvgfeTurbulenceElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgfeTurbulenceElement>,
{
}

// #[cfg(feature = "SvgFilterElement")]
pub trait SvgFilterElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgFilterElement>>
{
}

// #[cfg(feature = "SvgFilterElement")]
impl<State, Action, T> SvgFilterElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgFilterElement>,
{
}

// #[cfg(feature = "SvgForeignObjectElement")]
pub trait SvgForeignObjectElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgForeignObjectElement>>
{
}

// #[cfg(feature = "SvgForeignObjectElement")]
impl<State, Action, T> SvgForeignObjectElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgForeignObjectElement>,
{
}

// #[cfg(feature = "SvggElement")]
pub trait SvggElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvggElement>>
{
}

// #[cfg(feature = "SvggElement")]
impl<State, Action, T> SvggElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvggElement>,
{
}

// #[cfg(feature = "SvgGeometryElement")]
pub trait SvgGeometryElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgGeometryElement>>
{
}

// #[cfg(feature = "SvgGeometryElement")]
impl<State, Action, T> SvgGeometryElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgGeometryElement>,
{
}

// #[cfg(feature = "SvgGradientElement")]
pub trait SvgGradientElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgGradientElement>>
{
}

// #[cfg(feature = "SvgGradientElement")]
impl<State, Action, T> SvgGradientElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgGradientElement>,
{
}

// #[cfg(feature = "SvgGraphicsElement")]
pub trait SvgGraphicsElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgGraphicsElement>>
{
}

// #[cfg(feature = "SvgGraphicsElement")]
impl<State, Action, T> SvgGraphicsElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgGraphicsElement>,
{
}

// #[cfg(feature = "SvgImageElement")]
pub trait SvgImageElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgImageElement>>
{
}

// #[cfg(feature = "SvgImageElement")]
impl<State, Action, T> SvgImageElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgImageElement>,
{
}

// #[cfg(feature = "SvgLinearGradientElement")]
pub trait SvgLinearGradientElement<State, Action = ()>:
    SvgGradientElement<State, Action, DomNode: AsRef<web_sys::SvgLinearGradientElement>>
{
}

// #[cfg(feature = "SvgLinearGradientElement")]
impl<State, Action, T> SvgLinearGradientElement<State, Action> for T
where
    T: SvgGradientElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgLinearGradientElement>,
{
}

// #[cfg(feature = "SvgLineElement")]
pub trait SvgLineElement<State, Action = ()>:
    SvgGeometryElement<State, Action, DomNode: AsRef<web_sys::SvgLineElement>>
{
}

// #[cfg(feature = "SvgLineElement")]
impl<State, Action, T> SvgLineElement<State, Action> for T
where
    T: SvgGeometryElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgLineElement>,
{
}

// #[cfg(feature = "SvgMarkerElement")]
pub trait SvgMarkerElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgMarkerElement>>
{
}

// #[cfg(feature = "SvgMarkerElement")]
impl<State, Action, T> SvgMarkerElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgMarkerElement>,
{
}

// #[cfg(feature = "SvgMaskElement")]
pub trait SvgMaskElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgMaskElement>>
{
}

// #[cfg(feature = "SvgMaskElement")]
impl<State, Action, T> SvgMaskElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgMaskElement>,
{
}

// #[cfg(feature = "SvgMetadataElement")]
pub trait SvgMetadataElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgMetadataElement>>
{
}

// #[cfg(feature = "SvgMetadataElement")]
impl<State, Action, T> SvgMetadataElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgMetadataElement>,
{
}

// #[cfg(feature = "SvgmPathElement")]
pub trait SvgmPathElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgmPathElement>>
{
}

// #[cfg(feature = "SvgmPathElement")]
impl<State, Action, T> SvgmPathElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgmPathElement>,
{
}

// #[cfg(feature = "SvgPathElement")]
pub trait SvgPathElement<State, Action = ()>:
    SvgGeometryElement<State, Action, DomNode: AsRef<web_sys::SvgPathElement>>
{
}

// #[cfg(feature = "SvgPathElement")]
impl<State, Action, T> SvgPathElement<State, Action> for T
where
    T: SvgGeometryElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgPathElement>,
{
}

// #[cfg(feature = "SvgPatternElement")]
pub trait SvgPatternElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgPatternElement>>
{
}

// #[cfg(feature = "SvgPatternElement")]
impl<State, Action, T> SvgPatternElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgPatternElement>,
{
}

// #[cfg(feature = "SvgPolygonElement")]
pub trait SvgPolygonElement<State, Action = ()>:
    SvgGeometryElement<State, Action, DomNode: AsRef<web_sys::SvgPolygonElement>>
{
}

// #[cfg(feature = "SvgPolygonElement")]
impl<State, Action, T> SvgPolygonElement<State, Action> for T
where
    T: SvgGeometryElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgPolygonElement>,
{
}

// #[cfg(feature = "SvgPolylineElement")]
pub trait SvgPolylineElement<State, Action = ()>:
    SvgGeometryElement<State, Action, DomNode: AsRef<web_sys::SvgPolylineElement>>
{
}

// #[cfg(feature = "SvgPolylineElement")]
impl<State, Action, T> SvgPolylineElement<State, Action> for T
where
    T: SvgGeometryElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgPolylineElement>,
{
}

// #[cfg(feature = "SvgRectElement")]
pub trait SvgRectElement<State, Action = ()>:
    SvgGeometryElement<State, Action, DomNode: AsRef<web_sys::SvgRectElement>>
{
}

// #[cfg(feature = "SvgRectElement")]
impl<State, Action, T> SvgRectElement<State, Action> for T
where
    T: SvgGeometryElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgRectElement>,
{
}

// #[cfg(feature = "SvgScriptElement")]
pub trait SvgScriptElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgScriptElement>>
{
}

// #[cfg(feature = "SvgScriptElement")]
impl<State, Action, T> SvgScriptElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgScriptElement>,
{
}

// #[cfg(feature = "SvgSetElement")]
pub trait SvgSetElement<State, Action = ()>:
    SvgAnimationElement<State, Action, DomNode: AsRef<web_sys::SvgSetElement>>
{
}

// #[cfg(feature = "SvgSetElement")]
impl<State, Action, T> SvgSetElement<State, Action> for T
where
    T: SvgAnimationElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgSetElement>,
{
}

// #[cfg(feature = "SvgStopElement")]
pub trait SvgStopElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgStopElement>>
{
}

// #[cfg(feature = "SvgStopElement")]
impl<State, Action, T> SvgStopElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgStopElement>,
{
}

// #[cfg(feature = "SvgStyleElement")]
pub trait SvgStyleElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgStyleElement>>
{
}

// #[cfg(feature = "SvgStyleElement")]
impl<State, Action, T> SvgStyleElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgStyleElement>,
{
}

// #[cfg(feature = "SvgSwitchElement")]
pub trait SvgSwitchElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgSwitchElement>>
{
}

// #[cfg(feature = "SvgSwitchElement")]
impl<State, Action, T> SvgSwitchElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgSwitchElement>,
{
}

// #[cfg(feature = "SvgSymbolElement")]
pub trait SvgSymbolElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgSymbolElement>>
{
}

// #[cfg(feature = "SvgSymbolElement")]
impl<State, Action, T> SvgSymbolElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgSymbolElement>,
{
}

// #[cfg(feature = "SvgTextContentElement")]
pub trait SvgTextContentElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgTextContentElement>>
{
}

// #[cfg(feature = "SvgTextContentElement")]
impl<State, Action, T> SvgTextContentElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgTextContentElement>,
{
}

// #[cfg(feature = "SvgTextPathElement")]
pub trait SvgTextPathElement<State, Action = ()>:
    SvgTextContentElement<State, Action, DomNode: AsRef<web_sys::SvgTextPathElement>>
{
}

// #[cfg(feature = "SvgTextPathElement")]
impl<State, Action, T> SvgTextPathElement<State, Action> for T
where
    T: SvgTextContentElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgTextPathElement>,
{
}

// #[cfg(feature = "SvgTextPositioningElement")]
pub trait SvgTextPositioningElement<State, Action = ()>:
    SvgTextContentElement<State, Action, DomNode: AsRef<web_sys::SvgTextPositioningElement>>
{
}

// #[cfg(feature = "SvgTextPositioningElement")]
impl<State, Action, T> SvgTextPositioningElement<State, Action> for T
where
    T: SvgTextContentElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgTextPositioningElement>,
{
}

// #[cfg(feature = "SvgtSpanElement")]
pub trait SvgtSpanElement<State, Action = ()>:
    SvgTextPositioningElement<State, Action, DomNode: AsRef<web_sys::SvgtSpanElement>>
{
}

// #[cfg(feature = "SvgtSpanElement")]
impl<State, Action, T> SvgtSpanElement<State, Action> for T
where
    T: SvgTextPositioningElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgtSpanElement>,
{
}

// #[cfg(feature = "SvgViewElement")]
pub trait SvgViewElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgViewElement>>
{
}

// #[cfg(feature = "SvgViewElement")]
impl<State, Action, T> SvgViewElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgViewElement>,
{
}

// #[cfg(feature = "SvgRadialGradientElement")]
pub trait SvgRadialGradientElement<State, Action = ()>:
    SvgGradientElement<State, Action, DomNode: AsRef<web_sys::SvgRadialGradientElement>>
{
}

// #[cfg(feature = "SvgRadialGradientElement")]
impl<State, Action, T> SvgRadialGradientElement<State, Action> for T
where
    T: SvgGradientElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgRadialGradientElement>,
{
}

// #[cfg(feature = "SvgsvgElement")]
pub trait SvgsvgElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgsvgElement>>
{
}

// #[cfg(feature = "SvgsvgElement")]
impl<State, Action, T> SvgsvgElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgsvgElement>,
{
}

// #[cfg(feature = "SvgTextElement")]
pub trait SvgTextElement<State, Action = ()>:
    SvgTextPositioningElement<State, Action, DomNode: AsRef<web_sys::SvgTextElement>>
{
}

// #[cfg(feature = "SvgTextElement")]
impl<State, Action, T> SvgTextElement<State, Action> for T
where
    T: SvgTextPositioningElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgTextElement>,
{
}

// #[cfg(feature = "SvgTitleElement")]
pub trait SvgTitleElement<State, Action = ()>:
    SvgElement<State, Action, DomNode: AsRef<web_sys::SvgTitleElement>>
{
}

// #[cfg(feature = "SvgTitleElement")]
impl<State, Action, T> SvgTitleElement<State, Action> for T
where
    T: SvgElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgTitleElement>,
{
}

// #[cfg(feature = "SvgUseElement")]
pub trait SvgUseElement<State, Action = ()>:
    SvgGraphicsElement<State, Action, DomNode: AsRef<web_sys::SvgUseElement>>
{
}

// #[cfg(feature = "SvgUseElement")]
impl<State, Action, T> SvgUseElement<State, Action> for T
where
    T: SvgGraphicsElement<State, Action>,
    T::DomNode: AsRef<web_sys::SvgUseElement>,
{
}
