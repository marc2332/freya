use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let mut show_popup = use_state(|| true);

    rect()
        .maybe_child(show_popup().then(|| {
            Popup::new()
                .on_close_request(move |_| show_popup.set(false))
                .child(PopupTitle::new("Title".to_string()))
                .child(PopupContent::new().child("Hello, World!"))
        }))
        .child(
            Button::new()
                .child("Open")
                .on_press(move |_| show_popup.toggle()),
        )
        .into()
}
