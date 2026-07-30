use std::{
    borrow::Cow,
    time::Duration,
};

use async_io::Timer;
use freya_animation::{
    easing::Function,
    hook::{
        AnimatedValue,
        Ease,
        OnChange,
        OnCreation,
        ReadAnimatedValue,
        use_animation,
    },
    prelude::AnimNum,
};
use freya_core::prelude::*;
use torin::prelude::{
    Area,
    Size,
    Size2D,
};

use crate::{
    attached::{
        Attached,
        AttachedPosition,
    },
    context_menu::ContextMenu,
    define_theme,
    get_theme,
    menu::EDGE_MARGIN,
};

define_theme! {
    %[component]
    pub Tooltip {
        %[fields]
        color: Color,
        background: Color,
        border_fill: Color,
        /// Font family for the tooltip text; an empty string inherits the ambient font.
        font_family: String,
        font_size: f32,
        font_weight: i32,
    }
}

/// Tooltip component.
///
/// # Example
///
/// Use [Tooltip::new_text] to show plain text, laid out on a single line and wrapped
/// into a fixed-width card only once it would grow too wide:
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Tooltip::new_text("Hello, World!")
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_tooltip.png").render();
/// ```
///
/// Use [Tooltip::new] to show any element:
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Tooltip::new().child(
///         rect()
///             .horizontal()
///             .cross_align(Alignment::Center)
///             .spacing(4.)
///             .child(
///                 rect()
///                     .width(Size::px(10.))
///                     .height(Size::px(10.))
///                     .corner_radius(5.)
///                     .background(Color::GREEN),
///             )
///             .child("Connected"),
///     )
/// }
/// # let _ = app();
/// ```
///
/// # Preview
/// ![Tooltip Preview][tooltip]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("tooltip", "images/gallery_tooltip.png")
)]
#[derive(PartialEq, Clone)]
pub struct Tooltip {
    /// Theme override.
    pub(crate) theme: Option<TooltipThemePartial>,
    /// Text to show in the [Tooltip], set by [Tooltip::new_text]. Rendered instead of
    /// [`children`](Self::children), because plain text is the one content the tooltip can
    /// size for itself — see the two-phase sizing in [`Component::render`].
    text: Option<Cow<'static, str>>,
    /// Content to show in the [Tooltip].
    children: Vec<Element>,
    key: DiffKey,
}

impl Default for Tooltip {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyExt for Tooltip {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl ChildrenExt for Tooltip {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Tooltip {
    pub fn new() -> Self {
        Self {
            theme: None,
            text: None,
            children: vec![],
            key: DiffKey::None,
        }
    }

    /// Create a [Tooltip] with a text label: a single line, wrapped into a fixed-width
    /// card only when it would otherwise grow too wide.
    pub fn new_text(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: Some(text.into()),
            ..Self::new()
        }
    }
}

impl Component for Tooltip {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, TooltipThemePreference, "tooltip");
        let TooltipTheme {
            background,
            color,
            border_fill,
            font_family,
            font_size,
            font_weight,
        } = theme;

        /// Cap on the card's width: longer messages wrap instead of stretching across
        /// the window.
        const MAX_WIDTH: f32 = 340.;

        // Two-phase "max-content" sizing, for the text form only: an absolutely-positioned
        // overlay inherits its *trigger's* available width, so a wrapping label would fold
        // at that (one glyph per line off a 28px icon button) instead of at its own content.
        // So: lay the text out on a single overflow-allowed line first and measure it; only
        // when it genuinely exceeds the cap re-render as a fixed-width wrapping card (an
        // explicit width, like a menu panel's, isn't clamped by the parent). Arbitrary
        // children are left to size themselves — the tooltip can't know what they hold.
        let mut measured = use_state(|| None::<(String, f32)>);
        let text_width = self.text.as_ref().and_then(|text| {
            measured
                .read()
                .as_ref()
                .filter(|(recorded, _)| recorded == text)
                .map(|(_, width)| *width)
        });
        let wraps = text_width.is_some_and(|width| width > MAX_WIDTH);
        let text = self.text.clone();

        rect()
            .interactive(Interactive::No)
            .maybe(wraps, |el| el.width(Size::px(MAX_WIDTH)))
            // Hidden until measured, so an over-long line never flashes before wrapping.
            .opacity(if self.text.is_some() && text_width.is_none() {
                0.
            } else {
                1.
            })
            .padding((4., 10.))
            .border(
                Border::new()
                    .width(1.)
                    .alignment(BorderAlignment::Inner)
                    .fill(border_fill),
            )
            .background(background)
            .corner_radius(8.)
            .maybe(!font_family.is_empty(), |el| el.font_family(font_family))
            .font_size(font_size)
            .font_weight(font_weight)
            .color(color)
            .children(self.children.clone())
            .map(text, |el, text| {
                let shown = text.clone();
                el.child(
                    label()
                        .maybe(!wraps, |el| el.max_lines(1))
                        .on_sized(move |e: Event<SizedEventData>| {
                            // Record once per text, from the single-line pass: the wrapped
                            // re-layout reports the capped width and must not overwrite the
                            // decision.
                            if measured.peek().as_ref().is_none_or(|(t, _)| *t != text) {
                                measured.set(Some((text.to_string(), e.area.width())));
                            }
                        })
                        .line_height(1.45)
                        .text(shown),
                )
            })
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
pub struct TooltipContainer {
    tooltip: Tooltip,
    children: Vec<Element>,
    position: AttachedPosition,
    layout: LayoutData,
    delay: Duration,
    key: DiffKey,
}

impl KeyExt for TooltipContainer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for TooltipContainer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for TooltipContainer {}
impl ContainerPositionExt for TooltipContainer {}

impl ChildrenExt for TooltipContainer {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl TooltipContainer {
    pub fn new(tooltip: Tooltip) -> Self {
        Self {
            tooltip,
            children: vec![],
            position: AttachedPosition::Bottom,
            layout: LayoutData::default(),
            delay: Duration::from_millis(500),
            key: DiffKey::None,
        }
    }

