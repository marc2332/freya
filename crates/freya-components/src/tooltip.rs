use std::borrow::Cow;

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
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Tooltip::new("Hello, World!")
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_tooltip.png").render();
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
    /// Text to show in the [Tooltip].
    text: Cow<'static, str>,
    key: DiffKey,
}

impl KeyExt for Tooltip {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Tooltip {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            theme: None,
            text: text.into(),
            key: DiffKey::None,
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
            .child(
                label()
                    .max_lines(1)
                    .font_size(font_size)
                    .color(color)
                    .text(self.text.clone()),
            )
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
    key: DiffKey,
}

impl KeyExt for TooltipContainer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
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
            key: DiffKey::None,
        }
    }

    pub fn position(mut self, position: AttachedPosition) -> Self {
        self.position = position;
        self
    }
}

impl Component for TooltipContainer {
    fn render(&self) -> impl IntoElement {
        let mut is_hovering = use_state(|| false);

        let animation = use_animation(move |conf| {
            conf.on_change(OnChange::Rerun);
            conf.on_creation(OnCreation::Finish);

            let scale = AnimNum::new(0.8, 1.)
                .time(350)
                .ease(Ease::Out)
                .function(Function::Expo);
            let opacity = AnimNum::new(0., 1.)
                .time(350)
                .ease(Ease::Out)
                .function(Function::Expo);

            if is_hovering() {
                (scale, opacity)
            } else {
                (scale.into_reversed(), opacity.into_reversed())
            }
        });

        let (scale, opacity) = animation.read().value();

        let on_pointer_over = move |_| {
            is_hovering.set(true);
        };

        let on_pointer_out = move |_| {
            is_hovering.set(false);
        };

        let is_visible = opacity > 0. && !ContextMenu::is_open();

        let padding = match self.position {
            AttachedPosition::Top => (0., 0., 5., 0.),
            AttachedPosition::Bottom => (5., 0., 0., 0.),
            AttachedPosition::Left => (0., 5., 0., 0.),
            AttachedPosition::Right => (0., 0., 0., 5.),
        };

        rect()
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
