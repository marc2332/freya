#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut open = use_state(|| false);

    rect().expanded().center().child(
        Attached::new(Button::new().on_press(move |_| open.toggle()).child("Menu"))
            .bottom()
            .maybe_child(open().then(|| {
                Menu::new()
                    .child(MenuButton::new().child("Profile"))
                    .child(MenuButton::new().child("Settings"))
                    .child(MenuButton::new().child("Log out"))
            })),
    )
}
