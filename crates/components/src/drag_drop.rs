use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::use_node_ref;
use torin::prelude::CursorPoint;

/// [`DragProvider`] component properties.
#[derive(Props)]
pub struct DragProviderProps<'a> {
    /// Inner children of the DragProvider.
    children: Element<'a>,
}

/// Provide a common place for [`DragZone`]s and [`DropZone`]s to exchange their data.
///
/// # Props
/// See [`DragProviderProps`].
///
#[allow(non_snake_case)]
pub fn DragProvider<'a, T: 'static>(cx: Scope<'a, DragProviderProps<'a>>) -> Element<'a> {
    use_shared_state_provider::<Option<T>>(cx, || None);
    render!(&cx.props.children)
}

/// [`DragZone`] component properties.
#[derive(Props)]
pub struct DragZoneProps<'a, T> {
    /// Element visible when dragging the element. This follows the cursor.
    drag_element: Element<'a>,
    /// Inner children for the DropZone.
    children: Element<'a>,
    /// Data that will be handled to the destination [`DropZone`].
    data: T,
}

/// Make the inner children draggable to other [`DropZone`].
///
/// # Props
/// See [`DragZoneProps`].
///
#[allow(non_snake_case)]
pub fn DragZone<'a, T: 'static + Clone>(cx: Scope<'a, DragZoneProps<'a, T>>) -> Element<'a> {
    let drags = use_shared_state::<Option<T>>(cx);
    let dragging = use_state(cx, || false);
    let pos = use_state(cx, CursorPoint::default);
    let (node_reference, size) = use_node_ref(cx);

    let onglobalmouseover = move |e: MouseEvent| {
        if *dragging.get() {
            let size = size.read();
            let coord = e.get_screen_coordinates();
            pos.set(
                (
                    coord.x - size.area.min_x() as f64,
                    coord.y - size.area.min_y() as f64,
                )
                    .into(),
            );
        }
    };

    let onmousedown = move |e: MouseEvent| {
        let size = size.read();
        let coord = e.get_screen_coordinates();
        pos.set(
            (
                coord.x - size.area.min_x() as f64,
                coord.y - size.area.min_y() as f64,
            )
                .into(),
        );
        dragging.set(true);
        *drags.unwrap().write() = Some(cx.props.data.clone());
    };

    let onglobalclick = move |_: MouseEvent| {
        if *dragging.get() {
            dragging.set(false);
            pos.set((0.0, 0.0).into());
            *drags.unwrap().write() = None;
        }
    };

    render!(
        if *dragging.get() {
            render!(
                rect {
                    width: "0",
                    height: "0",
                    offset_x: "{pos.x}",
                    offset_y: "{pos.y}",
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

/// [`DropZone`] component properties.
#[derive(Props)]
pub struct DropZoneProps<'a, T> {
    /// Inner children for the DropZone.
    children: Element<'a>,
    /// Handler for the `ondrop` event.
    ondrop: EventHandler<'a, T>,
}

/// Elements from [`DragZone`]s can be dropped here.
///
/// # Props
/// See [`DropZoneProps`].
///
#[allow(non_snake_case)]
pub fn DropZone<'a, T: 'static + Clone>(cx: Scope<'a, DropZoneProps<'a, T>>) -> Element<'a> {
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

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::{launch_test, FreyaEvent, MouseButton};

    #[tokio::test]
    pub async fn drag_drop() {
        fn drop_app(cx: Scope) -> Element {
            let state = use_state::<bool>(cx, || false);

            render!(
                DragProvider::<bool> {
                    rect {
                        height: "50%",
                        width: "100%",
                        DragZone {
                            data: true,
                            drag_element: render!(
                                label {
                                    width: "200",
                                    "Moving"
                                }
                            ),
                            label {
                                "Move"
                            }
                        }
                    },
                    DropZone {
                        ondrop: move |data: bool| {
                            state.set(data);
                        },
                        rect {
                            height: "50%",
                            width: "100%",
                            label {
                                "Enabled: {state}"
                            }
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(drop_app);
        let root = utils.root();
        utils.wait_for_update().await;

        utils.push_event(FreyaEvent::Mouse {
            name: "mousedown".to_string(),
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        utils.push_event(FreyaEvent::Mouse {
            name: "mouseover".to_string(),
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        utils.push_event(FreyaEvent::Mouse {
            name: "mouseover".to_string(),
            cursor: (5.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        assert_eq!(root.get(0).get(0).get(0).get(0).text(), Some("Moving"));

        utils.push_event(FreyaEvent::Mouse {
            name: "click".to_string(),
            cursor: (5.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        assert_eq!(
            root.get(1).get(0).get(0).get(0).text(),
            Some("Enabled: true")
        );
    }
}
