use wasm_bindgen::UnwrapThrowExt;

use crate::{
    dom_attribute::{create_dom_attribute_view, HtmlMediaElementAttr},
    ChangeFlags,
};

create_dom_attribute_view!(playbackRate, f64, HtmlMediaElement : {HtmlVideoElement, HtmlAudioElement});
create_dom_attribute_view!(play, bool, HtmlMediaElement : {HtmlVideoElement, HtmlAudioElement});

pub(crate) fn media_element_build_extra(
    el: &web_sys::HtmlMediaElement,
    attr: &HtmlMediaElementAttr,
) {
    match attr {
        HtmlMediaElementAttr::Play(play) => {
            if *play {
                let _ = el.play().unwrap_throw();
            }
            // TODO pause if play false? Would be relevant if autoplay == true
        }
        HtmlMediaElementAttr::PlaybackRate(playback_rate) => {
            el.set_playback_rate(*playback_rate);
        } // _ => unreachable!(),
    }
}

pub(crate) fn media_element_rebuild_extra(
    el: &web_sys::HtmlMediaElement,
    old: &HtmlMediaElementAttr,
    new: &HtmlMediaElementAttr,
) -> ChangeFlags {
    match (old, new) {
        (HtmlMediaElementAttr::Play(old_play), HtmlMediaElementAttr::Play(new_play))
            if old_play != new_play =>
        {
            if *new_play {
                let _ = el.play().unwrap_throw();
            } else {
                el.pause().unwrap_throw();
            }
            ChangeFlags::OTHER_CHANGE
        }
        _ => ChangeFlags::empty(),
    }
}
