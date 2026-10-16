use freya_core::prelude::*;
use torin::{
    content::Content,
    node::Node,
    prelude::{
        Area,
        Direction,
        Length,
    },
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
};

define_theme! {
    %[component]
    pub ResizableHandle {
        %[fields]
        background: Color,
        hover_background: Color,
        corner_radius: CornerRadius,
    }
}

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

    /// The upper bound for this sizing mode.
    fn max_size(&self) -> f32 {
        match self {
            Self::Pixels(_) => f32::MAX,
            Self::Percentage(_) => 100.,
        }
    }

    /// Scale factor to convert between pixels and this panel's unit system.
    fn flex_scale(&self, flex_factor: f32) -> f32 {
        match self {
            Self::Pixels(_) => 1.0,
            Self::Percentage(_) => flex_factor,
        }
    }
}

/// A snapshot of a panel's size and constraints.
#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub size: f32,
    pub initial_size: f32,
    pub min_size: f32,
    pub sizing: PanelSize,
}

impl Panel {
    fn redistribute<'a>(&self, panels: impl Iterator<Item = &'a mut Panel>, adding: bool) {
        if matches!(self.sizing, PanelSize::Pixels(_)) {
            return;
        }

        let mut buffer = self.size;
        for panel in panels.filter(|panel| matches!(panel.sizing, PanelSize::Percentage(_))) {
            let adjustment = (panel.initial_size - panel.size).min(buffer);
            if !adding || adjustment >= 0. {
                let adjustment = if adding { -adjustment } else { adjustment };
                panel.size = (panel.size + adjustment).max(panel.min_size);
                buffer -= panel.initial_size - panel.size;
            }
        }
    }
}

/// Resize state for a container. Panel membership and order come from its declarations.
#[derive(Default)]
pub struct ResizableContext {
    panels: Vec<(DiffKey, Panel)>,
    direction: Direction,
    dragging: Option<usize>,
}

impl ResizableContext {
    pub const HANDLE_SIZE: f32 = 4.0;

    pub fn direction(&self) -> Direction {
        self.direction
    }

    /// Returns a snapshot of the panels in declaration order.
    pub fn panels(&self) -> Vec<Panel> {
        self.panels.iter().map(|(_, panel)| *panel).collect()
    }

    fn reconcile(&mut self, mut panels: Vec<(DiffKey, Panel)>, direction: Direction) -> bool {
        let unchanged = self.panels.len() == panels.len()
            && self
                .panels
                .iter()
                .zip(&panels)
                .all(|((old_key, old), (key, new))| {
                    old_key == key && old.sizing == new.sizing && old.min_size == new.min_size
                });
        if unchanged && self.direction == direction {
            return false;
        }

        let mut previous = std::mem::take(&mut self.panels);
        let mut index = 0;
        while index < previous.len() {
            if panels.iter().any(|(key, _)| *key == previous[index].0) {
                index += 1;
            } else {
                let (_, removed) = previous.remove(index);
                removed.redistribute(previous.iter_mut().map(|(_, panel)| panel), false);
            }
        }

        for (key, panel) in &panels {
            if !previous.iter().any(|(old_key, _)| old_key == key) {
                panel.redistribute(previous.iter_mut().map(|(_, panel)| panel), true);
                previous.push((key.clone(), *panel));
            }
        }

        for (key, panel) in &mut panels {
            if let Some((_, old)) = previous.iter().find(|(old_key, old)| {
                old_key == key
                    && std::mem::discriminant(&old.sizing) == std::mem::discriminant(&panel.sizing)
            }) {
                panel.size = old.size;
            }
            panel.size = panel.size.clamp(panel.min_size, panel.sizing.max_size());
        }
        self.panels = panels;
        self.direction = direction;
        self.dragging = None;
        true
    }

