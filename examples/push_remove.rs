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
    let elements = use_ref(cx, || Vec::new());

    let add = |_| {
        let mut rng = rand::thread_rng();
        elements.write().push(rng.gen());
    };

    let remove = |_| {
        elements.write().pop();
    };

    render!(
        Button { onclick: add, label { "Add" } }
        Button { onclick: remove, label { "Remove" } }
        elements.read().iter().map(|e: &usize| rsx!(
            rect {
                key: "{e}",
                background: "rgb(150, 200, 225)",
                label {
                    "Element {e}"
                }
            }
        ))
    )
}
