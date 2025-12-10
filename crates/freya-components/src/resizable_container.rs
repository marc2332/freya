use freya_core::prelude::*;
use thiserror::Error;
use torin::{
    content::Content,
    prelude::{
        Area,
        Direction,
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
    pub id: usize,
}

#[derive(Default)]
pub struct ResizableContext {
    pub panels: Vec<Panel>,
    pub direction: Direction,
}

impl ResizableContext {
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn panels(&mut self) -> &mut Vec<Panel> {
        &mut self.panels
    }

    pub fn push_panel(&mut self, panel: Panel, order: Option<usize>) {
        let mut buffer = panel.size;

        for panel in &mut self.panels.iter_mut() {
            let resized_sized = (panel.initial_size - panel.size).min(buffer);

            if resized_sized >= 0. {
                panel.size = (panel.size - resized_sized).max(panel.min_size);
                let new_resized_sized = panel.initial_size - panel.size;
                buffer -= new_resized_sized;
            }
        }

        if let Some(order) = order {
            if self.panels.len() <= order {
                self.panels.push(panel);
            } else {
                self.panels.insert(order, panel);
            }
        } else {
            self.panels.push(panel);
        }
    }

    pub fn remove_panel(&mut self, id: usize) -> Result<(), ResizableError> {
        let removed_panel = self
            .panels
            .iter()
            .find(|p| p.id == id)
            .cloned()
            .ok_or(ResizableError::PanelNotFound)?;
        self.panels.retain(|e| e.id != id);

        let mut buffer = removed_panel.size;

        for panel in &mut self.panels.iter_mut() {
            let resized_sized = (panel.initial_size - panel.size).min(buffer);

            panel.size = (panel.size + resized_sized).max(panel.min_size);
            let new_resized_sized = panel.initial_size - panel.size;
            buffer -= new_resized_sized;
        }

        Ok(())
    }

    pub fn apply_resize(&mut self, panel_index: usize, distance: f32) -> bool {
        let mut changed_panels = false;

        let (corrected_distance, behind_range, forward_range) = if distance >= 0. {
            (distance, 0..panel_index, panel_index..self.panels.len())
        } else {
            (-distance, panel_index..self.panels.len(), 0..panel_index)
        };

        let mut acc_per = 0.0;

        // Resize panels to the right
        for panel in &mut self.panels[forward_range].iter_mut() {
            let old_size = panel.size;
            let new_size = (panel.size - corrected_distance).clamp(panel.min_size, 100.);

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
        if let Some(panel) = &mut self.panels[behind_range].iter_mut().next_back() {
            let new_size = (panel.size + acc_per).clamp(panel.min_size, 100.);

            if panel.size != new_size {
                changed_panels = true
            }

            panel.size = new_size;
        }

        changed_panels
    }
}

#[derive(PartialEq)]
pub struct ResizableContainer {
    /// Direction of the container.
    /// Default to [Direction::Vertical].
    direction: Direction,
    /// Inner children for the [ResizableContainer()].
    panels: Vec<ResizablePanel>,
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
}

impl Render for ResizableContainer {
    fn render(&self) -> impl IntoElement {
        let mut size = use_state(Area::default);
        use_provide_context(|| size);

        use_provide_context(|| {
            State::create(ResizableContext {
                direction: self.direction,
                ..Default::default()
            })
        });

        rect()
            .direction(self.direction)
            .on_sized(move |e: Event<SizedEventData>| size.set(e.area))
            .expanded()
            .content(Content::flex())
            .children_iter(self.panels.iter().enumerate().flat_map(|(i, e)| {
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
    initial_size: f32,
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
    pub fn new(initial_size: f32) -> Self {
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

    pub fn initial_size(mut self, initial_size: impl Into<f32>) -> Self {
        self.initial_size = initial_size.into();
        self
    }

    pub fn min_size(mut self, min_size: impl Into<f32>) -> Self {
        self.min_size = Some(min_size.into());
        self
    }

    pub fn order(mut self, order: impl Into<usize>) -> Self {
        self.order = Some(order.into());
        self
    }
}

impl Render for ResizablePanel {
    fn render(&self) -> impl IntoElement {
        let mut registry = use_consume::<State<ResizableContext>>();

        let id = use_hook(|| {
            let id = UseId::<ResizableContext>::get_in_hook();

            let panel = Panel {
                initial_size: self.initial_size,
                size: self.initial_size,
                min_size: self.min_size.unwrap_or(self.initial_size * 0.25),
                id,
            };

            registry.write().push_panel(panel, self.order);

            id
        });

        use_drop(move || {
            // Safe to ignore any error as we are dropping
            let _ = registry.write().remove_panel(id);
        });

        let registry = registry.read();
        let index = registry
            .panels
            .iter()
            .position(|e| e.id == id)
            .unwrap_or_default();

        let Panel { size, .. } = registry.panels[index];

        let (width, height) = match registry.direction {
            Direction::Horizontal => (Size::flex(size), Size::fill()),
            Direction::Vertical => (Size::fill(), Size::flex(size)),
        };

        rect()
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

impl Render for ResizableHandle {
    fn render(&self) -> impl IntoElement {
        let ResizableHandleTheme {
            background,
            hover_background,
        } = get_theme!(&self.theme, resizable_handle);
        let mut size = use_state(Area::default);
        let mut clicking = use_state(|| false);
        let mut status = use_state(HandleStatus::default);
        let mut registry = use_consume::<State<ResizableContext>>();
        let container_size = use_consume::<State<Area>>();
        let mut allow_resizing = use_state(|| false);

        let panel_index = self.panel_index;

        use_drop(move || {
            if *status.peek() == HandleStatus::Hovering {
                Cursor::set(CursorIcon::default());
            }
        });

        let cursor = match registry.read().direction {
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

        let on_capture_global_mouse_move = move |e: Event<MouseEventData>| {
            if *clicking.read() {
                e.prevent_default();

                if !*allow_resizing.read() {
                    return;
                }

                let coordinates = e.global_location;
                let mut registry = registry.write();

                let total_size = registry.panels.iter().fold(0., |acc, p| acc + p.size);

                let distance = match registry.direction {
                    Direction::Horizontal => {
                        let container_width = container_size.read().width();
                        let displacement = coordinates.x as f32 - size.read().min_x();
                        total_size / container_width * displacement
                    }
                    Direction::Vertical => {
                        let container_height = container_size.read().height();
                        let displacement = coordinates.y as f32 - size.read().min_y();
                        total_size / container_height * displacement
                    }
                };

                let changed_panels = registry.apply_resize(panel_index, distance);

                if changed_panels {
                    allow_resizing.set(false);
                }
            }
        };

        let on_pointer_down = move |e: Event<PointerEventData>| {
            e.stop_propagation();
            e.prevent_default();
            clicking.set(true);
        };

        let on_global_mouse_up = move |_| {
            if *clicking.read() {
                if *status.peek() != HandleStatus::Hovering {
                    Cursor::set(CursorIcon::default());
                }
                clicking.set(false);
            }
        };

        let (width, height) = match registry.read().direction {
            Direction::Horizontal => (Size::px(4.), Size::fill()),
            Direction::Vertical => (Size::fill(), Size::px(4.)),
        };

        let background = match *status.read() {
            _ if *clicking.read() => hover_background,
            HandleStatus::Hovering => hover_background,
            HandleStatus::Idle => background,
        };

        rect()
            .width(width)
            .height(height)
            .background(background)
            .on_sized(move |e: Event<SizedEventData>| {
                size.set(e.area);
                allow_resizing.set(true);
            })
            .on_pointer_down(on_pointer_down)
            .on_global_mouse_up(on_global_mouse_up)
            .on_pointer_enter(on_pointer_enter)
            .on_capture_global_mouse_move(on_capture_global_mouse_move)
            .on_pointer_leave(on_pointer_leave)
    }
}
