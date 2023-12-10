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
            .build(),
    );
}

fn app(cx: Scope) -> Element {
    render!(
        label { "Close the window :)" }
    )
}
