#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use reqwest::Url;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(
        ScrollView {
            for i in (0..10) {
                Works { // Swap this for DoesntWork and try to scroll
                    key: "{i}"
                }
            }
        }
    )
}

// This works because both the ScrolLView and this component use the same custom attribute
#[allow(non_snake_case)]
fn Works(cx: Scope) -> Element {
    let (reference, data) = use_node(cx);

    render!(
        rect {
            height: "50",
            margin: "20 0",
            width: "100%",
            background: "black",
            color: "white",
            reference: reference,
            label {
                width: "100%",
                "{data:?}"
            }
        }
    )
}

// This doesn't work because NetworkImage uses the ImageReference custom attribute, which is different from what the ScrollView uses
#[allow(non_snake_case)]
fn DoesntWork(cx: Scope) -> Element {
    let url = "https://rustacean.net/assets/rustacean-flat-happy.png".parse::<Url>().unwrap().into();
    
    render!(
        NetworkImage {
            url: url
        }
    )
}
