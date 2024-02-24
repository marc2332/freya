#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_height(60.0)
            .with_width(250.0)
            .on_setup(|window| {
                window.set_title("Hello World");
            })
            .on_exit(|window| {
                println!("Window title was {}", window.title());
            })
            .with_window_builder(|builder| builder.with_resizable(false))
            .build(),
    );
}

fn app() -> Element {
    rsx!(
        label { "Close the window :)" }
    )
}
