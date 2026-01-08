#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .horizontal()
        .child(CoolComponent(2))
        .child(CoolComponent(23))
        .child(CoolComponent(34))
}

#[derive(PartialEq)]
struct CoolComponent(i32);
impl Component for CoolComponent {
    fn render(&self) -> impl IntoElement {
        let mut state = use_state(|| self.0);

        let increase = move |_| {
            *state.write() += 1;
        };

        Button::new()
            .on_press(increase)
            .child(format!("Value: {}", state.read()))
    }
}
