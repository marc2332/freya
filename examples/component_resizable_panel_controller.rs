#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut controller = use_state(|| ResizableContext {
        panels: vec![],
        direction: Direction::Vertical,
    });

    let on_reset = move |_| {
        controller.write().reset();
    };

    rect()
        .expanded()
        .padding(12.)
        .spacing(12.)
        .child(
            rect()
                .horizontal()
                .spacing(10.)
                .cross_align(Alignment::Center)
                .child(Button::new().on_press(on_reset).child("Reset sizes"))
                .child("Resize panels, then click Reset"),
        )
        .child(
            ResizableContainer::new()
                .controller(controller)
                .panel(
                    ResizablePanel::new(50.0).min_size(10.).child(
                        rect()
                            .expanded()
                            .center()
                            .background((100, 150, 200))
                            .child("Panel 1"),
                    ),
                )
                .panel(
                    ResizablePanel::new(50.0).min_size(10.).child(
                        rect()
                            .expanded()
                            .center()
                            .background((150, 200, 100))
                            .child("Panel 2"),
                    ),
                ),
        )
}
