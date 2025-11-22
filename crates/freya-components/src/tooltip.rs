use std::borrow::Cow;

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
    pub text: Cow<'static, str>,
}

impl Tooltip {
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            theme: None,
            text: text.into(),
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
            .direction(direction)
            .on_sized(on_sized)
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .children(self.children.clone())
            .child(
                rect()
                    .width(Size::px(0.))
                    .height(Size::px(0.))
                    .layer(1500)
                    .maybe_child(if *is_hovering.read() {
                        Some(match self.position {
                            TooltipPosition::Below => rect()
                                .width(Size::px(size.read().width()))
                                .cross_align(Alignment::Center)
                                .main_align(Alignment::Center)
                                .padding((5., 0., 0., 0.))
                                .child(self.tooltip.clone()),
                            TooltipPosition::Besides => rect()
                                .height(Size::px(size.read().height()))
                                .cross_align(Alignment::Center)
                                .main_align(Alignment::Center)
                                .padding((0., 0., 0., 5.))
                                .child(self.tooltip.clone()),
                        })
                    } else {
                        None
                    }),
            )
    }
}
