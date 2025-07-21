#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::str::FromStr;

use dioxus::CapturedError;
use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            ErrorBoundary {
                handle_error: |_| rsx!( label { "An error occured inside this component!" }),
                ErrorThrower { }
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            spacing: "8",
            ErrorThrower { }
        }
    )
}

#[component]
fn ErrorThrower() -> Element {
    let onclick = |_| {
        throw_error(CapturedError::from_str("uhh!").unwrap());
    };

    rsx!(
        Button {
            onclick,
            label {
                "Throw an error!"
            }
        }
    )
}
