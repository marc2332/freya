use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let platform_state = PlatformState::get();

    rect()
        .expanded()
        .center()
        .spacing(8.)
        .child(format!(
            "{:?}",
            platform_state.focused_accessibility_id.read()
        ))
        .child(Button::new().child("Button 1"))
        .child(Button::new().child("Button 2"))
        .into()
}
