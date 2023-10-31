use xilem_html::{document_body, elements as el, interfaces::*, App};

pub fn main() {
    console_error_panic_hook::set_once();
    App::new(false, |is_playing| {
        el::div((
            el::video(())
                .attr(
                    "src",
                    "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                )
                .play(*is_playing)
                .on_ended(|is_playing, _| *is_playing = false),
            el::button("Play/Pause video").on_click(|is_playing: &mut bool, _| {
                *is_playing = !*is_playing;
            }),
        ))
    })
    .run(&document_body());
}
