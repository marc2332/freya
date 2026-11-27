use std::time::Duration;

use freya_animation::prelude::*;
use freya_components::{
    theming::hooks::get_theme_or_default,
    tile::Tile,
};
use freya_core::prelude::*;
use freya_engine::prelude::Paint;
use torin::prelude::{
    Area,
    Point2D,
    Position,
    Size,
    Size2D,
};

struct RippleInstance {
    id: u64,
    center: Point2D,
}

/// Adds a ripple overlay to components whose rendered content supports it.
pub trait RippleExt: ComponentContent + Sized {
    /// Add a ripple without changing the component's content layout.
    fn ripple(self) -> RippleComponent<Self>;
}

impl<T> RippleExt for T
where
    T: ComponentContent,
    T::Content: AccessibilityExt + ChildrenExt + EventHandlersExt + LayoutExt + StyleExt,
{
    fn ripple(self) -> RippleComponent<Self> {
        RippleComponent {
            component: self,
            effect: RippleEffect::default(),
        }
    }
}

/// A component decorated with a ripple overlay.
#[derive(Clone, PartialEq)]
pub struct RippleComponent<T> {
    component: T,
    effect: RippleEffect,
}

impl<T: ChildrenExt> ChildrenExt for RippleComponent<T> {
    fn children_mut(&mut self) -> &mut Vec<Element> {
        self.component.children_mut()
    }
}

impl<T: KeyExt> KeyExt for RippleComponent<T> {
    fn key_mut(&mut self) -> &mut DiffKey {
        self.component.key_mut()
    }
}

impl<T> RippleComponent<T> {
    /// Set the ripple color. Defaults to the theme's primary color.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.effect.color = Some(color.into());
        self
    }

    /// Set the animation duration. Defaults to 800ms.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.effect.duration = duration;
        self
    }
}

impl RippleComponent<Tile> {
    /// Set the tile's leading element.
    pub fn leading(mut self, leading: impl Into<Element>) -> Self {
        self.component = self.component.leading(leading);
        self
    }
}

impl<T> Component for RippleComponent<T>
where
    T: ComponentContent,
    T::Content: AccessibilityExt + ChildrenExt + EventHandlersExt + LayoutExt + StyleExt,
{
    fn render(&self) -> impl IntoElement {
        self.effect.render_on(self.component.render_content())
    }

    fn render_key(&self) -> DiffKey {
        self.component.render_key()
    }
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
    effect: RippleEffect,
}

impl Default for Ripple {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for Ripple {
    fn children_mut(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for Ripple {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for Ripple {
    fn layout_mut(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for Ripple {}

impl ContainerWithContentExt for Ripple {}

impl Ripple {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            layout: LayoutData::default(),
            key: DiffKey::None,
            effect: RippleEffect::default(),
        }
    }

    /// Set the color of the ripple effect.
    /// Defaults to the theme's primary color.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.effect.color = Some(color.into());
        self
    }

    /// Set the duration of the ripple animation.
    /// Default is 800ms.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.effect.duration = duration;
        self
    }
}

#[derive(Clone, PartialEq)]
struct RippleEffect {
    color: Option<Color>,
    duration: Duration,
}

impl Default for RippleEffect {
    fn default() -> Self {
        Self {
            color: None,
            duration: Duration::from_millis(800),
        }
    }
}

impl RippleEffect {
    fn render_on<T>(&self, mut el: T) -> T
    where
        T: AccessibilityExt + ChildrenExt + EventHandlersExt + LayoutExt + StyleExt,
    {
        let enabled = !el.accessibility_data_mut().builder.is_disabled();
        let corner_radius = el.style_mut().corner_radius;
        let layout = &el.layout_mut().layout;
        let offset = Point2D::new(
            layout.padding.left() + layout.offset_x.get(),
            layout.padding.top() + layout.offset_y.get(),
        );
        let mut target_area = use_state(Area::zero);
        let mut ripples = use_state::<Vec<RippleInstance>>(Vec::new);
        let mut ripple_counter = use_state(|| 0u64);

        let color = self.color.unwrap_or_else(|| {
            let theme = get_theme_or_default();
            theme.read().colors.primary
        });
        let duration = self.duration;

        let on_pointer_down = move |e: Event<PointerEventData>| {
            let id = ripple_counter();
            *ripple_counter.write() += 1;

            let center = e.global_location().cast::<f32>() - target_area.peek().origin.to_vector();
            ripples.write().push(RippleInstance { id, center });
        };

        let size = target_area().size;
        let max_size = size.width.max(size.height) * 2.5;

        if enabled {
            el = el.on_pointer_down(EventHandler::new(on_pointer_down).stacked());
        }
        el.on_sized(
            EventHandler::new(move |e: Event<SizedEventData>| {
                target_area.set_if_modified(e.visible_area);
            })
            .stacked(),
        )
        .children(ripples.read().iter().map(|ripple| RippleCircle {
            id: ripple.id,
            center: ripple.center,
            offset,
            size,
            corner_radius,
            color,
            duration,
            max_size,
            ripples,
        }))
    }
}

impl Component for Ripple {
    fn render(&self) -> impl IntoElement {
        self.effect.render_on(
            rect()
                .layout(self.layout.clone())
                .overflow(Overflow::Clip)
                .children(self.children.clone()),
        )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
struct RippleCircle {
    id: u64,
    center: Point2D,
    offset: Point2D,
    size: Size2D,
    corner_radius: CornerRadius,
    color: Color,
    duration: Duration,
    max_size: f32,
    ripples: State<Vec<RippleInstance>>,
}

impl Component for RippleCircle {
    fn render(&self) -> impl IntoElement {
        let id = self.id;
        let mut ripples = self.ripples;

        let animation = use_animation_with_dependencies(
            &(self.max_size, self.duration),
            move |conf, (max_size, duration)| {
                conf.on_creation(OnCreation::Run);

                (
                    AnimNum::new(0., *max_size)
                        .duration(*duration)
                        .function(Function::Expo)
                        .ease(Ease::Out),
                    AnimNum::new(0.35, 0.)
                        .duration(*duration)
                        .function(Function::Linear)
                        .ease(Ease::Out),
                )
            },
        );

        use_side_effect(move || {
            if !*animation.is_running().read() && *animation.has_run_yet().read() {
                ripples.write().retain(|r| r.id != id);
            }
        });

        let (size, opacity) = animation.get().value();
        let center = self.center;
        let color = self.color.with_a((opacity * 255.0) as u8);

        rect()
            .interactive(false)
            .position(
                Position::new_absolute()
                    .left(-self.offset.x)
                    .top(-self.offset.y),
            )
            .width(Size::px(self.size.width))
            .height(Size::px(self.size.height))
            .corner_radius(self.corner_radius)
            .layer(1)
            .background(RenderCallback::new(move |context| {
                if let RenderTarget::Background { canvas, .. } = &context.target {
                    let mut paint = Paint::default();
                    paint.set_anti_alias(true);
                    paint.set_color(color);
                    canvas.draw_circle((center.x, center.y), size / 2., &paint);
                }
            }))
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::U64(self.id)
    }
}
