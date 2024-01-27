#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    let form = use_form(|data| {
        println!("{data:?}");
    });

    rsx!(
        Input {
            ..form.register("name".to_string())
        },
        Input {
            ..form.register("description".to_string())
        },
        Button {
            ..form.submit(rsx!(
                label {
                    "Submit"
                }
            )),
        }
    )
}
