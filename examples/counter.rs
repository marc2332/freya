#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use rand::Rng;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app() -> Element {
    let mut data = use_signal(Vec::<usize>::new);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            Button {
                onclick: move |_| {
                    let l = data.len() - 1;
                    let mut item = data.write().remove(l);
                    data.write().insert(0, item);
                },
                label {
                    "Move"
                }
            }
            Button {
                onclick: move |_| {
                    let mut rng = rand::thread_rng();
                    data.write().push(rng.gen());
                },
                label {
                    "Add"
                }
            }
            for d in data.read().iter() {
                label {
                    key: "{d}",
                    height: "20",
                    "{d}"
                }
            }
        }
    )
}
