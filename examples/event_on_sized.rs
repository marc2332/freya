#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;
use torin::prelude::Area;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut size = use_state(Area::default);

    rect()
        .on_sized(move |e: Event<SizedEventData>| size.set(e.area))
        .expanded()
        .center()
        .background((0, 119, 182))
        .child(format!("{:?}", size.read()))
}
