#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let state = use_state(|| 5);

    rect()
        .expanded()
        .center()
        .horizontal()
        .spacing(6.)
        .child(CoolComponent(state))
        .child(CoolComponent(state))
        .child(CoolComponent(state))
}

#[derive(PartialEq)]
struct CoolComponent(State<i32>);

impl Component for CoolComponent {
    fn render(&self) -> impl IntoElement {
        let mut state = self.0;

        let increase = move |_| {
            *state.write() += 1;
        };

        Button::new()
            .on_press(increase)
            .child(format!("Value: {}", state.read()))
    }
}
