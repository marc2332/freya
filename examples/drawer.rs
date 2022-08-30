use std::time::Duration;

use dioxus::{prelude::*, core::UiEvent, events::{MouseData, MouseEvent}};
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

            sleep(Duration::from_millis(5)).await;
        },
    );

    cx.render(rsx! {
        view {
            height: "100%",
            width: "0",
            scroll_x: "{pos}",
            view {
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
        view {
            height: "100%",
            width: "100%",
            direction: "horizontal",
            Drawer {
                opened: *opened.get(),
                body: cx.render(rsx!( 
                    Button {
                        onclick: move |_| { opened.set(false) },
                        body: cx.render(rsx!(  text { "CLOSE"} ))
                    }
                    ScrollView {
                        body: cx.render(rsx!( 
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            Button {
                                onclick: move |_| {  },
                                body: cx.render(rsx!(  text { "Hi"} ))
                            }
                            
                        ))
                    }
                 ))
            }
            view {
                height: "100%",
                width: "100%",
                view {
                    height: "30",
                    width: "80",
                    background: "black",
                    onclick: move |_| { opened.set(true) },
                    text { "open"}
                }
            }
        }
    ))
}

#[allow(non_snake_case)]
fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element {

    cx.render(rsx!(
        view {
            width: "100%",
            height: "60",
            padding: "10",
            onclick: move |evt| cx.props.onclick.call(evt),
            view {
                width: "100%",
                height: "100%",
                background: "black",
                padding: "25",
                radius: "7",
                &cx.props.body
            }
        }
    ))
}

#[derive(Props)]
struct ButtonProps<'a> {
    body: Element<'a>,
    onclick: EventHandler<'a, MouseEvent>
}

#[allow(non_snake_case)]
fn ScrollView<'a>(cx: Scope<'a, ScrollViewProps<'a>>) -> Element {
    let mut height = use_state(&cx, || 0);

    let onscroll = move |e: UiEvent<MouseData>| {
        let page = e.coordinates().page();
        if *height.get() >= 0 && page.y > 0.0 {
            return;
        }
        height += (page.y as i32) * 20;
    };

    cx.render(rsx!(
        container {
            width: "100%",
            height: "100%",
            scroll_y: "{height}",
            onscroll: onscroll,
            &cx.props.body
        }
    ))
}

#[derive(Props)]
struct ScrollViewProps<'a> {
    body: Element<'a>,
}