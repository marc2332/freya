use freya_core::prelude::*;
use thiserror::Error;
use torin::{
    content::Content,
    prelude::{
        Area,
        Direction,
        Length,
    },
    size::Size,
};

use crate::{
    get_theme,
    theming::component_themes::{
        ResizableHandleTheme,
        ResizableHandleThemePartial,
    },
};

/// Sizing mode for a resizable panel.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PanelSize {
    /// Fixed pixel size.
    Pixels(Length),
    /// Proportional flex weight distributed among other percentage panels.
    Percentage(Length),
}

impl PanelSize {
    pub fn px(v: f32) -> Self {
        Self::Pixels(Length::new(v))
    }

    pub fn percent(v: f32) -> Self {
        Self::Percentage(Length::new(v))
    }

    pub fn value(&self) -> f32 {
        match self {
            Self::Pixels(v) | Self::Percentage(v) => v.get(),
        }
    }

    /// Convert a raw size value to the appropriate layout [Size].
    fn to_layout_size(&self, value: f32) -> Size {
        match self {
            Self::Pixels(_) => Size::px(value),
            Self::Percentage(_) => Size::flex(value),
        }
    }

    /// The upper bound for this sizing mode.
    fn max_size(&self) -> f32 {
        match self {
            Self::Pixels(_) => f32::MAX,
            Self::Percentage(_) => 100.,
        }
    }

    /// Convert a pixel displacement into a delta in this panel's unit system.
    fn pixel_to_delta(&self, pixels: f32, flex_factor: f32) -> f32 {
        match self {
            Self::Pixels(_) => pixels,
            Self::Percentage(_) => pixels * flex_factor,
        }
    }

    /// Convert a delta in this panel's unit system back to pixels.
    fn delta_to_pixels(&self, delta: f32, flex_factor: f32) -> f32 {
        match self {
            Self::Pixels(_) => delta,
            Self::Percentage(_) => delta / flex_factor.max(f32::MIN_POSITIVE),
        }
    }
}

#[derive(Error, Debug)]
pub enum ResizableError {
    #[error("Panel does not exist")]
    PanelNotFound,
}

#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub size: f32,
    pub initial_size: f32,
    pub min_size: f32,
    pub sizing: PanelSize,
    pub id: usize,
}

#[derive(Default)]
pub struct ResizableContext {
    pub panels: Vec<Panel>,
    pub direction: Direction,
}

impl ResizableContext {
    pub const HANDLE_SIZE: f32 = 4.0;

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn panels(&mut self) -> &mut Vec<Panel> {
        &mut self.panels
    }

    pub fn push_panel(&mut self, panel: Panel, order: Option<usize>) {
        // Only redistribute among percentage panels
        if matches!(panel.sizing, PanelSize::Percentage(_)) {
            let mut buffer = panel.size;

            for panel in &mut self
                .panels
                .iter_mut()
                .filter(|p| matches!(p.sizing, PanelSize::Percentage(_)))
            {
                let resized_sized = (panel.initial_size - panel.size).min(buffer);

                if resized_sized >= 0. {
                    panel.size = (panel.size - resized_sized).max(panel.min_size);
                    let new_resized_sized = panel.initial_size - panel.size;
                    buffer -= new_resized_sized;
                }
            }
        }

        match order {
            Some(order) if order < self.panels.len() => self.panels.insert(order, panel),
            _ => self.panels.push(panel),
        }
    }

    pub fn remove_panel(&mut self, id: usize) -> Result<(), ResizableError> {
        let removed_panel = self
            .panels
            .iter()
            .copied()
            .find(|p| p.id == id)
            .ok_or(ResizableError::PanelNotFound)?;
        self.panels.retain(|e| e.id != id);

        // Only redistribute among percentage panels
        if matches!(removed_panel.sizing, PanelSize::Percentage(_)) {
            let mut buffer = removed_panel.size;

            for panel in &mut self
                .panels
                .iter_mut()
                .filter(|p| matches!(p.sizing, PanelSize::Percentage(_)))
            {
                let resized_sized = (panel.initial_size - panel.size).min(buffer);

                panel.size = (panel.size + resized_sized).max(panel.min_size);
                let new_resized_sized = panel.initial_size - panel.size;
                buffer -= new_resized_sized;
            }
        }

        Ok(())
    }

