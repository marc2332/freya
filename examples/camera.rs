#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let (image_reference, camera_error) = use_camera(cx, CameraSettings::default());

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "100",
            shadow: "0 10 150 40 black",
            if let Some(err) = camera_error.get() {
                rsx!(
                    label {
                        color: "black",
                        "{err}"
                    }
                )
            } else {
                rsx!(
                    image {
                        width: "100%",
                        height: "100%",
                        image_reference: image_reference
                    }
                )
            }
        }
    )
}
