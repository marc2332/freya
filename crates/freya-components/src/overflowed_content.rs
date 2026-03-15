use std::time::Duration;

use freya_animation::prelude::{
    AnimDirection,
    AnimNum,
    Ease,
    Function,
    use_animation,
};
use freya_core::prelude::*;
use torin::{
    node::Node,
    prelude::Area,
    size::Size,
};

/// The direction in which [`OverflowedContent`] scrolls.
#[derive(Clone, PartialEq, Default)]
pub enum OverflowedContentDirection {
    /// Content enters from the right edge and scrolls to the left.
    #[default]
    RightToLeft,
    /// Content starts at the left edge and scrolls to the right.
    LeftToRight,
}

/// Where the [`OverflowedContent`] animation starts from.
#[derive(Clone, PartialEq, Default)]
pub enum OverflowedContentStart {
    /// Content starts off-screen and enters from the edge.
    #[default]
    Edge,
    /// Content starts visible at its natural position.
    Visible,
}

/// Animate the content of a container when the content overflows.
///
/// This is primarily targeted to text that can't be fully shown in small layouts.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Button::new().child(
///         OverflowedContent::new().width(Size::px(100.)).child(
///             label()
///                 .text("Freya is a cross-platform GUI library for Rust")
///                 .max_lines(1),
///         ),
///     )
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct OverflowedContent {
    children: Vec<Element>,
    layout: LayoutData,
    duration: Duration,
    direction: OverflowedContentDirection,
    start: OverflowedContentStart,
    key: DiffKey,
}

impl LayoutExt for OverflowedContent {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for OverflowedContent {}

impl Default for OverflowedContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for OverflowedContent {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for OverflowedContent {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl OverflowedContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            layout: Node {
                width: Size::fill(),
                height: Size::Inner,
                ..Default::default()
            }
            .into(),
            duration: Duration::from_secs(4),
            direction: OverflowedContentDirection::default(),
            start: OverflowedContentStart::default(),
            key: DiffKey::None,
        }
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.layout.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.layout.height = height.into();
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn direction(mut self, direction: OverflowedContentDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn right_to_left(self) -> Self {
        self.direction(OverflowedContentDirection::RightToLeft)
    }

    pub fn left_to_right(self) -> Self {
        self.direction(OverflowedContentDirection::LeftToRight)
    }

    pub fn start(mut self, start: OverflowedContentStart) -> Self {
        self.start = start;
        self
    }

    pub fn start_edge(self) -> Self {
        self.start(OverflowedContentStart::Edge)
    }

    pub fn start_visible(self) -> Self {
        self.start(OverflowedContentStart::Visible)
    }
}

impl Component for OverflowedContent {
    fn render(&self) -> impl IntoElement {
        let mut content_area = use_state(Area::default);
        let mut container_area = use_state(Area::default);

        let container_width = container_area.read().width();
        let content_width = content_area.read().width();
        let does_overflow = content_width > container_width;

        let duration = self.duration;

        let animation = use_animation(move |_| {
            AnimNum::new(0., 100.)
                .duration(duration)
                .ease(Ease::InOut)
                .function(Function::Linear)
        });

        let is_running = *animation.is_running().read();
        let has_run = *animation.has_run_yet().read();

        use_side_effect_with_deps(
            &(does_overflow, is_running, has_run),
            move |&(does_overflow, is_running, has_run)| {
                if does_overflow && (!has_run || !is_running) {
                    animation.run(AnimDirection::Forward);
                }
            },
        );

        let progress = animation.get().value();
        let is_first_cycle =
            *animation.runs().read() <= 1 && self.start == OverflowedContentStart::Visible;

        let offset_x = if does_overflow {
            match (&self.direction, is_first_cycle) {
                (OverflowedContentDirection::RightToLeft, false) => {
                    container_width - (content_width + container_width) * progress / 100.
                }
                (OverflowedContentDirection::RightToLeft, true) => {
                    -(content_width * progress / 100.)
                }
                (OverflowedContentDirection::LeftToRight, false) => {
                    (content_width + container_width) * progress / 100. - content_width
                }
                (OverflowedContentDirection::LeftToRight, true) => {
                    container_width * progress / 100.
                }
            }
        } else {
            0.
        };

        rect()
            .width(self.layout.width.clone())
            .height(self.layout.height.clone())
            .overflow(Overflow::Clip)
            .on_sized(move |e: Event<SizedEventData>| container_area.set(e.area))
            .child(
                rect()
                    .offset_x(offset_x)
                    .on_sized(move |e: Event<SizedEventData>| content_area.set(e.area))
                    .children(self.children.clone()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
