use std::time::Duration;

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

    use_effect(&cx, colors, |colors| async move {
        sleep(Duration::from_millis(50)).await;
        colors.with_mut(|colors| colors.reverse());
    });

    let big = colors[0];
    let mid = colors[1];
    let small = colors[2];

    cx.render(rsx! {
        div {
            background: "{big}",
            height: "200.0",
            width: "200.0",
            div {
                background: "{mid}",
                height: "150.0",
                width: "75.0",
                div {
                    background: "{small}",
                    height: "75.0",
                    width: "40.0",
                }
            }
        }
    })
}
