use std::{
    cell::RefCell,
    sync::{atomic::AtomicI32, Arc},
    time::Duration,
};

use dioxus::prelude::*;
use node::launch;
use tokio::time::sleep;
mod node;
mod run;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let colors = use_state(&cx, || vec!["green", "blue", "red"]);
    let padding = use_state(&cx, || 60.0);

    use_effect(&cx, (colors, padding), |(colors, padding)| async move {
        sleep(Duration::from_millis(50)).await;
        colors.with_mut(|colors| colors.reverse());
        padding.with_mut(|padding| {
            if *padding < 60.0 {
                *padding += 5.0;
            } else {
                *padding = 20.0;
            }
        });
    });

    let big = colors[0];
    let mid = colors[1];
    let small = colors[2];

    cx.render(rsx! {
        div {
            background: "{big}",
            height: "stretch",
            width: "stretch",
            padding: "50.0",
            div {
                background: "{mid}",
                height: "stretch",
                width: "stretch",
                padding: "{padding}",
                div {
                    background: "{small}",
                    height: "stretch",
                    width: "stretch",
                }
            }
        }
    })
}
