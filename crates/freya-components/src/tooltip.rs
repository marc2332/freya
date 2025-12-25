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
use torin::{
    prelude::{
        Alignment,
        Area,
        Direction,
    },
    size::Size,
};

use crate::{
    get_theme,
    theming::component_themes::{
        TooltipTheme,
        TooltipThemePartial,
    },
};

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
/// # }, (250., 250.).into(), "./images/gallery_tooltip.png");
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

impl Render for Tooltip {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, tooltip);
        let TooltipTheme {
            background,
            color,
            border_fill,
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
                    .font_size(14.)
                    .color(color)
                    .text(self.text.clone()),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum TooltipPosition {
    Besides,
    #[default]
    Below,
}

#[derive(PartialEq)]
pub struct TooltipContainer {
    tooltip: Tooltip,
    children: Vec<Element>,
    position: TooltipPosition,
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
            position: TooltipPosition::Below,
            key: DiffKey::None,
        }
    }

    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }
}

impl Render for TooltipContainer {
    fn render(&self) -> impl IntoElement {
        let mut is_hovering = use_state(|| false);
        let mut size = use_state(Area::default);

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

        let on_pointer_enter = move |_| {
            is_hovering.set(true);
        };

        let on_pointer_leave = move |_| {
            is_hovering.set(false);
        };

        let on_sized = move |e: Event<SizedEventData>| {
            size.set(e.area);
        };

        let direction = match self.position {
            TooltipPosition::Below => Direction::vertical(),
            TooltipPosition::Besides => Direction::horizontal(),
        };

        rect()
            .a11y_focusable(false)
            .a11y_role(AccessibilityRole::Tooltip)
            .direction(direction)
            .on_sized(on_sized)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .children(self.children.clone())
            .child(
                rect()
                    .width(Size::px(0.))
                    .height(Size::px(0.))
                    .layer(Layer::Overlay)
                    .opacity(opacity)
                    .overflow(if opacity == 0. {
                        Overflow::Clip
                    } else {
                        Overflow::None
                    })
                    .child({
                        match self.position {
                            TooltipPosition::Below => rect()
                                .width(Size::px(size.read().width()))
                                .cross_align(Alignment::Center)
                                .main_align(Alignment::Center)
                                .scale(scale)
                                .padding((5., 0., 0., 0.))
                                .child(self.tooltip.clone()),
                            TooltipPosition::Besides => rect()
                                .height(Size::px(size.read().height()))
                                .cross_align(Alignment::Center)
                                .main_align(Alignment::Center)
                                .scale(scale)
                                .padding((0., 0., 0., 5.))
                                .child(self.tooltip.clone()),
                        }
                    }),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
