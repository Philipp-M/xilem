use xilem_html::{document_body, elements as el, elements::Element, App, Event, View, ViewMarker};

#[derive(Default)]
struct AppState {
    clicks: i32,
    class: &'static str,
    text: String,
}

impl AppState {
    fn increment(&mut self) {
        self.clicks += 1;
    }
    fn decrement(&mut self) {
        self.clicks -= 1;
    }
    fn reset(&mut self) {
        self.clicks = 0;
    }
    fn change_class(&mut self) {
        if self.class == "gray" {
            self.class = "green";
        } else {
            self.class = "gray";
        }
    }

    fn change_text(&mut self) {
        if self.text == "test" {
            self.text = "test2".into();
        } else {
            self.text = "test".into();
        }
    }
}

/// You can create functions that generate views.
fn btn<A, F>(label: &'static str, click_fn: F) -> el::Button<AppState, A, &'static str>
where
    A: 'static,
    F: Fn(&mut AppState, Event<web_sys::MouseEvent, web_sys::HtmlButtonElement>) + 'static,
{
    el::button(label).on_click(click_fn)
}

fn app_logic(state: &mut AppState) -> impl View<AppState> + ViewMarker {
    el::div((
        btn("+1 click", |state, _| state.increment()),
        btn("-1 click", |state, _| state.decrement()),
        btn("reset clicks", |state, _| state.reset()),
        btn("a different class", |state, _| state.change_class()),
        btn("change text", |state, _| state.change_text()),
        el::br(()),
        state.text.clone(),
    ))
}

pub fn main() {
    console_error_panic_hook::set_once();
    let app = App::new(AppState::default(), app_logic);
    app.run(&document_body());
}
