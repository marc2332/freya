#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[derive(Props)]
struct DragProviderProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
fn DragProvider<'a, T: 'static>(cx: Scope<'a, DragProviderProps<'a>>) -> Element<'a> {
    use_shared_state_provider::<Option<T>>(cx, || None);
    render!(&cx.props.children)
}

#[derive(Props)]
struct DragZoneProps<'a, T> {
    // TODO: Make this optional and fallback to `children`
    drag_element: Element<'a>,
    children: Element<'a>,
    data: T,
}

#[allow(non_snake_case)]
fn DragZone<'a, T: 'static + Clone>(cx: Scope<'a, DragZoneProps<'a, T>>) -> Element<'a> {
    let drags = use_shared_state::<Option<T>>(cx);
    let dragging = use_state(cx, || false);
    let pos = use_state(cx, || (0.0, 0.0));
    let (node_reference, size) = use_node_ref(cx);

    let onglobalmouseover = move |e: MouseEvent| {
        if *dragging.get() {
            let size = size.read();
            let coord = e.get_screen_coordinates();
            pos.set((coord.x - size.x as f64, coord.y - size.y as f64));
        }
    };

    let onmousedown = move |e: MouseEvent| {
        let size = size.read();
        let coord = e.get_screen_coordinates();
        pos.set((coord.x - size.x as f64, coord.y - size.y as f64));
        dragging.set(true);
        *drags.unwrap().write() = Some(cx.props.data.clone())
    };

    let onglobalclick = move |_: MouseEvent| {
        if *dragging.get() {
            dragging.set(false);
            pos.set((0.0, 0.0));
            *drags.unwrap().write() = None;
        }
    };

    render!(
        if *dragging.get() {
            render!(
                rect {
                    width: "0",
                    height: "0",
                    scroll_x: "{pos.0}",
                    scroll_y: "{pos.1}",
                    &cx.props.drag_element
                }
            )
        }
        rect {
            reference: node_reference,
            onglobalclick: onglobalclick,
            onglobalmouseover: onglobalmouseover,
            onmousedown: onmousedown,
            &cx.props.children
        }
    )
}

#[derive(Props)]
struct DropDoneProps<'a, T> {
    ondrop: EventHandler<'a, T>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
fn DropZone<'a, T: 'static + Clone>(cx: Scope<'a, DropDoneProps<'a, T>>) -> Element<'a> {
    let drags = use_shared_state::<Option<T>>(cx);

    let onclick = move |_: MouseEvent| {
        if let Some(drags) = drags {
            if let Some(current_drags) = &*drags.read() {
                cx.props.ondrop.call(current_drags.clone());
            }
            if drags.read().is_some() {
                *drags.write() = None;
            }
        }
    };

    render!(
        rect {
            onclick: onclick,
            &cx.props.children
        }
    )
}

enum SwapDirection {
    LeftToRight,
    RightToLeft,
}

fn app(cx: Scope) -> Element {
    let data = use_state::<(Vec<String>, Vec<String>)>(cx, || {
        (
            vec!["I Like".to_string(), "Rust".to_string(), "🦀!".to_string()],
            vec![],
        )
    });

    let swap = |el: String, direction: SwapDirection| {
        data.with_mut(|data| {
            data.0.retain(|e| e != &el);
            data.1.retain(|e| e != &el);
            match direction {
                SwapDirection::LeftToRight => {
                    data.1.push(el);
                }
                SwapDirection::RightToLeft => {
                    data.0.push(el);
                }
            }
        });
    };

    render!(
        DragProvider::<String> {
            rect {
                direction: "horizontal",
                width: "100%",
                height: "100%",
                DropZone::<String> {
                    ondrop: move |data: String| {
                        swap(data, SwapDirection::RightToLeft);
                    }
                    rect {
                        width: "50%",
                        height: "100%",
                        background: "rgb(234, 226, 183)",
                        for el in data.get().0.iter() {
                            rsx!(
                                DragZone {
                                    data: el.to_string(),
                                    drag_element: render!(
                                        label {
                                            width: "200",
                                            font_size: "20",
                                           "Moving '{el}'"
                                        }
                                    ),
                                    label {
                                        font_size: "30",
                                        "{el}"
                                    }
                                }
                            )
                        }
                    }
                }

                DropZone::<String> {
                    ondrop: move |data: String| {
                        swap(data, SwapDirection::LeftToRight);
                    }
                    rect {
                        width: "100%",
                        height: "100%",
                        background: "rgb(0, 48, 73)",
                        direction: "vertical",
                        color: "white",
                        for el in data.get().1.iter() {
                            rsx!(
                                DragZone {
                                    data: el.to_string(),
                                    drag_element: render!(
                                        label {
                                            width: "200",
                                            font_size: "20",
                                           "Moving '{el}'"
                                        }
                                    ),
                                    label {
                                        font_size: "30",
                                        "{el}"
                                    }
                                }
                            )
                        }
                    }
                }
            }
        }
    )
}
