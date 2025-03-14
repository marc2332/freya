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
fn app() -> Element {
    let image = use_camera(CameraSettings::default());

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            padding: "50",
            if let Some(err) = &*image.error().read() {
                label {
                    color: "black",
                    "{err}"
                }
            } else {
                image {
                    width: "100%",
                    height: "100%",
                    aspect_ratio: "none",
                    reference: image.attribute(),
                    image_reference: image.image_attribute()
                }
            }
        }
    )
}
