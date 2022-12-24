#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use tokio::{time::sleep, sync::mpsc::unbounded_channel};

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let enabled = use_state(&cx, || false);  
    let is_enabled = if *enabled.get() { "Yes" } else { "No" };

    render!(
        rect {
            width: "90%",
            height: "85%",
            onclick: move |_| {
                println!("clicked");
                enabled.set(!enabled.get());
            },
            label {
                color: "black",
                "{is_enabled}"
            }
            vec![0,0,0,0,0,0,0,0].iter().enumerate().map(move |(i, _)| {
                println!("{}", i);                       
                rsx! {
                    rect {
                        key: "{i}",
                        label {
                            "{i}"
                        }
                    }
                }
            })
        }
    )
}
