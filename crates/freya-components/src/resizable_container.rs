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

    /// Convert a raw size value to the appropriate layout [Size].
    fn to_layout_size(self, value: f32) -> Size {
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

    /// Scale factor to convert between pixels and this panel's unit system.
    fn flex_scale(&self, flex_factor: f32) -> f32 {
        match self {
            Self::Pixels(_) => 1.0,
            Self::Percentage(_) => flex_factor,
        }
    }
}

#[derive(Error, Debug)]
pub enum ResizableError {
    #[error("Panel does not exist")]
    PanelNotFound,
}

/// What a drag did to the panels around a handle.
#[derive(Clone, Copy, Debug, Default)]
pub struct ResizeOutcome {
    /// Whether any panel's size actually changed.
    pub changed: bool,
    /// The panel the drag is pressing against its minimum, when it asked for less than that panel
    /// allows. Reported rather than silently discarded, so a caller can offer the
    /// "drag past the edge to collapse it" gesture that the clamp would otherwise hide.
    pub blocked: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub struct Panel {
    /// The size this panel is laid out at, in its own units. Derived: when the container runs out
    /// of room this is pulled below [`desired`](Self::desired), and restored as room returns.
    pub size: f32,
    /// The size the user asked for, in the panel's own units. Only a drag changes it, which is
    /// what lets a squeezed container restore every panel once it grows back.
    pub desired: f32,
    pub initial_size: f32,
    /// The clamp a *drag* obeys, in the panel's own units: pixels for [`PanelSize::Pixels`], flex
    /// weight for [`PanelSize::Percentage`].
    pub min_size: f32,
    /// The floor the *container* honours when it shrinks, always in pixels. Separate from
    /// [`min_size`](Self::min_size) because a flex weight cannot say how many pixels a panel needs.
    pub min_pixels: f32,
    /// The upper clamp a drag obeys, in the panel's own units.
    pub max_size: f32,
    pub sizing: PanelSize,
    pub id: usize,
}

pub struct ResizableContext {
    pub panels: Vec<Panel>,
    pub direction: Direction,
    /// Thickness (px) of each resize handle between panels. Defaults to
    /// [`HANDLE_SIZE`](Self::HANDLE_SIZE).
    pub handle_size: f32,
    /// Whether a handle is currently being dragged. Lets a panel tell a size it was *given* from a
    /// size it was *asked* for, so a container squeeze is not mistaken for a user resize.
    pub dragging: bool,
    /// A panel a drag has pushed far enough past its minimum to mean "collapse this". Consumed by
    /// the panel itself, which fires its `on_collapse`.
    pub collapse_request: Option<usize>,
}

impl Default for ResizableContext {
    fn default() -> Self {
        Self {
            panels: Vec::new(),
            direction: Direction::default(),
            handle_size: Self::HANDLE_SIZE,
            dragging: false,
            collapse_request: None,
        }
    }
}

impl ResizableContext {
    /// The default resize-handle thickness in pixels.
    pub const HANDLE_SIZE: f32 = 4.0;

    /// How far past a panel's minimum a drag must keep going before it counts as "collapse this"
    /// rather than "I have reached the end". Far enough that arriving at the floor never trips it,
    /// short enough that the gesture is discoverable by pulling at the edge.
    pub const COLLAPSE_OVER_DRAG: f32 = 24.0;

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

            for panel in self
                .panels
                .iter_mut()
                .filter(|p| matches!(p.sizing, PanelSize::Percentage(_)))
            {
                let resized_sized = (panel.initial_size - panel.size).min(buffer);

                if resized_sized >= 0. {
                    panel.size = (panel.size - resized_sized).max(panel.min_size);
                    // Redistribution is a structural change, not container pressure, so it moves
                    // `desired` too. Leaving it behind would have the next `reflow` undo it.
                    panel.desired = panel.size;
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

            for panel in self
                .panels
                .iter_mut()
                .filter(|p| matches!(p.sizing, PanelSize::Percentage(_)))
            {
                let resized_sized = (panel.initial_size - panel.size).min(buffer);

                panel.size = (panel.size + resized_sized).max(panel.min_size);
                panel.desired = panel.size;
                let new_resized_sized = panel.initial_size - panel.size;
                buffer -= new_resized_sized;
            }
        }

        Ok(())
    }

