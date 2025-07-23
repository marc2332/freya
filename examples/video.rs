#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    path::PathBuf,
    str::FromStr,
};

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Video", (960.0, 560.0));
}

fn app() -> Element {
    rsx!(Video {
        width: "fill",
        height: "fill",
        path: PathBuf::from_str("/home/marc/Downloads/sample_640x360.mp4").unwrap()
    })
}
