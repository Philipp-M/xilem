use wasm_bindgen::UnwrapThrowExt;

use super::create_dom_attribute_view;
use crate::ChangeFlags;

#[derive(PartialEq, Clone, Debug, PartialOrd)]
pub enum HtmlMediaElementAttr {
    Play(bool),
    PlaybackRate(f64),
}

create_dom_attribute_view!(playbackRate, f64, HtmlMediaElement : {HtmlVideoElement, HtmlAudioElement});
create_dom_attribute_view!(play, bool, HtmlMediaElement : {HtmlVideoElement, HtmlAudioElement});

pub(crate) fn build_dom_attribute(el: &web_sys::HtmlMediaElement, attr: &HtmlMediaElementAttr) {
    match attr {
        HtmlMediaElementAttr::Play(play) => {
            if *play {
                let _ = el.play().unwrap_throw();
            }
            // TODO pause if play false? Would be relevant if autoplay == true
        }
        HtmlMediaElementAttr::PlaybackRate(playback_rate) => {
            el.set_playback_rate(*playback_rate);
        }
    }
}

pub(crate) fn rebuild_dom_attribute(
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