    /// Re-derive every panel's laid-out size from the size its user asked for and the room the
    /// container actually has.
    ///
    /// Called whenever the container is measured, so a window that shrinks squeezes the panels
    /// instead of letting them overflow it, and a window that grows back restores them. It reads
    /// [`Panel::desired`] and writes [`Panel::size`]; nothing here is a user decision, which is
    /// why it never touches `desired`.
    ///
    /// **Proportional panels surrender first**, because taking the leftover is what proportional
    /// sizing means; they stop at their [`min_pixels`](Panel::min_pixels). Only then do the pixel
    /// panels give, and they give **in equal measure** rather than in declaration order, so two
    /// side panels around a main one narrow together. Whatever a panel already sitting on its
    /// floor cannot absorb is re-shared among those that still have room.
    pub fn reflow(&mut self, container_size: f32) -> bool {
        if self.panels.is_empty() {
            return false;
        }

        // Compared against the state on entry rather than tracked as we go: the pass below starts
        // by undoing the previous squeeze, and counting that as a change would report one on every
        // layout pass for as long as the container stayed small.
        let before: Vec<f32> = self.panels.iter().map(|p| p.size).collect();

        // Pressure is re-derived from scratch every time rather than accumulated, so releasing it
        // is simply a matter of arriving at a larger `room`.
        for panel in &mut self.panels {
            panel.size = panel.desired;
        }

        let handle_space = self.panels.len().saturating_sub(1) as f32 * self.handle_size;
        let flex_floor: f32 = self
            .panels
            .iter()
            .filter(|p| matches!(p.sizing, PanelSize::Percentage(_)))
            .map(|p| p.min_pixels)
            .sum();
        let room = (container_size - handle_space - flex_floor).max(0.);

        // Only the pixel panels consume `room`; the proportional ones live in what is left over,
        // which is exactly what makes them the first to give.
        let mut giving: Vec<usize> = self
            .panels
            .iter()
            .enumerate()
            .filter(|(_, p)| matches!(p.sizing, PanelSize::Pixels(_)) && p.size > p.min_pixels)
            .map(|(i, _)| i)
            .collect();

        let pixel_total: f32 = self
            .panels
            .iter()
            .filter(|p| matches!(p.sizing, PanelSize::Pixels(_)))
            .map(|p| p.size)
            .sum();
        let mut shortfall = pixel_total - room;

        while shortfall > f32::EPSILON && !giving.is_empty() {
            let share = shortfall / giving.len() as f32;
            let mut absorbed = 0.;

            giving.retain(|&i| {
                let panel = &mut self.panels[i];
                let take = share.min(panel.size - panel.min_pixels).max(0.);
                panel.size -= take;
                absorbed += take;
                panel.size > panel.min_pixels
            });

            // Every remaining panel is on its floor: there is nothing left to give, and the
            // overflow is a paint concern from here on.
            if absorbed <= f32::EPSILON {
                break;
            }
            shortfall -= absorbed;
        }

        self.panels
            .iter()
            .zip(&before)
            .any(|(panel, before)| panel.size != *before)
    }

