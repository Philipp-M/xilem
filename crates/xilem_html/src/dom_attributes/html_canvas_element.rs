use super::{create_dom_attribute_view, DomAttr};
use crate::ChangeFlags;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlCanvasElementAttr {
    Width(u32),
    Height(u32),
}

create_dom_attribute_view!(width, u32, HtmlCanvasElement);
create_dom_attribute_view!(height, u32, HtmlCanvasElement);

// TODO there may be less boilerplate heavy (but still flexible and non-mentally challenging/complex ways to express the stuff below...)

pub(crate) fn canvas_element_build_extra(el: &web_sys::Element, attr: &DomAttr) {
    match attr {
        DomAttr::HtmlCanvasElement(HtmlCanvasElementAttr::Width(width)) => {
            let el = el.dyn_ref::<web_sys::HtmlCanvasElement>().unwrap_throw();
            el.set_width(*width);
        }
        DomAttr::HtmlCanvasElement(HtmlCanvasElementAttr::Height(height)) => {
            let el = el.dyn_ref::<web_sys::HtmlCanvasElement>().unwrap_throw();
            el.set_height(*height);
        }
        _ => unreachable!(),
    }
}

pub(crate) fn canvas_element_rebuild_extra(
    el: &web_sys::Element,
    old: &DomAttr,
    new: &DomAttr,
) -> ChangeFlags {
    match (old, new) {
        (
            DomAttr::HtmlCanvasElement(HtmlCanvasElementAttr::Width(_old_width)),
            DomAttr::HtmlCanvasElement(HtmlCanvasElementAttr::Width(new_width)),
        ) => {
            let el = el.dyn_ref::<web_sys::HtmlCanvasElement>().unwrap_throw();
            el.set_width(*new_width);
            ChangeFlags::OTHER_CHANGE
        }
        (
            DomAttr::HtmlCanvasElement(HtmlCanvasElementAttr::Height(_old_height)),
            DomAttr::HtmlCanvasElement(HtmlCanvasElementAttr::Height(new_height)),
        ) => {
            let el = el.dyn_ref::<web_sys::HtmlCanvasElement>().unwrap_throw();
            el.set_height(*new_height);
            ChangeFlags::OTHER_CHANGE
        }
        _ => ChangeFlags::empty(),
    }
}
