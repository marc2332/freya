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

    rect().expanded().main_align(Alignment::End).child(
        Attached::new(
            Button::new()
                .on_press(move |e: Event<PressEventData>| {
                    ContextMenu::open_from_event(
                        &e,
                        Menu::new()
                            .child(MenuButton::new().child("Profile"))
                            .child(MenuButton::new().child("Settings"))
                            .child(MenuButton::new().child("Log out")),
                    );
                })
                .child("Menu"),
        )
        .bottom()
        .maybe_child(open().then(|| {
            Menu::new()
                .child(MenuButton::new().child("Profile"))
                .child(MenuButton::new().child("Settings"))
                .child(MenuButton::new().child("Log out"))
        })),
    )
}
