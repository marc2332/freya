use freya::prelude::*;
use freya_core::{
    prelude::Border,
    style::{
        color::Color,
        text_shadow::TextShadow,
    },
};

const NAME_COLOR: (u8, u8, u8) = (102, 163, 217);
const SEPARATOR_COLOR: (u8, u8, u8) = (215, 215, 215);
const VALUE_COLOR: (u8, u8, u8) = (252, 181, 172);

fn color_swatch(color: Color) -> impl IntoElement {
    rect()
        .width(Size::px(17.))
        .height(Size::px(17.))
        .corner_radius(CornerRadius::new_all(5.))
        .background(Color::WHITE)
        .padding(2.5)
        .child(
            rect()
                .corner_radius(CornerRadius::new_all(3.))
                .width(Size::fill())
                .height(Size::fill())
                .background(color),
        )
}

#[derive(Clone, PartialEq)]
pub struct Property {
    name: String,
    value: String,
}

impl Property {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl Component for Property {
    fn render(&self) -> impl IntoElement {
        rect()
            .overflow(Overflow::Clip)
            .width(Size::fill())
            .direction(Direction::Horizontal)
            .cross_align(Alignment::center())
            .child(
                paragraph()
                    .width(Size::fill())
                    .font_size(15.)
                    .span(Span::new(self.name.clone()).color(NAME_COLOR))
                    .span(Span::new(": ").color(SEPARATOR_COLOR))
                    .span(Span::new(self.value.clone()).color(VALUE_COLOR)),
            )
    }
}

#[derive(Clone, PartialEq)]
pub struct GradientProperty {
    name: String,
    fill: Fill,
}

impl GradientProperty {
    pub fn new(name: impl Into<String>, fill: Fill) -> Self {
        Self {
            name: name.into(),
            fill,
        }
    }
}

impl Component for GradientProperty {
    fn render(&self) -> impl IntoElement {
        paragraph()
            .line_height(1.9)
            .font_size(15.)
            .span(Span::new(self.name.to_string()).color(NAME_COLOR))
            .span(Span::new(": ").color(SEPARATOR_COLOR))
            .span(Span::new(format!("{:?}", self.fill)).color(VALUE_COLOR))
    }
}

#[derive(Clone, PartialEq)]
pub struct ColorProperty {
    name: String,
    color: Color,
}

impl ColorProperty {
    pub fn new(name: impl Into<String>, color: Color) -> Self {
        Self {
            name: name.into(),
            color,
        }
    }
}

impl Component for ColorProperty {
    fn render(&self) -> impl IntoElement {
        rect()
            .overflow(Overflow::Clip)
            .width(Size::fill())
            .direction(Direction::Horizontal)
            .cross_align(Alignment::center())
            .child(
                paragraph()
                    .font_size(15.)
                    .span(Span::new(self.name.clone()).color(NAME_COLOR))
                    .span(Span::new(": ").color(SEPARATOR_COLOR)),
            )
            .child(rect().width(Size::px(5.)))
            .child(color_swatch(self.color))
            .child(rect().width(Size::px(5.)))
            .child(
                label()
                    .font_size(15.)
                    .color(Color::from_rgb(VALUE_COLOR.0, VALUE_COLOR.1, VALUE_COLOR.2))
                    .text(self.color.pretty()),
            )
    }
}

#[derive(Clone, PartialEq)]
pub struct ShadowProperty {
    name: String,
    shadow: Shadow,
}

impl ShadowProperty {
    pub fn new(name: impl Into<String>, shadow: Shadow) -> Self {
        Self {
            name: name.into(),
            shadow,
        }
    }
}

impl Component for ShadowProperty {
    fn render(&self) -> impl IntoElement {
        rect()
            .overflow(Overflow::Clip)
            .width(Size::fill())
            .direction(Direction::Horizontal)
            .cross_align(Alignment::center())
            .font_size(15.)
            .children(vec![
                paragraph()
                    .span(Span::new(self.name.clone()).color(NAME_COLOR))
                    .span(Span::new(": ").color(SEPARATOR_COLOR))
                    .span(Span::new(self.shadow.to_string()).color(VALUE_COLOR))
                    .into(),
                rect().width(Size::px(5.)).into(),
                color_swatch(self.shadow.color).into_element(),
                rect().width(Size::px(5.)).into(),
                label()
                    .color(Color::from_rgb(VALUE_COLOR.0, VALUE_COLOR.1, VALUE_COLOR.2))
                    .text(format!("{:?}", self.shadow.color))
                    .into(),
            ])
    }
}

#[derive(Clone, PartialEq)]
pub struct BorderProperty {
    name: String,
    border: Border,
}

impl BorderProperty {
    pub fn new(name: impl Into<String>, border: Border) -> Self {
        Self {
            name: name.into(),
            border,
        }
    }
}

impl Component for BorderProperty {
    fn render(&self) -> impl IntoElement {
        rect()
            .overflow(Overflow::Clip)
            .width(Size::fill())
            .direction(Direction::Horizontal)
            .cross_align(Alignment::center())
            .children(vec![
                paragraph()
                    .font_size(15.)
                    .span(Span::new(self.name.clone()).color(NAME_COLOR))
                    .span(Span::new(": ").color(SEPARATOR_COLOR))
                    .span(Span::new(self.border.pretty()).color(VALUE_COLOR))
                    .into(),
                rect().width(Size::px(5.)).into(),
                color_swatch(self.border.fill).into_element(),
                rect().width(Size::px(5.)).into(),
                label()
                    .font_size(15.)
                    .color(Color::from_rgb(VALUE_COLOR.0, VALUE_COLOR.1, VALUE_COLOR.2))
                    .text(self.border.fill.pretty())
                    .into(),
            ])
    }
}

#[derive(Clone, PartialEq)]
pub struct TextShadowProperty {
    name: String,
    text_shadow: TextShadow,
}

impl TextShadowProperty {
    pub fn new(name: impl Into<String>, text_shadow: TextShadow) -> Self {
        Self {
            name: name.into(),
            text_shadow,
        }
    }
}

impl Component for TextShadowProperty {
    fn render(&self) -> impl IntoElement {
        let color = self.text_shadow.color;
        let value = format!(
            "{} {} {}",
            self.text_shadow.offset.0, self.text_shadow.offset.1, self.text_shadow.blur_sigma
        );

        rect()
            .width(Size::fill())
            .direction(Direction::Horizontal)
            .cross_align(Alignment::center())
            .font_size(15.)
            .children(vec![
                paragraph()
                    .span(Span::new(self.name.to_string()).color(NAME_COLOR))
                    .span(Span::new(": ").color(SEPARATOR_COLOR))
                    .span(Span::new(value).color(VALUE_COLOR))
                    .into(),
                rect().width(Size::px(5.)).into(),
                color_swatch(color).into_element(),
                rect().width(Size::px(5.)).into(),
                label()
                    .color(Color::from_rgb(VALUE_COLOR.0, VALUE_COLOR.1, VALUE_COLOR.2))
                    .text(format!("{:?}", color))
                    .into(),
            ])
    }
}