    pub fn apply_resize(
        &mut self,
        panel_index: usize,
        pixel_distance: f32,
        container_size: f32,
    ) -> bool {
        let mut changed_panels = false;

        // Precompute conversion factor between pixels and flex weight
        let handle_space = self.panels.len().saturating_sub(1) as f32 * Self::HANDLE_SIZE;
        let (px_total, flex_total) =
            self.panels
                .iter()
                .fold((0.0f32, 0.0f32), |(px, flex), p| match p.sizing {
                    PanelSize::Pixels(_) => (px + p.size, flex),
                    PanelSize::Percentage(_) => (px, flex + p.size),
                });
        let flex_factor = flex_total / (container_size - px_total - handle_space).max(1.0);

        let (corrected_pixel_distance, behind_range, forward_range) = if pixel_distance >= 0. {
            (
                pixel_distance,
                0..panel_index,
                panel_index..self.panels.len(),
            )
        } else {
            (
                -pixel_distance,
                panel_index..self.panels.len(),
                0..panel_index,
            )
        };

        let mut acc_pixels = 0.0;

        // Resize panels in the forward direction (shrink)
        for panel in &mut self.panels[forward_range].iter_mut() {
            let old_size = panel.size;
            let delta = panel
                .sizing
                .pixel_to_delta(corrected_pixel_distance, flex_factor);
            let new_size = (panel.size - delta).clamp(panel.min_size, panel.sizing.max_size());
            changed_panels |= panel.size != new_size;
            panel.size = new_size;
            acc_pixels -= panel
                .sizing
                .delta_to_pixels(new_size - old_size, flex_factor);

            if old_size > panel.min_size {
                break;
            }
        }

        // Resize panels in the behind direction (grow)
        if let Some(panel) = &mut self.panels[behind_range].iter_mut().next_back() {
            let delta = panel.sizing.pixel_to_delta(acc_pixels, flex_factor);
            let new_size = (panel.size + delta).clamp(panel.min_size, panel.sizing.max_size());
            changed_panels |= panel.size != new_size;
            panel.size = new_size;
        }

        changed_panels
    }

    pub fn reset(&mut self) {
        for panel in &mut self.panels {
            panel.size = panel.initial_size;
        }
    }
}

/// A container with resizable panels.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     ResizableContainer::new()
///         .panel(ResizablePanel::new(PanelSize::percent(50.)).child("Panel 1"))
///         .panel(ResizablePanel::new(PanelSize::percent(50.)).child("Panel 2"))
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(
/// #       ResizableContainer::new()
/// #           .panel(ResizablePanel::new(PanelSize::percent(50.)).child("Panel 1"))
/// #           .panel(ResizablePanel::new(PanelSize::percent(50.)).child("Panel 2"))
/// #   )
/// # }, "./images/gallery_resizable_container.png").render();
/// ```
///
/// # Preview
/// ![ResizableContainer Preview][resizable_container]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("resizable_container", "images/gallery_resizable_container.png"),
)]
#[derive(PartialEq, Clone)]
pub struct ResizableContainer {
    direction: Direction,
    panels: Vec<ResizablePanel>,
    controller: Option<Writable<ResizableContext>>,
}

