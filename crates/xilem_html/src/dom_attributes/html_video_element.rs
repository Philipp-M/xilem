use super::create_dom_attribute_view;
use crate::ChangeFlags;

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlVideoElementAttr {
    Width(u32),
    Height(u32),
}

create_dom_attribute_view!(width, u32, HtmlVideoElement: {});
create_dom_attribute_view!(height, u32, HtmlVideoElement: {});

pub(crate) fn build_dom_attribute(el: &web_sys::HtmlVideoElement, attr: &HtmlVideoElementAttr) {
    match attr {
        HtmlVideoElementAttr::Width(width) => el.set_width(*width),
        HtmlVideoElementAttr::Height(height) => el.set_height(*height),
    }
}

pub(crate) fn rebuild_dom_attribute(
    el: &web_sys::HtmlVideoElement,
    old: &HtmlVideoElementAttr,
    new: &HtmlVideoElementAttr,
) -> ChangeFlags {
    match (old, new) {
        (HtmlVideoElementAttr::Width(old_width), HtmlVideoElementAttr::Width(new_width))
            if old_width != new_width =>
        {
            el.set_width(*new_width);
            ChangeFlags::OTHER_CHANGE
        }
        (HtmlVideoElementAttr::Height(old_height), HtmlVideoElementAttr::Height(new_height))
            if old_height != new_height =>
        {
            el.set_height(*new_height);
            ChangeFlags::OTHER_CHANGE
        }
        _ => ChangeFlags::empty(),
    }
}
