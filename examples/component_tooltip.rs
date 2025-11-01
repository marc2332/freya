#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    rect()
        .expanded()
        .center()
        .spacing(10.)
        .child(TooltipContainer::new(Tooltip::new("Hello!")).child(Button::new().child("Below!")))
        .child(
            TooltipContainer::new(Tooltip::new("Hello!"))
                .position(TooltipPosition::Besides)
                .child(Button::new().child("Besides!")),
        )
        .into()
}
