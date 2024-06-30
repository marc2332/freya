use dioxus::prelude::*;
use freya_common::NodeReferenceLayout;
use freya_elements::{
    elements as dioxus_elements,
    events::MouseEvent,
};
use freya_hooks::{
    use_applied_theme,
    use_node_signal,
    use_platform,
    ResizableHandleTheme,
    ResizableHandleThemeWith,
};
use winit::window::CursorIcon;

enum ResizableItem {
    Panel(f32),
    Handle,
}

impl ResizableItem {
    /// Get the size of the [ResizableItem::Panel]. Will panic if called in a [ResizableItem::Handle].
    fn size(&self) -> f32 {
        match self {
            Self::Panel(size) => *size,
            Self::Handle => panic!("Not a Panel"),
        }
    }

    /// Try to write a size of a [ResizableItem::Panel]. Will return [None] if called in a [ResizableItem::Handle].
    fn try_write_size(&mut self) -> Option<&mut f32> {
        match self {
            Self::Panel(old_size) => Some(old_size),
            Self::Handle => None,
        }
    }
}

#[derive(Default)]
struct ResizableContext {
    pub registry: Vec<ResizableItem>,
    pub direction: String,
}

/// Resizable container, used in combination with [ResizablePanel] and [ResizableHandle].
///
/// Example:
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         ResizableContainer {
///             ResizablePanel {
///                 initial_size: 50.0,
///                 label {
///                     "Panel 1"
///                 }
///             }
///             ResizableHandle { }
///             ResizablePanel {
///                 initial_size: 50.0,
///                 label {
///                     "Panel 2"
///                 }
///             }
///         }
///     )
/// }
/// ```
#[component]
pub fn ResizableContainer(
    /// Direction of the container, `vertical`/`horizontal`.
    /// Default to `vertical`.
    #[props(default = "vertical".to_string())]
    direction: String,
    /// Inner children for the [ResizableContainer].
    children: Element,
) -> Element {
    let (node_reference, size) = use_node_signal();
    use_context_provider(|| size);

    use_context_provider(|| {
        Signal::new(ResizableContext {
            direction: direction.clone(),
            ..Default::default()
        })
    });

    rsx!(
        rect {
            reference: node_reference,
            direction: "{direction}",
            width: "fill",
            height: "fill",
            {children}
        }
    )
}

/// Resizable panel to be used in combination with [ResizableContainer] and [ResizableHandle].
#[component]
pub fn ResizablePanel(
    /// Initial size of the Panel. Value should be between `0..100`. Default to `10`.
    #[props(default = 10.)]
    initial_size: f32, // TODO: Automatically assign the remaining space in the last element with unspecified size?
    /// Inner children for the [ResizablePanel].
    children: Element,
) -> Element {
    let mut registry = use_context::<Signal<ResizableContext>>();

    let index = use_hook(move || {
        registry
            .write()
            .registry
            .push(ResizableItem::Panel(initial_size));
        registry.peek().registry.len() - 1
    });

    let registry = registry.read();

    let size = registry.registry[index].size();
    let (width, height) = match registry.direction.as_str() {
        "horizontal" => (format!("{size}%"), "fill".to_owned()),
        _ => ("fill".to_owned(), format!("{size}%")),
    };

    rsx!(
        rect {
            width: "{width}",
            height: "{height}",
            overflow: "clip",
            {children}
        }
    )
}

/// Describes the current status of the Handle.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum HandleStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the handle.
    Hovering,
}

