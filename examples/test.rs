use std::time::Duration;

use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use tokio::time::sleep;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let colors = use_state(&cx, || vec!["green", "blue", "red"]);
    let padding = use_state(&cx, || 10);

    use_effect(&cx, colors, |colors| async move {
        sleep(Duration::from_millis(1000)).await;
        colors.with_mut(|colors| colors.reverse());
    });

    use_effect(&cx, padding, |padding| async move {
        sleep(Duration::from_millis(10)).await;
        padding.with_mut(|padding| {
            if *padding < 65 {
                *padding += 1;
            } else {
                *padding = 5;
            }
        });
    });

    let big = colors[0];
    let mid = colors[1];
    let small = colors[2];

    cx.render(rsx! {
        view {
            background: "{big}",
            height: "stretch",
            width: "stretch",
            padding: "50",
            text {
                layer: "1",
                "hello",
            }
            view {
                background: "{mid}",
                height: "auto",
                width: "stretch",
                padding: "{padding}",
                layer: "2",
                text {
                    layer: "1",
                    "World",
                }
                view {
                    background: "{small}",
                    height: "auto",
                    width: "stretch",
                    padding: "20",
                    layer: "1",
                    text {
                        layer: "1",
                        "ddddddd",
                    }
                }
            },
        }
    })
}
