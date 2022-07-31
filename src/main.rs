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
            height: "stretch",
            width: "stretch",
            padding: "50.0",
            div {
                background: "{mid}",
                height: "stretch",
                width: "stretch",
                padding: "60.0",
                div {
                    background: "{small}",
                    height: "stretch",
                    width: "stretch",
                }
            }
        }
    })
}