impl Default for ResizableContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl ResizableContainer {
    pub fn new() -> Self {
        Self {
            direction: Direction::Vertical,
            panels: vec![],
            controller: None,
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn panel(mut self, panel: impl Into<Option<ResizablePanel>>) -> Self {
        if let Some(panel) = panel.into() {
            self.panels.push(panel);
        }
        self
    }

    pub fn panels_iter(mut self, panels: impl Iterator<Item = ResizablePanel>) -> Self {
        self.panels.extend(panels);
        self
    }

    pub fn controller(mut self, controller: impl Into<Writable<ResizableContext>>) -> Self {
        self.controller = Some(controller.into());
        self
    }
}

impl Component for ResizableContainer {
    fn render(&self) -> impl IntoElement {
        let mut size = use_state(Area::default);
        use_provide_context(|| size);

        let direction = use_reactive(&self.direction);
        use_provide_context(|| {
            self.controller.clone().unwrap_or_else(|| {
                let mut state = State::create(ResizableContext {
                    direction: self.direction,
                    ..Default::default()
                });

                Effect::create_sync_with_gen(move |current_gen| {
                    let direction = direction();
                    if current_gen > 0 {
                        state.write().direction = direction;
                    }
                });

                state.into_writable()
            })
        });

        rect()
            .direction(self.direction)
            .on_sized(move |e: Event<SizedEventData>| size.set(e.area))
            .expanded()
            .content(Content::flex())
            .children(self.panels.iter().enumerate().flat_map(|(i, e)| {
                if i > 0 {
                    vec![ResizableHandle::new(i).into(), e.clone().into()]
                } else {
                    vec![e.clone().into()]
                }
            }))
    }
}

#[derive(PartialEq, Clone)]
pub struct ResizablePanel {
    key: DiffKey,
    initial_size: PanelSize,
    min_size: Option<f32>,
    children: Vec<Element>,
    order: Option<usize>,
}

impl KeyExt for ResizablePanel {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for ResizablePanel {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl ResizablePanel {
    pub fn new(initial_size: PanelSize) -> Self {
        Self {
            key: DiffKey::None,
            initial_size,
            min_size: None,
            children: vec![],
            order: None,
        }
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }

    pub fn initial_size(mut self, initial_size: PanelSize) -> Self {
        self.initial_size = initial_size;
        self
    }

    /// Set the minimum size for this panel (in the same units as the panel's sizing mode).
    pub fn min_size(mut self, min_size: impl Into<f32>) -> Self {
        self.min_size = Some(min_size.into());
        self
    }

    pub fn order(mut self, order: impl Into<usize>) -> Self {
        self.order = Some(order.into());
        self
    }
}

impl Component for ResizablePanel {
    fn render(&self) -> impl IntoElement {
        let registry = use_consume::<Writable<ResizableContext>>();

        let initial_value = self.initial_size.value();
        let id = use_hook({
            let mut registry = registry.clone();
            move || {
                let id = UseId::<ResizableContext>::get_in_hook();
                let panel = Panel {
                    initial_size: initial_value,
                    size: initial_value,
                    min_size: self.min_size.unwrap_or(initial_value * 0.25),
                    sizing: self.initial_size,
                    id,
                };
                registry.write().push_panel(panel, self.order);
                id
            }
        });

        use_drop({
            let mut registry = registry.clone();
            move || {
                let _ = registry.write().remove_panel(id);
            }
        });

        let registry = registry.read();
        let index = registry
            .panels
            .iter()
            .position(|e| e.id == id)
            .unwrap_or_default();

        let Panel { size, sizing, .. } = registry.panels[index];
        let main_size = sizing.to_layout_size(size);

        let (width, height) = match registry.direction {
            Direction::Horizontal => (main_size, Size::fill()),
            Direction::Vertical => (Size::fill(), main_size),
        };

        rect()
            .a11y_role(AccessibilityRole::Pane)
            .width(width)
            .height(height)
            .overflow(Overflow::Clip)
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(DiffKey::None)
    }
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

#[derive(PartialEq)]
pub struct ResizableHandle {
    panel_index: usize,
    /// Theme override.
    pub(crate) theme: Option<ResizableHandleThemePartial>,
}

impl ResizableHandle {
    pub fn new(panel_index: usize) -> Self {
        Self {
            panel_index,
            theme: None,
        }
    }
}

impl Component for ResizableHandle {
    fn render(&self) -> impl IntoElement {
        let ResizableHandleTheme {
            background,
            hover_background,
            corner_radius,
        } = get_theme!(&self.theme, resizable_handle);
        let mut size = use_state(Area::default);
        let mut clicking = use_state(|| false);
        let mut status = use_state(HandleStatus::default);
        let registry = use_consume::<Writable<ResizableContext>>();
        let container_size = use_consume::<State<Area>>();
        let mut allow_resizing = use_state(|| false);

        let panel_index = self.panel_index;
        let direction = registry.read().direction;

        use_drop(move || {
            if *status.peek() == HandleStatus::Hovering {
                Cursor::set(CursorIcon::default());
            }
        });

        let cursor = match direction {
            Direction::Horizontal => CursorIcon::ColResize,
            _ => CursorIcon::RowResize,
        };

        let on_pointer_leave = move |_| {
            *status.write() = HandleStatus::Idle;
            if !clicking() {
                Cursor::set(CursorIcon::default());
            }
        };

        let on_pointer_enter = move |_| {
            *status.write() = HandleStatus::Hovering;
            Cursor::set(cursor);
        };

        let on_capture_global_pointer_move = {
            let mut registry = registry.clone();
            move |e: Event<PointerEventData>| {
                if *clicking.read() {
                    e.prevent_default();

                    if !*allow_resizing.read() {
                        return;
                    }

                    let coords = e.global_location();
                    let handle = size.read();
                    let container = container_size.read();
                    let mut registry = registry.write();

                    let (pixel_displacement, container_axis_size) = match registry.direction {
                        Direction::Horizontal => {
                            (coords.x as f32 - handle.min_x(), container.width())
                        }
                        Direction::Vertical => {
                            (coords.y as f32 - handle.min_y(), container.height())
                        }
                    };

                    let changed_panels =
                        registry.apply_resize(panel_index, pixel_displacement, container_axis_size);

                    if changed_panels {
                        allow_resizing.set(false);
                    }
                }
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            e.prevent_default();
            clicking.set(true);
        };

        let on_global_pointer_press = move |_: Event<PointerEventData>| {
            if *clicking.read() {
                if *status.peek() != HandleStatus::Hovering {
                    Cursor::set(CursorIcon::default());
                }
                clicking.set(false);
            }
        };

        let handle_size = Size::px(ResizableContext::HANDLE_SIZE);
        let (width, height) = match direction {
            Direction::Horizontal => (handle_size, Size::fill()),
            Direction::Vertical => (Size::fill(), handle_size),
        };

        let background = match *status.read() {
            HandleStatus::Idle if !*clicking.read() => background,
            _ => hover_background,
        };

        rect()
            .width(width)
            .height(height)
            .background(background)
            .corner_radius(corner_radius)
            .on_sized(move |e: Event<SizedEventData>| {
                size.set(e.area);
                allow_resizing.set(true);
            })
            .on_pointer_down(on_pointer_down)
            .on_global_pointer_press(on_global_pointer_press)
            .on_pointer_enter(on_pointer_enter)
            .on_capture_global_pointer_move(on_capture_global_pointer_move)
            .on_pointer_leave(on_pointer_leave)
    }
}
