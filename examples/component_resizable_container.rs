#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let mut panels = use_state(|| 5);

    ResizableContainer::new()
        .panel(
            ResizablePanel::new(50.).child(
                rect()
                    .expanded()
                    .center()
                    .child("Panel 1")
                    .child(
                        Button::new()
                            .on_press(move |_| *panels.write() += 1)
                            .child("New"),
                    )
                    .child(
                        Button::new()
                            .on_press(move |_| *panels.write() -= 1)
                            .child("Pop"),
                    ),
            ),
        )
        .panel(
            ResizablePanel::new(50.).child(
                ResizableContainer::new()
                    .direction(Direction::Horizontal)
                    .panels_iter((1..panels()).map(|panel| {
                        ResizablePanel::new(50.)
                            .key(&panel)
                            .order(panel as usize)
                            .initial_size(panel as f32 * 15.)
                            .min_size(panel as f32 * 5.)
                            .child(
                                rect()
                                    .expanded()
                                    .center()
                                    .corner_radius(6.)
                                    .color(Color::WHITE)
                                    .background((15, 163, 242))
                                    .child(format!("Panel {panel}")),
                            )
                    })),
            ),
        )
        .into()
}
