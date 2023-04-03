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

    let onglobalmouseover = |e: MouseEvent| {
        if *dragging.get() {
            pos.set(e.get_screen_coordinates().to_tuple());
        }
    };

    let onmousedown = move |e: MouseEvent| {
        pos.set(e.get_element_coordinates().to_tuple());
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
        rect {
            onglobalclick: onglobalclick,
            onglobalmouseover: onglobalmouseover,
            onmousedown: onmousedown,
            &cx.props.children
        }
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

fn app(cx: Scope) -> Element {
    let data = use_state::<(Vec<String>, Vec<String>)>(cx, || {
        (
            vec!["I Like".to_string(), "Rust".to_string(), "🦀!".to_string()],
            vec![],
        )
    });

    let swap = |el: String| {
        data.with_mut(|data| {
            data.0.retain(|e| e != &el);
            data.1.push(el);
        });
    };

    render!(
        DragProvider::<String> {
            rect {
                direction: "horizontal",
                width: "100%",
                height: "100%",
                rect {
                    width: "50%",
                    height: "100%",
                    background: "yellow",
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
                DropZone::<String> {
                    ondrop: move |data: String| {
                        swap(data);
                    }
                    rect {
                        width: "100%",
                        height: "100%",
                        background: "red",
                        direction: "vertical",
                        color: "white",
                        for el in data.get().1.iter() {
                            rsx!(
                                label {
                                    font_size: "30",
                                    "{el}"
                                }
                            )
                        }
                    }
                }
            }
        }
    )
}
