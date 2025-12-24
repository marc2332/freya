use std::time::Duration;

use freya_animation::prelude::*;
use freya_components::theming::hooks::get_theme_or_default;
use freya_core::prelude::*;
use torin::prelude::{
    Area,
    Position,
    Size,
};

/// A ripple effect instance
#[derive(Clone, PartialEq)]
struct RippleInstance {
    id: u64,
    x: f32,
    y: f32,
}

/// A container that shows a Material Design-style ripple effect when clicked.
///
/// The ripple expands from the click position and fades out.
///
/// ```rust
/// # use freya::{material_design::*, prelude::*};
/// fn app() -> impl IntoElement {
///     Ripple::new().child(
///         rect()
///             .width(Size::px(200.))
///             .height(Size::px(100.))
///             .background((100, 100, 200))
///             .center()
///             .child("Click me!"),
///     )
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct Ripple {
    children: Vec<Element>,
    layout: LayoutData,
    key: DiffKey,
    color: Option<Color>,
    duration: Duration,
}

impl Default for Ripple {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for Ripple {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for Ripple {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for Ripple {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for Ripple {}

impl Ripple {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            layout: LayoutData::default(),
            key: DiffKey::None,
            color: None,
            duration: Duration::from_millis(800),
        }
    }

    /// Set the color of the ripple effect.
    /// Default is white with transparency.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the duration of the ripple animation.
    /// Default is 800ms.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

impl Render for Ripple {
    fn render(&self) -> impl IntoElement {
        let mut container_area = use_state(Area::default);
        let mut ripples = use_state::<Vec<RippleInstance>>(Vec::new);
        let mut ripple_counter = use_state(|| 0u64);

        let color = self.color.unwrap_or_else(|| {
            let theme = get_theme_or_default();
            theme.colors.primary
        });
        let duration = self.duration;

        let on_mouse_down = move |e: Event<MouseEventData>| {
            let location = e.element_location;
            let id = ripple_counter();
            *ripple_counter.write() += 1;

            ripples.write().push(RippleInstance {
                id,
                x: location.x as f32,
                y: location.y as f32,
            });
        };

        let area = container_area.read();
        let max_dimension = area.width().max(area.height());

        rect()
            .layout(self.layout.clone())
            .interactive(false)
            .overflow(Overflow::Clip)
            .on_mouse_down(on_mouse_down)
            .on_sized(move |e: Event<SizedEventData>| container_area.set(e.area))
            .children(self.children.clone())
            .children_iter(ripples.read().iter().map(|ripple| {
                RippleCircle {
                    id: ripple.id,
                    x: ripple.x,
                    y: ripple.y,
                    color,
                    duration,
                    max_size: max_dimension * 2.5,
                    ripples,
                    key: DiffKey::U64(ripple.id),
                }
                .into()
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Clone, PartialEq)]
struct RippleCircle {
    id: u64,
    x: f32,
    y: f32,
    color: Color,
    duration: Duration,
    max_size: f32,
    ripples: State<Vec<RippleInstance>>,
    key: DiffKey,
}

impl KeyExt for RippleCircle {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for RippleCircle {
    fn render(&self) -> impl IntoElement {
        let id = self.id;
        let mut ripples = self.ripples;

        let animation = use_animation_with_dependencies(
            &(self.max_size, self.duration),
            move |conf, (max_size, duration)| {
                conf.on_creation(OnCreation::Run);

                (
                    // Scale animation: 0 -> max_size
                    AnimNum::new(0., *max_size)
                        .duration(*duration)
                        .function(Function::Expo)
                        .ease(Ease::Out),
                    // Opacity animation: 0.35 -> 0
                    AnimNum::new(0.35, 0.)
                        .duration(*duration)
                        .function(Function::Linear)
                        .ease(Ease::Out),
                )
            },
        );

        // Remove ripple when animation finishes
        use_side_effect(move || {
            if !*animation.is_running().read() && *animation.has_run_yet().read() {
                ripples.write().retain(|r| r.id != id);
            }
        });

        let (size, opacity) = animation.get().value();

        let half_size = size / 2.0;
        let left = self.x - half_size;
        let top = self.y - half_size;

        rect()
            .position(Position::new_absolute().left(left).top(top))
            .width(Size::px(size))
            .height(Size::px(size))
            .corner_radius(CornerRadius::new_all(size / 2.0))
            .layer(1)
            .background(self.color.with_a((opacity * 255.0) as u8))
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::U64(self.id)
    }
}