/// Resizable panel to be used in combination with [ResizableContainer] and [ResizablePanel].
#[component]
pub fn ResizableHandle(
    /// Theme override.
    theme: Option<ResizableHandleThemeWith>,
) -> Element {
    let ResizableHandleTheme {
        background,
        hover_background,
    } = use_applied_theme!(&theme, resizable_handle);
    let (node_reference, size) = use_node_signal();
    let mut clicking = use_signal(|| false);
    let mut status = use_signal(HandleStatus::default);
    let mut registry = use_context::<Signal<ResizableContext>>();
    let container_size = use_context::<ReadOnlySignal<NodeReferenceLayout>>();
    let platform = use_platform();

    use_drop(move || {
        if *status.peek() == HandleStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let index = use_hook(move || {
        registry.write().registry.push(ResizableItem::Handle);
        registry.peek().registry.len() - 1
    });

    let cursor = match registry.read().direction.as_str() {
        "horizontal" => CursorIcon::ColResize,
        _ => CursorIcon::RowResize,
    };

    let onmouseleave = move |_: MouseEvent| {
        *status.write() = HandleStatus::Idle;
        if !*clicking.peek() {
            platform.set_cursor(CursorIcon::default());
        }
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();
        *status.write() = HandleStatus::Hovering;
        platform.set_cursor(cursor);
    };

    let onmouseover = {
        move |e: MouseEvent| {
            if *clicking.peek() {
                let coordinates = e.get_screen_coordinates();
                let mut registry = registry.write();

                let displacement_per = match registry.direction.as_str() {
                    "horizontal" => {
                        let displacement = coordinates.x as f32 - size.peek().area.min_x();
                        100. / container_size.read().area.width() * displacement
                    }
                    _ => {
                        let displacement = coordinates.y as f32 - size.peek().area.min_y();
                        100. / container_size.read().area.height() * displacement
                    }
                };

                if displacement_per > 0. {
                    // Resizing to the right

                    let mut available_per = displacement_per;
                    let mut acc_per = 0.0;

                    // Resize panels to the right
                    for next_item in &mut registry.registry[index..].iter_mut() {
                        if let Some(size) = next_item.try_write_size() {
                            let old_size = *size;
                            let new_size = (*size - available_per).clamp(0., 100.);

                            *size = new_size;
                            available_per = displacement_per - new_size - old_size;
                            acc_per -= new_size - old_size;

                            // Stop carrying panels to the right as they still have size
                            if old_size > 0. {
                                break;
                            }
                        }
                    }

                    // Resize panels to the left
                    for prev_item in &mut registry.registry[0..index].iter_mut().rev() {
                        if let Some(size) = prev_item.try_write_size() {
                            let old_size = *size;
                            let new_size = (*size + acc_per).clamp(0., 100.);

                            *size = new_size;

                            // Stop carrying panels to the left as they still have size
                            if old_size > 0. {
                                break;
                            }
                        }
                    }
                } else {
                    // Resizing to the left
                    let mut available_per = displacement_per;
                    let mut acc_per = 0.0;
                    // Resize panels to the left
                    for prev_item in &mut registry.registry[0..index].iter_mut().rev() {
                        if let Some(size) = prev_item.try_write_size() {
                            let old_size = *size;
                            let new_size = (*size + available_per).clamp(0., 100.);

                            *size = new_size;
                            available_per = displacement_per - new_size - old_size;
                            acc_per += new_size - old_size;

                            // Stop carrying panels to the left as they still have size
                            if old_size > 0. {
                                break;
                            }
                        }
                    }

                    // Resize panels to the right
                    for next_item in &mut registry.registry[index..].iter_mut() {
                        if let Some(size) = next_item.try_write_size() {
                            let old_size = *size;
                            let new_size = (*size - acc_per).clamp(0., 100.);

                            *size = new_size;

                            // Stop carrying panels to the right as they still have size
                            if old_size > 0. {
                                break;
                            }
                        }
                    }
                }
            }
        }
    };

    let onmousedown = {
        move |e: MouseEvent| {
            e.stop_propagation();
            clicking.set(true);
        }
    };

    let onclick = move |_: MouseEvent| {
        if *clicking.peek() {
            if *status.peek() != HandleStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
            clicking.set(false);
        }
    };

    let (width, height) = match registry.read().direction.as_str() {
        "horizontal" => ("4", "fill"),
        _ => ("fill", "4"),
    };

    let background = match status() {
        _ if clicking() => hover_background,
        HandleStatus::Hovering => hover_background,
        HandleStatus::Idle => background,
    };

    rsx!(rect {
        reference: node_reference,
        width: "{width}",
        height: "{height}",
        background: "{background}",
        onmousedown,
        onglobalclick: onclick,
        onmouseenter,
        onglobalmouseover: onmouseover,
        onmouseleave,
    })
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn resizable_container() {
        fn resizable_container_app() -> Element {
            rsx!(
                ResizableContainer {
                    ResizablePanel {
                        initial_size: 50.,
                        label {
                            "Panel 0"
                        }
                    }
                    ResizableHandle { }
                    ResizablePanel { // Panel 1
                        initial_size: 50.,
                        ResizableContainer {
                            direction: "horizontal",
                            ResizablePanel {
                                initial_size: 33.33,
                                label {
                                    "Panel 2"
                                }
                            }
                            ResizableHandle { }
                            ResizablePanel {
                                initial_size: 33.33,
                                label {
                                    "Panel 3"
                                }
                            }
                            ResizableHandle { }
                            ResizablePanel {
                                initial_size: 33.33,
                                label {
                                    "Panel 4"
                                }
                            }
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(resizable_container_app);
        utils.wait_for_update().await;
        let root = utils.root();

        let container = root.get(0);
        let panel_0 = container.get(0);
        let panel_1 = container.get(2);
        let panel_2 = panel_1.get(0).get(0);
        let panel_3 = panel_1.get(0).get(2);
        let panel_4 = panel_1.get(0).get(4);

        assert_eq!(panel_0.layout().unwrap().area.height().round(), 250.0);
        assert_eq!(panel_1.layout().unwrap().area.height().round(), 250.0);
        assert_eq!(panel_2.layout().unwrap().area.width().round(), 167.0);
        assert_eq!(panel_3.layout().unwrap().area.width().round(), 167.0);
        assert_eq!(panel_4.layout().unwrap().area.width().round(), 167.0);

        // Vertical
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (100.0, 250.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (100.0, 200.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (0.0, 0.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(panel_0.layout().unwrap().area.height().round(), 200.0); // 250 - 50
        assert_eq!(panel_1.layout().unwrap().area.height().round(), 300.0);

        // Horizontal
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (167.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (185.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (0.0, 0.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_eq!(panel_2.layout().unwrap().area.width().round(), 185.0); // 165 + 20
        assert_eq!(panel_3.layout().unwrap().area.width().round(), 148.0);

        // Horizontal but pushing two handles
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (341.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (25.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (25.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(panel_2.layout().unwrap().area.width().round(), 21.0);
        assert_eq!(panel_3.layout().unwrap().area.width().round(), 0.0);
        assert_eq!(panel_4.layout().unwrap().area.width().round(), 479.0);
    }
}
