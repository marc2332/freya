#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(250.0, 60.0)
            .on_setup(|window| {
                window.set_title("Hello World");
            })
            .on_exit(|window| {
                println!("Window title was {}", window.title());
            })
            .with_window_attributes(|attributes| attributes.with_resizable(false)),
    );
}

fn app() -> Element {
    rsx!(
        label { "Close the window :)" }
    )
}
