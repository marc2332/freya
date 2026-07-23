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

use crate::{
    attached::{
        Attached,
        AttachedPosition,
    },
    context_menu::ContextMenu,
    define_theme,
    get_theme,
};

define_theme! {
    %[component]
    pub Tooltip {
        %[fields]
        color: Color,
        background: Color,
        border_fill: Color,
        font_size: f32,
    }
}

/// Tooltip component.
///
/// # Example
///
/// Use [Tooltip::new_text] to show plain text in a single line:
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
            children: vec![],
            key: DiffKey::None,
        }
    }

    /// Create a [Tooltip] with a single-line text label.
    pub fn new_text(text: impl Into<Cow<'static, str>>) -> Self {
        Self::new().child(label().max_lines(1).text(text))
    }
}

impl Component for Tooltip {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, TooltipThemePreference, "tooltip");
        let TooltipTheme {
            background,
            color,
            border_fill,
            font_size,
        } = theme;

        rect()
            .interactive(Interactive::No)
            .padding((4., 10.))
            .border(
                Border::new()
                    .width(1.)
                    .alignment(BorderAlignment::Inner)
                    .fill(border_fill),
            )
            .background(background)
            .corner_radius(8.)
            .font_size(font_size)
            .color(color)
            .children(self.children.clone())
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

        let on_pointer_out = move |_| {
            if let Some(handle) = delay_task.write().take() {
                handle.cancel();
            }
            is_hovering.set_if_modified(false);
        };

        let is_visible = opacity > 0. && !ContextMenu::is_open();

        let padding = match self.position {
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
            .on_pointer_out(on_pointer_out)
            .child(
                Attached::new(rect().children(self.children.clone()))
                    .position(self.position)
                    .maybe_child(is_visible.then(|| {
                        rect()
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