    /// Resizes panels around a divider at the given declaration index (1..panel count).
    pub fn apply_resize(
        &mut self,
        panel_index: usize,
        pixel_distance: f32,
        container_size: f32,
    ) -> bool {
        if panel_index == 0 || panel_index >= self.panels.len() {
            return false;
        }
        let panels = &mut self.panels;
        let mut changed_panels = false;

        // Precompute conversion factor between pixels and flex weight
        let handle_space = panels.len().saturating_sub(1) as f32 * Self::HANDLE_SIZE;
        let (px_total, flex_total) =
            panels
                .iter()
                .fold((0.0, 0.0), |(px, flex): (f32, f32), (_, p)| {
                    match p.sizing {
                        PanelSize::Pixels(_) => (px + p.size, flex),
                        PanelSize::Percentage(_) => (px, flex + p.size),
                    }
                });
        let flex_factor = flex_total / (container_size - px_total - handle_space).max(1.0);

        let abs_distance = pixel_distance.abs();
        let (behind_range, forward_range) = if pixel_distance >= 0. {
            (0..panel_index, panel_index..panels.len())
        } else {
            (panel_index..panels.len(), 0..panel_index)
        };

        let mut acc_pixels = 0.0;

        // Shrink forward panels
        for (_, panel) in panels[forward_range].iter_mut() {
            let old_size = panel.size;
            let scale = panel.sizing.flex_scale(flex_factor);
            let new_size =
                (panel.size - abs_distance * scale).clamp(panel.min_size, panel.sizing.max_size());
            changed_panels |= panel.size != new_size;
            panel.size = new_size;
            acc_pixels -= (new_size - old_size) / scale.max(f32::MIN_POSITIVE);

            if old_size > panel.min_size {
                break;
            }
        }

        // Grow behind panel
        if let Some((_, panel)) = panels[behind_range].last_mut() {
            let scale = panel.sizing.flex_scale(flex_factor);
            let new_size =
                (panel.size + acc_pixels * scale).clamp(panel.min_size, panel.sizing.max_size());
            changed_panels |= panel.size != new_size;
            panel.size = new_size;
        }

        changed_panels
    }

