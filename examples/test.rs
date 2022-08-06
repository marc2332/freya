use dioxus::prelude::*;
use std::time::Duration;
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
        div {
            background: "{big}",
            height: "stretch",
            width: "stretch",
            padding: "50",
            p {
                tabindex: "1",
                "hello",
            }
            div {
                background: "{mid}",
                height: "auto",
                width: "stretch",
                padding: "{padding}",
                tabindex: "2",
                p {
                    tabindex: "1",
                    "World",
                }
                div {
                    background: "yellow",
                    height: "100",
                    width: "100",
                    padding: "20",
                    tabindex: "1",
                    p {
                        tabindex: "1",
                        "ddddddd",
                    }
                }
            },
        }
    })
}
