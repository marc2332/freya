#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const SOURCES: [&str; 3] = [
    "https://images.pexels.com/photos/842711/pexels-photo-842711.jpeg",
    "https://images.pexels.com/photos/1402787/pexels-photo-1402787.jpeg",
    "https://images.pexels.com/photos/417074/pexels-photo-417074.jpeg",
];

fn app() -> impl IntoElement {
    let mut index = use_state(|| 0);

    rect()
        .expanded()
        .cross_align(Alignment::Center)
        .main_align(Alignment::Center)
        .spacing(16.)
        .child(
            ImageViewer::new(SOURCES[index()])
                .width(Size::px(600.))
                .height(Size::px(400.)),
        )
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .child(
                    Button::new()
                        .on_press(move |_| {
                            *index.write() = (index() + SOURCES.len() - 1) % SOURCES.len()
                        })
                        .child("Previous"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| *index.write() = (index() + 1) % SOURCES.len())
                        .child("Next"),
                ),
        )
}
