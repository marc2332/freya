use std::time::Duration;

use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use tokio::time::sleep;
use trev::launch;

fn main() {
    launch(app);
}

#[derive(Props)]
struct DrawerOptions<'a> {
    opened: bool,
    body: Element<'a>,
}

#[allow(non_snake_case)]
fn Drawer<'a>(cx: Scope<'a, DrawerOptions<'a>>) -> Element<'a> {
    let pos = use_state(&cx, || 0);

    use_effect(
        &cx,
        (&cx.props.opened, pos),
        move |(opened, mut pos)| async move {
            if *pos == -150 && opened == false {
                pos -= 100;
            }
            if *pos == -250 && opened == true {
                pos += 100;
            }
            if (*pos >= 0 && opened == true) || (*pos <= -150 && opened == false) {
                return;
            }

            if opened {
                pos += 1;
            } else {
                pos -= 1;
            }

            sleep(Duration::from_millis(5)).await;
        },
    );

    cx.render(rsx! {
        view {
            height: "100%",
            width: "0",
            scroll_x: "{pos}",
            layer: "1",
            view {
                height: "100%",
                width: "200",
                background: "blue",
                shadow: "5 0 200 25.0 black",
                &cx.props.body
            }
        }
    })
}
fn app(cx: Scope) -> Element {
    let opened = use_state(&cx, || false);

    cx.render(rsx!(
        view {
            height: "100%",
            width: "100%",
            direction: "horizontal",
            Drawer {
                opened: *opened.get(),
                body: cx.render(rsx!( view {
                    height: "45",
                    width: "100%",
                    background: "black",
                    layer: "1",
                    onclick: move |_| { opened.set(false) },
                    padding: "10",
                    text { layer:"1", "CLOSE"}
                } ))
            }
            view {
                height: "100%",
                width: "100%",
                view {
                    height: "30",
                    width: "80",
                    background: "black",
                    onclick: move |_| { opened.set(true) },
                    text { layer:"1", "open"}
                }
            }
        }
    ))
}