    pub fn reset(&mut self) {
        for (_, panel) in &mut self.panels {
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
/// ```
///
/// See the [interactive components demo](https://freyaui.dev/demo).
#[derive(PartialEq, Clone)]
pub struct ResizableContainer {
    key: DiffKey,
    layout: LayoutData,
    panels: Vec<ResizablePanel>,
    controller: Option<Writable<ResizableContext>>,
}

impl Default for ResizableContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutExt for ResizableContainer {
    fn layout_mut(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for ResizableContainer {}

impl KeyExt for ResizableContainer {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ResizableContainer {
    pub fn new() -> Self {
        Self {
            key: DiffKey::None,
            layout: Node {
                width: Size::fill(),
                height: Size::fill(),
                content: Content::flex(),
                ..Default::default()
            }
            .into(),
            panels: vec![],
            controller: None,
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.layout.direction = direction;
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

        let mut controller = use_provide_context(|| {
            self.controller
                .clone()
                .unwrap_or_else(|| State::create(ResizableContext::default()).into_writable())
        });
        let declarations: Vec<_> = self
            .panels
            .iter()
            .enumerate()
            .map(|(index, panel)| {
                let initial_size = panel.initial_size.value();
                (
                    panel.key.clone().or(DiffKey::DefaultU64(index as u64)),
                    Panel {
                        size: initial_size,
                        initial_size,
                        min_size: panel.min_size.unwrap_or(initial_size * 0.25),
                        sizing: panel.initial_size,
                    },
                )
            })
            .collect();
        controller.write_if(|mut context| context.reconcile(declarations, self.layout.direction));
        let mut drag_controller = controller.clone();
        let context = controller.read();

        rect()
            .layout(self.layout.clone())
            .on_sized(move |e: Event<SizedEventData>| size.set(e.area))
            .on_global_pointer_up(move |_: Event<PointerEventData>| {
                drag_controller.write_if(|mut context| context.dragging.take().is_some());
            })
            .children(
                context
                    .panels
                    .iter()
                    .zip(&self.panels)
                    .enumerate()
                    .flat_map(|(index, ((key, state), panel))| {
                        let main_size = match state.sizing {
                            PanelSize::Pixels(_) => Size::px(state.size),
                            PanelSize::Percentage(_) => Size::flex(state.size),
                        };
                        let (width, height) = match context.direction {
                            Direction::Horizontal => (main_size, Size::fill()),
                            Direction::Vertical => (Size::fill(), main_size),
                        };

                        [
                            (index > 0)
                                .then(|| ResizableHandle::new(index).key(index).into_element()),
                            Some(
                                rect()
                                    .key(key)
                                    .a11y_role(AccessibilityRole::Pane)
                                    .width(width)
                                    .height(height)
                                    .overflow(panel.overflow)
                                    .children(panel.children.clone())
                                    .into_element(),
                            ),
                        ]
                        .into_iter()
                        .flatten()
                    }),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// A panel declaration. Its position in the container determines its order.
/// Use a key to preserve its size and child state when moving it.
#[derive(PartialEq, Clone)]
pub struct ResizablePanel {
    key: DiffKey,
    initial_size: PanelSize,
    min_size: Option<f32>,
    children: Vec<Element>,
    overflow: Overflow,
}

impl KeyExt for ResizablePanel {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for ResizablePanel {
    fn children_mut(&mut self) -> &mut Vec<Element> {
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
            overflow: Overflow::Clip,
        }
    }

    /// Sets the initial size, also used by the controller's reset operation.
    /// Existing resized sizes are retained unless the sizing mode changes.
    pub fn initial_size(mut self, initial_size: PanelSize) -> Self {
        self.initial_size = initial_size;
        self
    }

    /// Set the minimum size for this panel (in the same units as the panel's sizing mode).
    pub fn min_size(mut self, min_size: f32) -> Self {
        self.min_size = Some(min_size);
        self
    }

    /// Sets how content is clipped inside the panel bounds.
    pub fn overflow(mut self, overflow: impl Into<Overflow>) -> Self {
        self.overflow = overflow.into();
        self
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
    key: DiffKey,
    panel_index: usize,
    /// Theme override.
    pub(crate) theme: Option<ResizableHandleThemePartial>,
}

impl ResizableHandle {
    pub fn new(panel_index: usize) -> Self {
        Self {
            key: DiffKey::None,
            panel_index,
            theme: None,
        }
    }
}

impl KeyExt for ResizableHandle {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for ResizableHandle {
    fn render(&self) -> impl IntoElement {
        let ResizableHandleTheme {
            background,
            hover_background,
            corner_radius,
        } = get_theme!(
            &self.theme,
            ResizableHandleThemePreference,
            "resizable_handle"
        );
        let mut size = use_state(Area::default);
        let mut status = use_state(HandleStatus::default);
        let mut controller = use_consume::<Writable<ResizableContext>>();
        let container_size = use_consume::<State<Area>>();
        let mut allow_resizing = use_state(|| false);

        let panel_index = self.panel_index;
        let (direction, dragging) = {
            let context = controller.read();
            (context.direction, context.dragging == Some(panel_index))
        };

        let cursor = match direction {
            Direction::Horizontal => CursorIcon::ColResize,
            _ => CursorIcon::RowResize,
        };

        let on_capture_global_pointer_move = {
            let mut controller = controller.clone();
            move |e: Event<PointerEventData>| {
                if controller.peek().dragging == Some(panel_index) {
                    e.prevent_default();

                    if !*allow_resizing.read() {
                        return;
                    }

                    let coords = e.global_location();
                    let handle = size.read();
                    let container = container_size.read();
                    let mut context = controller.write();

                    let (pixel_displacement, container_axis_size) = match context.direction {
                        Direction::Horizontal => {
                            (coords.x as f32 - handle.min_x(), container.width())
                        }
                        Direction::Vertical => {
                            (coords.y as f32 - handle.min_y(), container.height())
                        }
                    };

                    let changed_panels =
                        context.apply_resize(panel_index, pixel_displacement, container_axis_size);

                    if changed_panels {
                        allow_resizing.set(false);
                    }
                }
            }
        };

        let handle_size = Size::px(ResizableContext::HANDLE_SIZE);
        let (width, height) = match direction {
            Direction::Horizontal => (handle_size, Size::fill()),
            Direction::Vertical => (Size::fill(), handle_size),
        };

        let background = match *status.read() {
            HandleStatus::Idle if !dragging => background,
            _ => hover_background,
        };

        rect()
            .width(width)
            .height(height)
            .background(background)
            .corner_radius(corner_radius)
            .cursor(cursor)
            .on_sized(move |e: Event<SizedEventData>| {
                size.set(e.area);
                allow_resizing.set(true);
            })
            .on_pointer_down(move |e: Event<PointerEventData>| {
                if !e.data().is_primary() {
                    return;
                }
                e.stop_propagation();
                e.prevent_default();
                controller.write().dragging = Some(panel_index);
            })
            .on_pointer_enter(move |_| status.set(HandleStatus::Hovering))
            .on_capture_global_pointer_move(on_capture_global_pointer_move)
            .on_pointer_leave(move |_| status.set(HandleStatus::Idle))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
