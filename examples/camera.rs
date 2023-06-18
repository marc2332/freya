#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(feature = "use_camera")]
use freya::prelude::*;

#[cfg(not(feature = "use_camera"))]
fn main() {
    panic!("Run with the 'use_camera' feature");
}

#[cfg(feature = "use_camera")]
fn main() {
    launch(app);
}
#[cfg(feature = "use_camera")]
fn app(cx: Scope) -> Element {
    let (image_reference, camera_error) = use_camera(cx, CameraSettings::default());

    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
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
