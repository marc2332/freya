#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use rand::Rng;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let elements = use_state(cx, || Vec::new());

    let add = {
        to_owned![elements];
        move |_| {
            let mut rng = rand::thread_rng();
            elements.with_mut(|elements| {
                elements.push(rng.gen());
            })
        }
    };

    let remove = {
        to_owned![elements];
        move |_| {
            elements.with_mut(|elements| {
                elements.pop();
            })
        }
    };

    render!(rect {
        background: "rgb(225, 200, 150)",
        elements.get().iter().map(|e: &usize| rsx!(
            rect {
                key: "{e}",
                background: "rgb(150, 200, 225)",
                width: "100%",
                label {
                    "Element {e}"
                }
            }
        ))
        Button {
            onclick: add,
            label {
                "Add"
            }
        }
        Button {
            onclick: remove,
            label {
                "Remove"
            }
        }
    })
}
