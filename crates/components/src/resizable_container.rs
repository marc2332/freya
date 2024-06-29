use dioxus::prelude::*;
use freya_common::NodeReferenceLayout;
use freya_elements::{
    elements as dioxus_elements,
    events::MouseEvent,
};
use freya_hooks::{
    use_node_signal,
    use_platform,
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
pub fn ResizableHandle() -> Element {
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
                let size = size.peek();

                let displacement_per = match registry.read().direction.as_str() {
                    "horizontal" => {
                        let displacement = coordinates.x as f32 - size.area.min_x();
                        100. / container_size.read().area.width() * displacement
                    }
                    _ => {
                        let displacement = coordinates.y as f32 - size.area.min_y();
                        100. / container_size.read().area.height() * displacement
                    }
                };

                let mut registry = registry.write();

                {
                    let mut prev_index = index - 1;
                    let mut prev_panel: Option<Option<&mut ResizableItem>> =
                        Some(registry.registry.get_mut(prev_index));
                    while let Some(Some(ref mut panel)) = prev_panel.take() {
                        if let Some(size) = panel.try_write_size() {
                            *size = (*size + displacement_per).clamp(0., 100.);

                            if *size <= 0. && prev_index > 0 {
                                prev_index -= 1;
                                prev_panel = Some(registry.registry.get_mut(prev_index));
                            }
                        } else {
                            prev_index -= 1;
                            prev_panel = Some(registry.registry.get_mut(prev_index));
                        }
                    }
                }

                {
                    let mut next_index = index + 1;
                    let mut next_panel: Option<Option<&mut ResizableItem>> =
                        Some(registry.registry.get_mut(next_index));
                    while let Some(Some(ref mut panel)) = next_panel.take() {
                        if let Some(size) = panel.try_write_size() {
                            *size = (*size - displacement_per).clamp(0., 100.);

                            if *size <= 0. && next_index > 0 {
                                next_index += 1;
                                next_panel = Some(registry.registry.get_mut(next_index));
                            }
                        } else {
                            next_index += 1;
                            next_panel = Some(registry.registry.get_mut(next_index));
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
        clicking.set(false);
        if *status.peek() != HandleStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    };

    let (width, height) = match registry.read().direction.as_str() {
        "horizontal" => ("4", "fill"),
        _ => ("fill", "4"),
    };

    rsx!(rect {
        reference: node_reference,
        width: "{width}",
        height: "{height}",
        background: "rgb(200, 200, 200)", // TODO: Support theming
        onmousedown,
        onglobalclick: onclick,
        onmouseenter,
        onglobalmouseover: onmouseover,
        onmouseleave,
    })
}
