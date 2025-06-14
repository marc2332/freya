use dioxus::prelude::*;
use freya_core::{
    custom_attributes::NodeReferenceLayout,
    platform::CursorIcon,
};
use freya_elements::{
    self as dioxus_elements,
    events::MouseEvent,
};
use freya_hooks::{
    use_applied_theme,
    use_node_signal,
    use_platform,
    ResizableHandleTheme,
    ResizableHandleThemeWith,
};

struct Panel {
    pub size: f32,
    pub min_size: f32,
}

enum ResizableItem {
    Panel(Panel),
    Handle,
}

impl ResizableItem {
    /// Get the [Panel] of the [ResizableItem]. Will panic if called in a [ResizableItem::Handle].
    fn panel(&self) -> &Panel {
        match self {
            Self::Panel(panel) => panel,
            Self::Handle => panic!("Not a Panel"),
        }
    }

    /// Try to get the mutable [Panel] of the [ResizableItem]. Will return [None] if called in a [ResizableItem::Handle].
    fn try_panel_mut(&mut self) -> Option<&mut Panel> {
        match self {
            Self::Panel(panel) => Some(panel),
            Self::Handle => None,
        }
    }
}

#[derive(Default)]
struct ResizableContext {
    pub registry: Vec<ResizableItem>,
    pub direction: String,
}

/// Resizable container, used in combination with [ResizablePanel()] and [ResizableHandle()].
///
/// Example:
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         ResizableContainer {
///             direction: "vertical",
///             ResizablePanel {
///                 initial_size: 50.0,
///                 label {
///                     "Panel 1"
///                 }
///             }
///             ResizableHandle { }
///             ResizablePanel {
///                 initial_size: 50.0,
///                 min_size: 30.0,
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
    /// Inner children for the [ResizableContainer()].
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
            content: "flex",
            {children}
        }
    )
}

/// Resizable panel to be used in combination with [ResizableContainer()] and [ResizableHandle()].
#[component]
pub fn ResizablePanel(
    /// Initial size in % for this panel. Default to `10`.
    #[props(default = 10.)]
    initial_size: f32, // TODO: Automatically assign the remaining space in the last element with unspecified size?
    /// Minimum size in % for this panel. Default to `4`.
    #[props(default = 4.)]
    min_size: f32,
    /// Inner children for the [ResizablePanel()].
    children: Element,
) -> Element {
    let mut registry = use_context::<Signal<ResizableContext>>();

    let index = use_hook(move || {
        registry.write().registry.push(ResizableItem::Panel(Panel {
            size: initial_size,
            min_size,
        }));
        registry.peek().registry.len() - 1
    });

    let registry = registry.read();

    let Panel { size, .. } = registry.registry[index].panel();

    let (width, height) = match registry.direction.as_str() {
        "horizontal" => (format!("flex({size})"), "fill".to_owned()),
        _ => ("fill".to_owned(), format!("flex({size}")),
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

/// Resizable panel to be used in combination with [ResizableContainer()] and [ResizablePanel()].
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
    let mut allow_resizing = use_signal(|| false);

    use_memo(move || {
        size.read();
        allow_resizing.set(true);

        // Only allow more resizing after the node layout has updated
    });

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
        if !clicking() {
            platform.set_cursor(CursorIcon::default());
        }
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();
        *status.write() = HandleStatus::Hovering;
        platform.set_cursor(cursor);
    };

    let onmousemove = move |e: MouseEvent| {
        if clicking() {
            if !allow_resizing() {
                return;
            }

            let coordinates = e.get_screen_coordinates();
            let mut registry = registry.write();

            let displacement_per: f32 = match registry.direction.as_str() {
                "horizontal" => {
                    let container_width = container_size.read().area.width();
                    let displacement = coordinates.x as f32 - size.read().area.min_x();
                    100. / container_width * displacement
                }
                _ => {
                    let container_height = container_size.read().area.height();
                    let displacement = coordinates.y as f32 - size.read().area.min_y();
                    100. / container_height * displacement
                }
            };

            let mut changed_panels = false;

            if displacement_per >= 0. {
                // Resizing to the right

                let mut acc_per = 0.0;

                // Resize panels to the right
                for next_item in &mut registry.registry[index..].iter_mut() {
                    if let Some(panel) = next_item.try_panel_mut() {
                        let old_size = panel.size;
                        let new_size = (panel.size - displacement_per).clamp(panel.min_size, 100.);

                        if panel.size != new_size {
                            changed_panels = true
                        }

                        panel.size = new_size;
                        acc_per -= new_size - old_size;

                        if old_size > panel.min_size {
                            break;
                        }
                    }
                }

                // Resize panels to the left
                for prev_item in &mut registry.registry[0..index].iter_mut().rev() {
                    if let Some(panel) = prev_item.try_panel_mut() {
                        let new_size = (panel.size + acc_per).clamp(panel.min_size, 100.);

                        if panel.size != new_size {
                            changed_panels = true
                        }

                        panel.size = new_size;
                        break;
                    }
                }
            } else {
                // Resizing to the left

                let mut acc_per = 0.0;

                // Resize panels to the left
                for prev_item in &mut registry.registry[0..index].iter_mut().rev() {
                    if let Some(panel) = prev_item.try_panel_mut() {
                        let old_size = panel.size;
                        let new_size = (panel.size + displacement_per).clamp(panel.min_size, 100.);

                        if panel.size != new_size {
                            changed_panels = true
                        }

                        panel.size = new_size;
                        acc_per += new_size - old_size;

                        if old_size > panel.min_size {
                            break;
                        }
                    }
                }

                // Resize panels to the right
                for next_item in &mut registry.registry[index..].iter_mut() {
                    if let Some(panel) = next_item.try_panel_mut() {
                        let new_size = (panel.size - acc_per).clamp(panel.min_size, 100.);

                        if panel.size != new_size {
                            changed_panels = true
                        }

                        panel.size = new_size;
                        break;
                    }
                }
            }

            if changed_panels {
                allow_resizing.set(false);
            }
        }
    };

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();
        clicking.set(true);
    };

    let onclick = move |_: MouseEvent| {
        if clicking() {
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
        onglobalmousemove: onmousemove,
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

        assert_eq!(panel_0.layout().unwrap().area.height().round(), 248.0);
        assert_eq!(panel_1.layout().unwrap().area.height().round(), 248.0);
        assert_eq!(panel_2.layout().unwrap().area.width().round(), 164.0);
        assert_eq!(panel_3.layout().unwrap().area.width().round(), 164.0);
        assert_eq!(panel_4.layout().unwrap().area.width().round(), 164.0);

        // Vertical
        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (100.0, 250.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseMove,
            cursor: (100.0, 200.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseUp,
            cursor: (0.0, 0.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(panel_0.layout().unwrap().area.height().round(), 200.0); // 250 - 50
        assert_eq!(panel_1.layout().unwrap().area.height().round(), 296.0); // 500 - 200 - 4

        // Horizontal
        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (167.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseMove,
            cursor: (187.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseUp,
            cursor: (0.0, 0.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert_eq!(panel_2.layout().unwrap().area.width().round(), 187.0); // 167 + 20
        assert_eq!(panel_3.layout().unwrap().area.width().round(), 141.0);
    }
}
