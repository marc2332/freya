use components::ScrollView;
use dioxus::{events::MouseEvent, prelude::*};
use elements_namespace as dioxus_elements;
use freya::launch;
use tokio::time::Instant;

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
    let timer = use_state(&cx, || Instant::now());

    use_effect(
        &cx,
        (&cx.props.opened, pos, timer),
        move |(opened, mut pos, timer)| async move {
            async {
                loop {
                    if timer.elapsed().as_nanos() > 500000 {
                        break;
                    }
                }
            }
            .await;

            if *pos == -200 && opened == false {
                pos -= 100;
            }
            if *pos == -250 && opened == true {
                pos += 100;
            }
            if (*pos >= 0 && opened == true) || (*pos <= -200 && opened == false) {
                return;
            }

            if opened {
                pos += 1;
            } else {
                pos -= 1;
            }

            timer.with_mut(|timer| {
                *timer = Instant::now();
            });
        },
    );

    cx.render(rsx! {
        rect {
            height: "100%",
            width: "0",
            scroll_x: "{pos}",
            rect {
                height: "100%",
                width: "200",
                background: "gray",
                layer: "-10",
                shadow: "5 0 200 25.0 black",
                &cx.props.body
            }
        }
    })
}
fn app(cx: Scope) -> Element {
    let opened = use_state(&cx, || false);

    cx.render(rsx!(
        rect {
            height: "100%",
            width: "100%",
            direction: "horizontal",
            Drawer {
                opened: *opened.get(),
                body: cx.render(rsx!(
                    Button {
                        onclick: move |_| { opened.set(false) },
                        body: cx.render(rsx!(  label { "CLOSE"} ))
                    }
                    ScrollView {
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                        Button {
                            onclick: move |_| {  },
                            body: cx.render(rsx!(  label { "Hi"} ))
                        }
                    }
                 ))
            }
            rect {
                height: "100%",
                width: "100%",
                rect {
                    height: "30",
                    width: "80",
                    background: "black",
                    onclick: move |_| { opened.set(true) },
                    label { "open"}
                }
            }
        }
    ))
}

#[allow(non_snake_case)]
fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {
    let background = use_state(&cx, || "black");

    cx.render(rsx!(
        rect {
            width: "100%",
            height: "60",
            padding: "10",
            onclick: move |evt| cx.props.onclick.call(evt),
            rect {
                width: "100%",
                height: "100%",
                padding: "2",
                radius: "7",
                background: "green",
                rect {
                    onmouseover: move |_| {
                        background.set("gray");
                    },
                    onmouseleave: move |_| {
                        background.set("black");
                    },
                    width: "100%",
                    height: "100%",
                    background: "{background}",
                    padding: "25",
                    radius: "6",
                    &cx.props.body
                }
            }
        }
    ))
}

#[derive(Props)]
struct ButtonProps<'a> {
    body: Element<'a>,
    onclick: EventHandler<'a, MouseEvent>,
}
