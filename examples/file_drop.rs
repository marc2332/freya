#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

#[derive(PartialEq)]
enum Status {
    Idle,
    Hovering
}

fn app() -> Element {
    let mut path = use_signal::<Option<String>>(|| None);
    let mut status = use_signal(|| Status::Idle);

    let msg = path.read().clone().unwrap_or("Drop a file!".to_string());

    let background = if *status.read() == Status::Hovering {
        "rgb(109, 198, 227)"
    } else {
        "rgb(0, 119, 182)"
    };

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "{background}",
            color: "white",
            onfilehover: move |_| status.set(Status::Hovering),
            onfilehovercancelled: move |_| status.set(Status::Idle),
            onfiledrop: move |e| {
                status.set(Status::Idle);
                path.set(Some(e.file_path.as_ref().unwrap().to_string_lossy().to_string()))
            },
            label {
                font_size: "35",
                "{msg}"
            }
        }
    )
}
