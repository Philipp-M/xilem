use super::create_dom_attribute_view;
use crate::ChangeFlags;

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlCanvasElementAttr {
    Width(u32),
    Height(u32),
}

create_dom_attribute_view!(width, u32, HtmlCanvasElement);
create_dom_attribute_view!(height, u32, HtmlCanvasElement);

// TODO there may be less boilerplate heavy (but still flexible and non-mentally challenging/complex ways to express the stuff below...)

pub(crate) fn build_dom_attribute(el: &web_sys::HtmlCanvasElement, attr: &HtmlCanvasElementAttr) {
    match attr {
        HtmlCanvasElementAttr::Width(width) => el.set_width(*width),
        HtmlCanvasElementAttr::Height(height) => el.set_height(*height),
    }
}

pub(crate) fn rebuild_dom_attribute(
    el: &web_sys::HtmlCanvasElement,
    old: &HtmlCanvasElementAttr,
    new: &HtmlCanvasElementAttr,
) -> ChangeFlags {
    match (old, new) {
        (HtmlCanvasElementAttr::Width(old_width), HtmlCanvasElementAttr::Width(new_width))
            if old_width != new_width =>
        {
            el.set_width(*new_width);
            ChangeFlags::OTHER_CHANGE
        }
        (HtmlCanvasElementAttr::Height(old_height), HtmlCanvasElementAttr::Height(new_height))
            if old_height != new_height =>
        {
            el.set_height(*new_height);
            ChangeFlags::OTHER_CHANGE
        }
        _ => ChangeFlags::empty(),
    }
}