    pub fn apply_resize(
        &mut self,
        panel_index: usize,
        pixel_distance: f32,
        container_size: f32,
    ) -> ResizeOutcome {
        let mut changed_panels = false;
        let mut blocked = None;

        // Precompute conversion factor between pixels and flex weight
        let handle_space = self.panels.len().saturating_sub(1) as f32 * self.handle_size;
        let (px_total, flex_total) =
            self.panels
                .iter()
                .fold((0.0, 0.0), |(px, flex): (f32, f32), p| match p.sizing {
                    PanelSize::Pixels(_) => (px + p.size, flex),
                    PanelSize::Percentage(_) => (px, flex + p.size),
                });
        let flex_factor = flex_total / (container_size - px_total - handle_space).max(1.0);

        // A panel's floor, in its own units. A proportional panel's `min_size` is a flex weight,
        // which says nothing about how many pixels it is actually down to — so its real floor is
        // whichever of the two bites first, with `min_pixels` converted through the same factor.
        //
        // Getting this wrong is what made a drag cascade: the shrinking panel was judged
        // "exhausted" at an arbitrary weight unrelated to its width, and the loop moved on to the
        // panel *past* it. Dragging the sidebar then narrowed the inspector on the far side.
        let floor = |panel: &Panel| match panel.sizing {
            PanelSize::Pixels(_) => panel.min_size,
            PanelSize::Percentage(_) => panel.min_size.max(panel.min_pixels * flex_factor),
        };

        let abs_distance = pixel_distance.abs();

        // A handle sits between `panel_index - 1` and `panel_index`, and **moves exactly those
        // two**: one gives up width, the other takes it. When the giving one reaches its floor the
        // drag simply stops.
        //
        // It used to cascade — shrink the next panel along, and the next — which is where a
        // three-panel shell went wrong twice over. The ranges also ran the wrong way for a
        // negative drag (`0..panel_index` iterates *away* from the handle, and
        // `panel_index..len`'s `last()` is the container's last panel, not the handle's
        // neighbour), so pulling one side panel in resized the one on the opposite side of the
        // window. Even with those fixed, a cascade means a drag that has used up the middle
        // carries on into the far side panel, which is not what a splitter does and not what
        // anyone dragging one expects.
        let (shrink_index, grow_index) = if pixel_distance >= 0. {
            (Some(panel_index), panel_index.checked_sub(1))
        } else {
            (panel_index.checked_sub(1), Some(panel_index))
        };

        // A drag can only move as much width as the panel on the growing side can accept, so a
        // handle pushed past that panel's `max_size` does not go on shrinking its neighbour into a
        // gap nothing is filling.
        //
        // **Only a pixel panel has a real ceiling.** A proportional panel's `max_size` is a weight
        // (100 by construction, which is also where it starts), so reading it as a width limit
        // said "this panel is already full" about the one panel that is never full — and every
        // drag that grows the middle returned having done nothing at all. That is most of the
        // useful dragging in a three-panel shell, including making either side panel smaller.
        // Its weight is capped in the grow step below; the layout does the rest, because a flex
        // panel takes whatever the others leave regardless of what its weight says.
        let headroom = grow_index
            .and_then(|i| self.panels.get(i))
            .map(|panel| match panel.sizing {
                PanelSize::Percentage(_) => f32::INFINITY,
                PanelSize::Pixels(_) => {
                    let scale = panel.sizing.flex_scale(flex_factor);
                    (panel.max_size - panel.size) / scale.max(f32::MIN_POSITIVE)
                }
            })
            .unwrap_or(0.);
        let abs_distance = abs_distance.min(headroom.max(0.));
        if abs_distance <= f32::EPSILON {
            return ResizeOutcome {
                changed: false,
                blocked,
            };
        }

        let mut acc_pixels = 0.0;

        // Shrink the one panel on the side the handle is moving into.
        if let Some(panel) = shrink_index.and_then(|i| self.panels.get_mut(i)) {
            let old_size = panel.size;
            let panel_floor = floor(panel);
            let scale = panel.sizing.flex_scale(flex_factor);
            let wanted = panel.size - abs_distance * scale;
            let new_size = wanted.clamp(panel_floor.min(panel.max_size), panel.max_size);

            // The drag is asking for less than this panel allows. Reported so a caller can offer
            // a "keep pulling" gesture; the clamp itself is what stops the drag here.
            if wanted < panel_floor {
                blocked = Some(panel.id);
            }

            if panel.size != new_size {
                changed_panels = true;
                panel.size = new_size;
                // A drag is the only gesture that states an intent, so it is the only thing that
                // moves `desired` — and only for the panels it actually moved, or a drag under
                // pressure would bake a squeezed neighbour's size in as its owner's choice.
                panel.desired = new_size;
            }
            acc_pixels -= (new_size - old_size) / scale.max(f32::MIN_POSITIVE);
        }

        // Grow the panel on the other side of the handle.
        if let Some(panel) = grow_index.and_then(|i| self.panels.get_mut(i)) {
            let panel_floor = floor(panel);
            let scale = panel.sizing.flex_scale(flex_factor);
            let new_size = (panel.size + acc_pixels * scale)
                .clamp(panel_floor.min(panel.max_size), panel.max_size);
            if panel.size != new_size {
                changed_panels = true;
                panel.size = new_size;
                panel.desired = new_size;
            }
        }

        ResizeOutcome {
            changed: changed_panels,
            blocked,
        }
    }

