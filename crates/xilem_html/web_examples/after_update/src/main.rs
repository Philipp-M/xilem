use xilem_html::{document_body, elements as el, interfaces::Element, App};

pub fn main() {
    console_error_panic_hook::set_once();
    App::new(false, |_focus_input| {
        el::div((
            el::input(()).after_update(|focus_input, el| {
                if *focus_input {
                    let _ = el.focus();
                    *focus_input = false;
                }
            }),
            el::button("Focus the input").on_click(|focus_input: &mut bool, _| {
                *focus_input = true;
            }),
        ))
    })
    .run(&document_body());
}
