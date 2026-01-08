use std::time::Duration;

use freya_animation::prelude::{AnimDirection, AnimNum, Ease, Function, OnFinish, use_animation};
use freya_core::prelude::*;
use torin::{node::Node, prelude::Area, size::Size};

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
                height: Size::fill(),
                ..Default::default()
            }
            .into(),
            duration: Duration::from_secs(4),
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
}

impl Render for OverflowedContent {
    fn render(&self) -> impl IntoElement {
        let mut label_size = use_state(Area::default);
        let mut rect_size = use_state(Area::default);

        let rect_width = rect_size.read().width();
        let label_width = label_size.read().width();
        let does_overflow = label_width > rect_width;

        let duration = self.duration;
        let animation = use_animation(move |conf| {
            conf.on_finish(OnFinish::restart());

            AnimNum::new(0., 100.)
                .duration(duration)
                .ease(Ease::InOut)
                .function(Function::Linear)
        });

        use_side_effect_with_deps(&does_overflow, move |does_overflow| {
            if *does_overflow {
                animation.run(AnimDirection::Forward);
            }
        });

        let progress = animation.get().value();
        let offset_x = if does_overflow {
            ((label_width + rect_width) * progress / 100.) - rect_width
        } else {
            0.
        };

        rect()
            .width(self.layout.width.clone())
            .height(self.layout.height.clone())
            .offset_x(-offset_x)
            .overflow(Overflow::Clip)
            .on_sized(move |e: Event<SizedEventData>| rect_size.set(e.area))
            .child(
                rect()
                    .on_sized(move |e: Event<SizedEventData>| label_size.set(e.area))
                    .children(self.children.clone()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