    pub fn reset(&mut self) {
        for panel in &mut self.panels {
            panel.size = panel.initial_size;
            panel.desired = panel.initial_size;
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
    handle_size: f32,
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
            handle_size: ResizableContext::HANDLE_SIZE,
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Thickness (px) of the resize handles between panels. Defaults to `4.0`. (Ignored when an
    /// external `controller` is supplied — set it on that [`ResizableContext`] instead.)
    pub fn handle_size(mut self, handle_size: f32) -> Self {
        self.handle_size = handle_size;
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
        let mut registry = use_provide_context(|| {
            self.controller.clone().unwrap_or_else(|| {
                let mut state = State::create(ResizableContext {
                    direction: self.direction,
                    handle_size: self.handle_size,
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

        // Every measurement of the container re-derives the panels from the sizes their users
        // asked for, so shrinking squeezes them instead of letting them overflow, and growing
        // back restores them. Guarded on an actual change: `reflow` is a no-op at rest, and
        // writing regardless would wake every panel on each layout pass.
        let on_sized = move |e: Event<SizedEventData>| {
            size.set(e.area);
            let axis = match registry.peek().direction {
                Direction::Horizontal => e.area.width(),
                Direction::Vertical => e.area.height(),
            };
            registry.write_if(|mut registry| registry.reflow(axis));
        };

        rect()
            .direction(self.direction)
            .on_sized(on_sized)
            .expanded()
            .content(Content::flex())
            .children(self.panels.iter().enumerate().flat_map(|(i, e)| {
                if i > 0 {
                    vec![
                        ResizableHandle::new(i).into_element(),
                        e.clone().into_element(),
                    ]
                } else {
                    vec![e.clone().into_element()]
                }
            }))
    }
}

#[derive(PartialEq, Clone)]
pub struct ResizablePanel {
    key: DiffKey,
    initial_size: PanelSize,
    min_size: Option<f32>,
    min_pixels: Option<f32>,
    max_size: Option<f32>,
    children: Vec<Element>,
    order: Option<usize>,
    on_resized: Option<EventHandler<f32>>,
    on_collapse: Option<EventHandler<()>>,
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
            min_pixels: None,
            max_size: None,
            children: vec![],
            order: None,
            on_resized: None,
            on_collapse: None,
        }
    }

    pub fn initial_size(mut self, initial_size: PanelSize) -> Self {
        self.initial_size = initial_size;
        self
    }

    /// Set the minimum size for this panel (in the same units as the panel's sizing mode).
    pub fn min_size(mut self, min_size: f32) -> Self {
        self.min_size = Some(min_size);
        self
    }

    /// Set the maximum size for this panel (in the same units as the panel's sizing mode).
    /// Defaults to unbounded for a pixel panel and to `100.` for a proportional one.
    pub fn max_size(mut self, max_size: f32) -> Self {
        self.max_size = Some(max_size);
        self
    }

    /// The floor this panel keeps when the **container** shrinks, always in pixels.
    ///
    /// Distinct from [`min_size`](Self::min_size), which clamps a *drag* in the panel's own units:
    /// a proportional panel's minimum is a flex weight, and a flex weight cannot say how many
    /// pixels the panel needs in order to still be worth drawing. Defaults to `min_size` for a
    /// pixel panel (where the two coincide) and to `0.` for a proportional one.
    pub fn min_pixels(mut self, min_pixels: f32) -> Self {
        self.min_pixels = Some(min_pixels);
        self
    }

    pub fn order(mut self, order: impl Into<usize>) -> Self {
        self.order = Some(order.into());
        self
    }

    /// Called with the panel's new size when a **drag** resizes it.
    ///
    /// Deliberately not fired when the container squeezes the panel: that size is one the panel was
    /// given, not one its user asked for, so a caller persisting it would overwrite the remembered
    /// size every time the window narrowed.
    pub fn on_resized(mut self, f: impl Into<EventHandler<f32>>) -> Self {
        self.on_resized = Some(f.into());
        self
    }

    /// Called when a drag pushes this panel more than
    /// [`COLLAPSE_OVER_DRAG`](ResizableContext::COLLAPSE_OVER_DRAG) past its minimum.
    ///
    /// The panel does not remove itself: it reports the gesture and the caller decides, because
    /// whatever re-opens the panel is the caller's too.
    pub fn on_collapse(mut self, f: impl Into<EventHandler<()>>) -> Self {
        self.on_collapse = Some(f.into());
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
                let min_size = self.min_size.unwrap_or(initial_value * 0.25);
                let panel = Panel {
                    initial_size: initial_value,
                    size: initial_value,
                    desired: initial_value,
                    min_size,
                    // A pixel panel's two minimums are the same number in the same unit; a
                    // proportional one has no pixel opinion unless the caller states it.
                    min_pixels: self.min_pixels.unwrap_or(match self.initial_size {
                        PanelSize::Pixels(_) => min_size,
                        PanelSize::Percentage(_) => 0.,
                    }),
                    max_size: self.max_size.unwrap_or(self.initial_size.max_size()),
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

        // A collapse is raised by the handle and consumed here, so the panel's own `on_collapse`
        // stays the caller-facing API while the gesture is detected where the drag happens.
        use_side_effect({
            let mut registry = registry.clone();
            let on_collapse = self.on_collapse.clone();
            move || {
                // A panel that declares no `on_collapse` never subscribes: `read()` here would
                // enlist *every* panel in the container's notifications, and `apply_resize` writes
                // that context on each pointer-move, so a drag would re-run this effect for every
                // panel on every frame to have them all early-return.
                let Some(on_collapse) = &on_collapse else {
                    return;
                };
                if registry.read().collapse_request != Some(id) {
                    return;
                }
                // Cleared first: the handler usually unmounts this panel, and a request left
                // standing would be picked up by whichever panel inherits the id.
                registry.write().collapse_request = None;
                on_collapse.call(());
            }
        });

        let registry_read = registry.read();
        let index = registry_read
            .panels
            .iter()
            .position(|e| e.id == id)
            .unwrap_or_default();

        let Panel {
            size,
            sizing,
            min_pixels,
            ..
        } = registry_read.panels[index];
        let main_size = sizing.to_layout_size(size);
        let floor = Size::px(min_pixels);

        let (width, height, min_width, min_height) = match registry_read.direction {
            Direction::Horizontal => (main_size, Size::fill(), floor, Size::default()),
            Direction::Vertical => (Size::fill(), main_size, Size::default(), floor),
        };
        drop(registry_read);

        // Only a drag reports a size. `dragging` is read inside the handler rather than captured,
        // so the panel is not re-rendered by the flag flipping.
        let on_sized = {
            let registry = registry.clone();
            let on_resized = self.on_resized.clone();
            move |e: Event<SizedEventData>| {
                let Some(on_resized) = &on_resized else {
                    return;
                };
                if !registry.peek().dragging {
                    return;
                }
                let area = e.area;
                on_resized.call(match registry.peek().direction {
                    Direction::Horizontal => area.width(),
                    Direction::Vertical => area.height(),
                });
            }
        };

        rect()
            .a11y_role(AccessibilityRole::Pane)
            .width(width)
            .height(height)
            // The floor reaches the layout node as well as the drag clamp: `min_size` alone is only
            // an opinion about dragging, and leaves a shrinking container free to measure the panel
            // to nothing.
            .min_width(min_width)
            .min_height(min_height)
            .overflow(Overflow::Clip)
            .on_sized(on_sized)
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
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
        } = get_theme!(
            &self.theme,
            ResizableHandleThemePreference,
            "resizable_handle"
        );
        let mut size = use_state(Area::default);
        let mut clicking = use_state(|| false);
        let mut status = use_state(HandleStatus::default);
        let registry = use_consume::<Writable<ResizableContext>>();
        let container_size = use_consume::<State<Area>>();
        let mut allow_resizing = use_state(|| false);

        let panel_index = self.panel_index;
        let direction = registry.read().direction;
        let handle_px = registry.read().handle_size;

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

                    let outcome =
                        registry.apply_resize(panel_index, pixel_displacement, container_axis_size);

                    // Once a panel is clamped its handle stops following the pointer, so the
                    // displacement keeps growing: that gap is how far past the floor the user has
                    // pulled, and past the threshold it reads as "collapse this" rather than "this
                    // is as far as it goes".
                    if let Some(blocked) = outcome.blocked {
                        if pixel_displacement.abs() >= ResizableContext::COLLAPSE_OVER_DRAG {
                            registry.collapse_request = Some(blocked);
                        }
                    }

                    if outcome.changed {
                        allow_resizing.set(false);
                    }
                }
            }
        };

        let on_pointer_down = {
            let mut registry = registry.clone();
            move |e: Event<PointerEventData>| {
                if !e.data().is_primary() {
                    return;
                }
                e.stop_propagation();
                e.prevent_default();
                clicking.set(true);
                // Held for the length of the gesture so a panel can tell the size it was given
                // from the size it was asked for.
                registry.write().dragging = true;
            }
        };

        let on_global_pointer_press = {
            let mut registry = registry;
            move |_: Event<PointerEventData>| {
                if *clicking.read() {
                    if *status.peek() != HandleStatus::Hovering {
                        Cursor::set(CursorIcon::default());
                    }
                    clicking.set(false);
                }
                // Cleared outside the `clicking` guard, on any global press: a gesture that ends
                // without this handle seeing its own pointer-up — the window losing focus
                // mid-drag, say — would otherwise leave the flag set for good, and every later
                // container squeeze would then be reported to `on_resized` as a user resize,
                // which is the overwrite that event exists to prevent. `write_if` so an ordinary
                // click elsewhere costs no notification.
                registry.write_if(|mut registry| {
                    let was_dragging = registry.dragging;
                    registry.dragging = false;
                    was_dragging
                });
            }
        };

        let handle_size = Size::px(handle_px);
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

#[cfg(test)]
mod tests {
    use torin::prelude::Direction;

    use crate::resizable_container::{
        Panel,
        PanelSize,
        ResizableContext,
    };

    /// The workspace shape these tests reason about: a proportional main pane between two
    /// pixel-sized side panels, with 1px handles between them.
    fn shell(sidebar_floor: f32, inspector_floor: f32) -> ResizableContext {
        let panel = |id, sizing: PanelSize, min_pixels| Panel {
            size: sizing.value(),
            desired: sizing.value(),
            initial_size: sizing.value(),
            min_size: 0.,
            min_pixels,
            max_size: sizing.max_size(),
            sizing,
            id,
        };

        ResizableContext {
            panels: vec![
                panel(0, PanelSize::px(288.), sidebar_floor),
                panel(1, PanelSize::percent(100.), 280.),
                panel(2, PanelSize::px(292.), inspector_floor),
            ],
            direction: Direction::Horizontal,
            handle_size: 1.,
            ..Default::default()
        }
    }

    fn sizes(ctx: &ResizableContext) -> Vec<f32> {
        ctx.panels.iter().map(|p| p.size).collect()
    }

    /// With room to spare, the proportional panel absorbs everything and the side panels keep
    /// every pixel they were given.
    #[test]
    fn a_roomy_container_leaves_the_pixel_panels_alone() {
        let mut ctx = shell(40., 40.);
        ctx.reflow(1200.);
        assert_eq!(sizes(&ctx), vec![288., 100., 292.]);
    }

    /// The proportional panel gives first, because taking the leftover is what proportional
    /// sizing means. Only once it is on its floor do the pixel panels start to give, and then
    /// they give **in equal measure** rather than in declaration order.
    #[test]
    fn pixel_panels_give_equally_once_the_main_pane_is_on_its_floor() {
        let mut ctx = shell(40., 40.);
        // room = 800 - 2 handles - 280 main floor = 518, against 580px of side panels.
        ctx.reflow(800.);

        let [sidebar, _, inspector] = sizes(&ctx)[..] else {
            panic!("expected three panels")
        };
        assert_eq!(sidebar, 288. - 31.);
        assert_eq!(inspector, 292. - 31.);
        assert_eq!(
            288. - sidebar,
            292. - inspector,
            "both side panels surrender the same number of pixels"
        );
    }

    /// Whatever a panel already sitting on its floor cannot absorb is re-shared among the panels
    /// that still have room, rather than being dropped.
    #[test]
    fn a_panel_on_its_floor_reshares_what_it_cannot_absorb() {
        let mut ctx = shell(200., 40.);
        ctx.reflow(500.);

        let [sidebar, _, inspector] = sizes(&ctx)[..] else {
            panic!("expected three panels")
        };
        assert_eq!(sidebar, 200., "stops at its floor");
        assert_eq!(
            inspector, 40.,
            "and takes the share the sidebar could not give"
        );
    }

    /// Past the point where every panel is on its floor there is nothing left to give, and the
    /// remainder is a paint concern rather than a negative measurement.
    #[test]
    fn nothing_is_pushed_below_its_floor() {
        let mut ctx = shell(40., 40.);
        ctx.reflow(200.);
        assert_eq!(sizes(&ctx), vec![40., 100., 40.]);
    }

    /// Pressure is re-derived rather than accumulated, so releasing it restores every panel to
    /// the size its user last asked for. This is what stops a narrowed window from permanently
    /// eating a panel's remembered width.
    #[test]
    fn growing_the_container_restores_every_panel() {
        let mut ctx = shell(40., 40.);
        ctx.reflow(200.);
        assert_eq!(sizes(&ctx), vec![40., 100., 40.]);

        ctx.reflow(1200.);
        assert_eq!(sizes(&ctx), vec![288., 100., 292.]);
    }

    /// `reflow` is a no-op at rest, so it can run on every layout pass without waking subscribers.
    #[test]
    fn reflow_reports_no_change_when_nothing_moves() {
        let mut ctx = shell(40., 40.);
        assert!(!ctx.reflow(1200.), "already fits");
        assert!(ctx.reflow(800.), "squeezed the side panels");
        assert!(!ctx.reflow(800.), "and settles");
    }
}

#[cfg(test)]
mod drag_tests {
    use torin::prelude::Direction;

    use crate::resizable_container::{
        Panel,
        PanelSize,
        ResizableContext,
    };

    /// The workspace shape: a proportional main pane between two pixel side panels, 1px handles.
    fn shell() -> ResizableContext {
        let panel = |id, sizing: PanelSize, min_size, min_pixels, max_size| Panel {
            size: sizing.value(),
            desired: sizing.value(),
            initial_size: sizing.value(),
            min_size,
            min_pixels,
            max_size,
            sizing,
            id,
        };

        ResizableContext {
            panels: vec![
                panel(0, PanelSize::px(288.), 48., 48., 520.),
                // The middle states `min_pixels` and waives the flex-weight minimum, which is what
                // a proportional panel wanting a *pixel* floor has to do. Left at the `initial *
                // 0.25` default it would be 25 weight — about 154px here — and would outrank the
                // 48px stub without anything saying so.
                panel(1, PanelSize::percent(100.), 0., 48., 100.),
                panel(2, PanelSize::px(292.), 48., 48., 560.),
            ],
            direction: Direction::Horizontal,
            handle_size: 1.,
            ..Default::default()
        }
    }

    /// Dragging the sidebar's handle right takes width from the **middle** and from nothing else.
    ///
    /// The regression: the middle is proportional, so its `min_size` is a flex weight (25 by
    /// default) with no relation to its width. The shrink loop read that as "this panel is
    /// exhausted" long before it was, and passed the drag on to the panel beyond -- so pulling the
    /// sidebar out quietly narrowed the inspector on the far side of the window.
    #[test]
    fn dragging_a_side_panel_only_takes_from_the_middle() {
        let mut ctx = shell();
        let inspector_before = ctx.panels[2].size;

        // A long drag, in the increments a real pointer delivers.
        for _ in 0..40 {
            ctx.apply_resize(1, 5., 1200.);
        }

        assert!(
            ctx.panels[0].size > 288.,
            "the sidebar grew: {}",
            ctx.panels[0].size
        );
        assert_eq!(
            ctx.panels[2].size, inspector_before,
            "the inspector on the far side must not move"
        );
    }

    /// The same, dragging the inspector's handle the other way.
    #[test]
    fn dragging_the_other_side_panel_only_takes_from_the_middle() {
        let mut ctx = shell();
        let sidebar_before = ctx.panels[0].size;

        for _ in 0..40 {
            ctx.apply_resize(2, -5., 1200.);
        }

        assert!(
            ctx.panels[2].size > 292.,
            "the inspector grew: {}",
            ctx.panels[2].size
        );
        assert_eq!(
            ctx.panels[0].size, sidebar_before,
            "the sidebar on the far side must not move"
        );
    }

    /// **Every handle resizes in both directions.** The regression this pins: the growing side's
    /// headroom was read off `max_size` for a proportional panel too, and the middle's weight
    /// starts at its own ceiling (100) — so "grow the middle" always looked full, and dragging a
    /// side panel *smaller* silently did nothing at all.
    #[test]
    fn both_handles_resize_in_both_directions() {
        for (handle, distance, shrinks) in [
            (1usize, 5., 1usize), // sidebar handle right: the middle gives
            (1, -5., 0),          // sidebar handle left:  the sidebar gives
            (2, 5., 2),           // inspector handle right: the inspector gives
            (2, -5., 1),          // inspector handle left:  the middle gives
        ] {
            let mut ctx = shell();
            let before = ctx.panels[shrinks].size;

            for _ in 0..10 {
                ctx.apply_resize(handle, distance, 1200.);
            }

            assert!(
                ctx.panels[shrinks].size < before,
                "handle {handle} dragged {distance} must shrink panel {shrinks}: \
                 {before} -> {}",
                ctx.panels[shrinks].size
            );
        }
    }

    /// A side panel dragged in stops at its own floor and never goes under it.
    #[test]
    fn a_side_panel_stops_at_its_floor() {
        // The inspector, pulled in from its own handle.
        let mut ctx = shell();
        for _ in 0..400 {
            ctx.apply_resize(2, 5., 1200.);
        }
        assert_eq!(ctx.panels[2].size, 48., "the inspector holds its floor");

        // And the sidebar, from its.
        let mut ctx = shell();
        for _ in 0..400 {
            ctx.apply_resize(1, -5., 1200.);
        }
        assert_eq!(ctx.panels[0].size, 48., "the sidebar holds its floor");
    }

    /// With the middle already on its floor, dragging the **other** side panel moves nothing.
    ///
    /// A handle owns its two neighbours and no more. The cascade this replaces meant a drag that
    /// had used up the middle carried on into the far side panel, so a sidebar sitting at its
    /// minimum still shrank when the inspector was pulled — the panel had a minimum and the
    /// container ignored it.
    #[test]
    fn a_side_panel_at_its_floor_is_not_shrunk_by_dragging_the_other_one() {
        let mut ctx = shell();

        // Pull the sidebar's handle right until the middle has nothing left to give.
        for _ in 0..400 {
            ctx.apply_resize(1, 5., 1200.);
        }
        let sidebar = ctx.panels[0].size;

        // Now pull the inspector's handle the other way, hard.
        for _ in 0..400 {
            ctx.apply_resize(2, -5., 1200.);
        }

        assert_eq!(
            ctx.panels[0].size, sidebar,
            "the sidebar is not the inspector's handle's business, at any width"
        );
        assert!(
            ctx.panels[1].size >= ctx.panels[1].min_size,
            "and the middle stops at its own floor: {}",
            ctx.panels[1].size
        );
    }

    /// The middle's floor is the **pixel** one it states, not the flex-weight minimum it would
    /// otherwise inherit.
    ///
    /// Asserted in weight rather than pixels on purpose. A proportional panel's size converts to
    /// pixels through `flex_factor`, which is itself derived from the current sizes — so once a
    /// drag has pushed the neighbours around, "how many pixels is the middle" has no stable answer
    /// to assert against, and a pixel bound here reads as precise while measuring nothing. What is
    /// well defined is the weight, and the bug was a *weight*: left unstated, `min_size` defaults
    /// to `initial * 0.25` = 25, which silently outranks the pixel floor and ends the drag with
    /// the middle still ~154px wide in this fixture.
    #[test]
    fn the_middles_floor_is_its_pixel_one_not_the_flex_weight_default() {
        let mut ctx = shell();
        // Otherwise the sidebar's own `max_size` is what ends the drag, well before the middle's
        // floor is anywhere in the picture.
        ctx.panels[0].max_size = f32::MAX;

        for _ in 0..400 {
            ctx.apply_resize(1, 5., 1200.);
        }

        assert!(
            ctx.panels[1].size < 25.,
            "the defaulted 25 weight is not what stopped the drag: {}",
            ctx.panels[1].size
        );
        assert!(
            ctx.panels[1].size >= 0.,
            "and it never goes negative: {}",
            ctx.panels[1].size
        );
        assert_eq!(ctx.panels[2].size, 292., "the far side never moved");
    }
}
