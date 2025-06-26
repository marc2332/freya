use dioxus::prelude::*;
use freya_core::{
    custom_attributes::NodeReferenceLayout,
    platform::CursorIcon,
};
use freya_elements::{
    self as dioxus_elements,
    events::MouseEvent,
    PointerEvent,
};
use freya_hooks::{
    use_applied_theme,
    use_node_signal,
    use_platform,
    ResizableHandleTheme,
    ResizableHandleThemeWith,
    UseId,
};

#[derive(Clone, Copy, Debug)]
struct Panel {
    pub size: f32,
    pub initial_size: f32,
    pub min_size: f32,
    pub id: usize,
}

#[derive(Default)]
struct ResizableContext {
    pub panels: Vec<Panel>,
    pub direction: String,
}

/// Resizable container, used in combination with [ResizablePanel()].
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

/// Resizable panel to be used in combination with [ResizableContainer()].
#[component]
pub fn ResizablePanel(
    /// Initial size in factors (e.g, 1 for 25% if the total is 4, or 25 for 25% if total is 100) for this panel.
    #[props(default = 50.)]
    initial_size: f32,
    /// Minimum size in factors for this panel. Default to 25% of the `initial_size`.
    min_size: Option<f32>,
    /// Inner children for the [ResizablePanel()].
    children: Element,
    /// Numeric order of this panel, only use if this panel will be render conditionally.
    order: Option<usize>,
) -> Element {
    let mut registry = use_context::<Signal<ResizableContext>>();

    let id = use_hook(move || {
        let mut registry = registry.write();
        let id = UseId::<ResizableContext>::get_in_hook();

        let created_panel = Panel {
            initial_size,
            size: initial_size,
            min_size: min_size.unwrap_or(initial_size * 0.25),
            id,
        };

         let mut buffer = created_panel.size;

        for panel in &mut registry.panels.iter_mut() {
            let resized_sized = (panel.initial_size - panel.size).min(buffer);

            panel.size = (panel.size - resized_sized).max(panel.min_size);
            let new_resized_sized = panel.initial_size - panel.size;
            buffer -= new_resized_sized;
        }

        if let Some(order) = order {
            registry.panels.insert(order, created_panel);
        } else {
            registry.panels.push(created_panel);
        }

        id
    });

    use_drop(move || {
        let mut registry = registry.write();
        let removed_panel = registry.panels.iter().find(|p| p.id == id).cloned().unwrap();
        registry.panels.retain(|e| e.id != id);

        let mut buffer = removed_panel.size;

        for panel in &mut registry.panels.iter_mut() {
            let resized_sized = (panel.initial_size - panel.size).min(buffer);

            panel.size = (panel.size + resized_sized).max(panel.min_size);
            let new_resized_sized = panel.initial_size - panel.size;
            buffer -= new_resized_sized;
        }
    });

    let registry = registry.read();
    let index = registry
        .panels
        .iter()
        .position(|e| e.id == id)
        .unwrap_or_default();

    let Panel { size, .. } = registry.panels[index];

    let (width, height) = match registry.direction.as_str() {
        "horizontal" => (format!("flex({size})"), "fill".to_owned()),
        _ => ("fill".to_owned(), format!("flex({size}")),
    };

    rsx!(
        if index > 0 {
            ResizableHandle {
                panel_index: index
             }
        }
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

#[component]
fn ResizableHandle(
    panel_index: usize,
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

    // Only allow more resizing after the node layout has updated
    use_effect(move || {
        size.read();
        allow_resizing.set(true);
    });

    use_drop(move || {
        if *status.peek() == HandleStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let cursor = match registry.read().direction.as_str() {
        "horizontal" => CursorIcon::ColResize,
        _ => CursorIcon::RowResize,
    };

    let onpointerleave = move |_: PointerEvent| {
        *status.write() = HandleStatus::Idle;
        if !clicking() {
            platform.set_cursor(CursorIcon::default());
        }
    };

    let onpointerenter = move |e: PointerEvent| {
        e.stop_propagation();
        *status.write() = HandleStatus::Hovering;
        platform.set_cursor(cursor);
    };

    let oncaptureglobalmousemove = move |e: MouseEvent| {
        if clicking() {
            if !allow_resizing() {
                return;
            }

            let coordinates = e.get_screen_coordinates();
            let mut registry = registry.write();

            let total_size = registry.panels.iter().fold(0., |acc, p| acc + p.size);

            let displacement_per: f32 = match registry.direction.as_str() {
                "horizontal" => {
                    let container_width = container_size.read().area.width();
                    let displacement = coordinates.x as f32 - size.read().area.min_x();
                    total_size / container_width * displacement
                }
                _ => {
                    let container_height = container_size.read().area.height();
                    let displacement = coordinates.y as f32 - size.read().area.min_y();
                    total_size / container_height * displacement
                }
            };

            let mut changed_panels = false;

            if displacement_per >= 0. {
                // Resizing to the right

                let mut acc_per = 0.0;

                // Resize panels to the right
                for panel in &mut registry.panels[panel_index..].iter_mut() {
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

                // Resize panels to the left
                if let Some(panel) = &mut registry.panels[0..panel_index].iter_mut().next_back() {
                    let new_size = (panel.size + acc_per).clamp(panel.min_size, 100.);

                    if panel.size != new_size {
                        changed_panels = true
                    }

                    panel.size = new_size;
                }
            } else {
                // Resizing to the left

                let mut acc_per = 0.0;

                // Resize panels to the left
                for panel in &mut registry.panels[0..panel_index].iter_mut().rev() {
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

                // Resize panels to the right
                if let Some(panel) = &mut registry.panels[panel_index..].iter_mut().next() {
                    let new_size = (panel.size - acc_per).clamp(panel.min_size, 100.);

                    if panel.size != new_size {
                        changed_panels = true
                    }

                    panel.size = new_size;
                }
            }

            if changed_panels {
                allow_resizing.set(false);
            }
            e.prevent_default();
        }
    };

    let onpointerdown = move |e: PointerEvent| {
        e.stop_propagation();
        e.prevent_default();
        clicking.set(true);
    };

    let onglobalpointerup = move |_| {
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
        onpointerdown,
        onglobalpointerup,
        onpointerenter,
        oncaptureglobalmousemove,
        onpointerleave,
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
                        min_size: 4.,
                        label {
                            "Panel 0"
                        }
                    }
                    ResizablePanel {
                        min_size: 4.,
                        ResizableContainer {
                            direction: "horizontal",
                            ResizablePanel {
                                min_size: 4.,
                                label {
                                    "Panel 2"
                                }
                            }
                            ResizablePanel {
                                min_size: 4.,
                                label {
                                    "Panel 3"
                                }
                            }
                            ResizablePanel {
                                min_size: 4.,
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
        let panel_0 = container.get(1);
        let panel_1 = container.get(3);
        let panel_2 = panel_1.get(0).get(1);
        let panel_3 = panel_1.get(0).get(3);
        let panel_4 = panel_1.get(0).get(5);

        assert_eq!(panel_0.layout().unwrap().area.height().round(), 248.0);
        assert_eq!(panel_1.layout().unwrap().area.height().round(), 248.0);
        assert_eq!(panel_2.layout().unwrap().area.width().round(), 164.0);
        assert_eq!(panel_3.layout().unwrap().area.width().round(), 164.0);
        assert_eq!(panel_4.layout().unwrap().area.width().round(), 164.0);

        // Vertical
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor: (100.0, 250.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseMove,
            cursor: (100.0, 200.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseUp,
            cursor: (0.0, 0.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(panel_0.layout().unwrap().area.height().round(), 200.0); // 250 - 50
        assert_eq!(panel_1.layout().unwrap().area.height().round(), 296.0); // 500 - 200 - 4

        // Horizontal
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseDown,
            cursor: (167.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseMove,
            cursor: (187.0, 300.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: MouseEventName::MouseUp,
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
