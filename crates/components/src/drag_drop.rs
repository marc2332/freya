use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::use_node_signal;
use torin::prelude::CursorPoint;

/// Properties for the [`DragProvider`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragProviderProps {
    /// Inner children of the DragProvider.
    children: Element,
}

/// Provide a common place for [`DragZone`]s and [`DropZone`]s to exchange their data.
#[allow(non_snake_case)]
pub fn DragProvider<T: 'static>(DragProviderProps { children }: DragProviderProps) -> Element {
    use_context_provider::<Signal<Option<T>>>(|| Signal::new(None));
    rsx!({ children })
}

/// Properties for the [`DragZone`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DragZoneProps<T: Clone + 'static + PartialEq> {
    /// Element visible when dragging the element. This follows the cursor.
    drag_element: Element,
    /// Inner children for the DropZone.
    children: Element,
    /// Data that will be handled to the destination [`DropZone`].
    data: T,
}

/// Make the inner children draggable to other [`DropZone`].
#[allow(non_snake_case)]
pub fn DragZone<T: 'static + Clone + PartialEq>(
    DragZoneProps {
        data,
        children,
        drag_element,
    }: DragZoneProps<T>,
) -> Element {
    let mut drags = use_context::<Signal<Option<T>>>();
    let mut dragging = use_signal(|| false);
    let mut pos = use_signal(CursorPoint::default);
    let (node_reference, size) = use_node_signal();

    let onglobalmouseover = move |e: MouseEvent| {
        if *dragging.read() {
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
        *drags.write() = Some(data.clone());
    };

    let onglobalclick = move |_: MouseEvent| {
        if *dragging.read() {
            dragging.set(false);
            pos.set((0.0, 0.0).into());
            *drags.write() = None;
        }
    };

    rsx!(
        if *dragging.read() {
            rect {
                width: "0",
                height: "0",
                offset_x: "{pos.read().x}",
                offset_y: "{pos.read().y}",
                {drag_element}
            }
        }
        rect {
            reference: node_reference,
            onglobalclick,
            onglobalmouseover: onglobalmouseover,
            onmousedown,
            {children}
        }
    )
}

/// Properties for the [`DropZone`] component.
#[derive(Props, PartialEq, Clone)]
pub struct DropZoneProps<T: 'static + PartialEq + Clone> {
    /// Inner children for the DropZone.
    children: Element,
    /// Handler for the `ondrop` event.
    ondrop: EventHandler<T>,
}

/// Elements from [`DragZone`]s can be dropped here.
#[allow(non_snake_case)]
pub fn DropZone<T: 'static + Clone + PartialEq>(props: DropZoneProps<T>) -> Element {
    let mut drags = use_context::<Signal<Option<T>>>();

    let onclick = move |_: MouseEvent| {
        if let Some(current_drags) = &*drags.read() {
            props.ondrop.call(current_drags.clone());
        }
        if drags.read().is_some() {
            *drags.write() = None;
        }
    };

    rsx!(
        rect {
            onclick,
            {props.children}
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn drag_drop() {
        fn drop_app() -> Element {
            let mut state = use_signal::<bool>(|| false);

            rsx!(
                DragProvider::<bool> {
                    rect {
                        height: "50%",
                        width: "100%",
                        DragZone {
                            data: true,
                            drag_element: rsx!(
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
                                "Enabled: {state.read()}"
                            }
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(drop_app);
        let root = utils.root();
        utils.wait_for_update().await;

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (5.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        assert_eq!(root.get(0).get(0).get(0).get(0).text(), Some("Moving"));

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
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
