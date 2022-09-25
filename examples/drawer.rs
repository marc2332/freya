#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus::{events::MouseEvent, prelude::*};
use freya::{dioxus_elements, *};

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
    let (start_opened, restart_opened, progress_opened) =
        use_animation(&cx, || AnimationMode::new_sine_in(-225.0..=0.0, 130));
    let (start_closed, restart_closed, progress_closed) =
        use_animation(&cx, || AnimationMode::new_sine_in(0.0..=-225.0, 200));

    use_effect(&cx, &cx.props.opened, move |opened| async move {
        if opened {
            start_opened();
            restart_closed();
        } else {
            start_closed();
            restart_opened();
        }
    });

    let pos = if cx.props.opened {
        progress_opened
    } else {
        progress_closed
    };

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
                        body: cx.render(rsx!( label { "Close Drawer"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                    Button {
                        onclick: move |_| {  },
                        body: cx.render(rsx!( label { "Hi"} ))
                    }
                 ))
            }
            rect {
                height: "100%",
                width: "100%",
                onclick: move |_| { opened.set(false) },
                rect {
                    padding: "5",
                    height: "30",
                    width: "100",
                    background: "black",
                    onclick: move |_| { opened.set(true) },
                   label { "Open Drawer"}
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