    pub fn position(mut self, position: AttachedPosition) -> Self {
        self.position = position;
        self
    }

    /// Delay before the tooltip is shown once the pointer starts hovering.
    /// Defaults to 500ms.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
}

impl Component for TooltipContainer {
    fn render(&self) -> impl IntoElement {
        let mut is_hovering = use_state(|| false);
        let mut delay_task = use_state::<Option<TaskHandle>>(|| None);

        let animation = use_animation(move |conf| {
            conf.on_change(OnChange::Rerun);
            conf.on_creation(OnCreation::Finish);

            let scale = AnimNum::new(0.9, 1.)
                .time(150)
                .ease(Ease::Out)
                .function(Function::Expo);
            let opacity = AnimNum::new(0., 1.)
                .time(150)
                .ease(Ease::Out)
                .function(Function::Expo);

            if is_hovering() {
                (scale, opacity)
            } else {
                (scale.into_reversed(), opacity.into_reversed())
            }
        });

        let (scale, opacity) = animation.read().value();

        let delay = self.delay;
        let on_pointer_over = move |_| {
            if let Some(handle) = delay_task.write().take() {
                handle.cancel();
            }
            let task = spawn(async move {
                Timer::after(delay).await;
                is_hovering.set_if_modified(true);
            });
            delay_task.set(Some(task));
        };

        // Shared dismiss: leaving the trigger hides the tooltip, and so does pressing it:
        // an activated control (opening a dropdown, firing an action) must not keep its
        // tooltip floating over whatever the press revealed.
        let dismiss = move |_| {
            if let Some(handle) = delay_task.write().take() {
                handle.cancel();
            }
            is_hovering.set_if_modified(false);
        };

        let is_visible = opacity > 0. && !ContextMenu::is_open();

        // Main-axis correction: the tooltip *flips* to the opposite side of the trigger
        // when the preferred side lacks room (sliding there, which is what `Attached`'s
        // window clamp would do, would drag it over the trigger itself). The cross axis
        // needs nothing here: `Attached` clamps its overlay into the window atomically
        // with the position. The flip derives from the trigger's area and the tooltip's
        // size only, both stable, so it can't oscillate with its own repositioning.
        let mut anchor_area = use_state(|| None::<Area>);
        let mut measured = use_state(|| None::<(Area, Size2D)>);

        let position = match (anchor_area(), measured()) {
            (Some(anchor), Some((tip, root_size))) => {
                let fits = |position: AttachedPosition| match position {
                    AttachedPosition::Top => anchor.min_y() - tip.height() >= EDGE_MARGIN,
                    AttachedPosition::Bottom => {
                        anchor.max_y() + tip.height() <= root_size.height - EDGE_MARGIN
                    }
                    AttachedPosition::Left => anchor.min_x() - tip.width() >= EDGE_MARGIN,
                    AttachedPosition::Right => {
                        anchor.max_x() + tip.width() <= root_size.width - EDGE_MARGIN
                    }
                };
                let opposite = match self.position {
                    AttachedPosition::Top => AttachedPosition::Bottom,
                    AttachedPosition::Bottom => AttachedPosition::Top,
                    AttachedPosition::Left => AttachedPosition::Right,
                    AttachedPosition::Right => AttachedPosition::Left,
                };
                if !fits(self.position) && fits(opposite) {
                    opposite
                } else {
                    self.position
                }
            }
            _ => self.position,
        };

        let padding = match position {
            AttachedPosition::Top => (0., 0., 5., 0.),
            AttachedPosition::Bottom => (5., 0., 0., 0.),
            AttachedPosition::Left => (0., 5., 0., 0.),
            AttachedPosition::Right => (0., 0., 0., 5.),
        };

        rect()
            .layout(self.layout.clone())
            .a11y_focusable(false)
            .a11y_role(AccessibilityRole::Tooltip)
            .on_pointer_over(on_pointer_over)
            .on_pointer_out(dismiss)
            .on_pointer_down(dismiss)
            .child(
                Attached::new(
                    rect()
                        .on_sized(move |e: Event<SizedEventData>| {
                            anchor_area.set_if_modified(Some(e.area));
                        })
                        .children(self.children.clone()),
                )
                .position(position)
                .maybe_child(is_visible.then(|| {
                    rect()
                        .on_sized(move |e: Event<SizedEventData>| {
                            // Only the tooltip's *size* (plus the window's) feeds the
                            // flip; tracking every re-measure keeps it fresh across
                            // remounts and cannot loop: the flip never changes the
                            // tooltip's size.
                            let root_size = *Platform::get().root_size.peek();
                            measured.set_if_modified(Some((e.area, root_size)));
                        })
                        .opacity(opacity)
                        .scale(scale)
                        .padding(padding)
                        .child(self.tooltip.clone())
                })),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
