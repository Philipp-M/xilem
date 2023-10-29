use wasm_bindgen::{JsCast, UnwrapThrowExt};

use super::{
    create_dom_attribute_view,
    html_media_element::{media_element_build_extra, media_element_rebuild_extra},
    DomAttr,
};
use crate::ChangeFlags;

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlVideoElementAttr {
    Width(u32),
    Height(u32),
}

create_dom_attribute_view!(width, u32, HtmlVideoElement);
create_dom_attribute_view!(height, u32, HtmlVideoElement);

pub(crate) fn video_element_build_extra(el: &web_sys::Element, attr: &DomAttr) {
    match attr {
        DomAttr::HtmlMediaElement(attr) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            media_element_build_extra(el, attr)
        }
        DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Width(width)) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            el.set_width(*width);
        }
        DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Height(height)) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            el.set_height(*height);
        }
        _ => unreachable!(),
    }
}

pub(crate) fn video_element_rebuild_extra(
    el: &web_sys::Element,
    old: &DomAttr,
    new: &DomAttr,
) -> ChangeFlags {
    match (old, new) {
        (DomAttr::HtmlMediaElement(old), DomAttr::HtmlMediaElement(new)) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            media_element_rebuild_extra(el, old, new)
        }
        (
            DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Width(_old_width)),
            DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Width(new_width)),
        ) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            el.set_width(*new_width);
            ChangeFlags::OTHER_CHANGE
        }
        (
            DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Height(_old_height)),
            DomAttr::HtmlVideoElement(HtmlVideoElementAttr::Height(new_height)),
        ) => {
            let el = el.dyn_ref::<web_sys::HtmlVideoElement>().unwrap_throw();
            el.set_height(*new_height);
            ChangeFlags::OTHER_CHANGE
        }
        _ => ChangeFlags::empty(),
    }
}
