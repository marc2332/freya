use freya::prelude::*;
use freya_core::style::color::Color;

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

/// A single `name: value` row in a node inspector tab.
///
/// Use [`Property::swatch`] to append a color preview, shared by the color,
/// shadow, border and text shadow attributes.
#[derive(Clone, PartialEq)]
pub struct Property {
    name: String,
    value: String,
    swatch: Option<(Color, String)>,
}

impl Property {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            swatch: None,
        }
    }

    /// Appends a color swatch followed by `label` to the row.
    pub fn swatch(mut self, color: Color, label: impl Into<String>) -> Self {
        self.swatch = Some((color, label.into()));
        self
    }
}

impl Component for Property {
    fn render(&self) -> impl IntoElement {
        rect()
            .overflow(Overflow::Clip)
            .width(Size::fill())
            .horizontal()
            .cross_align(Alignment::center())
            .child(
                paragraph()
                    .font_size(15.)
                    .maybe(self.swatch.is_none(), |p| p.width(Size::fill()))
                    .span(Span::new(self.name.clone()).color(NAME_COLOR))
                    .span(Span::new(": ").color(SEPARATOR_COLOR))
                    .span(Span::new(self.value.clone()).color(VALUE_COLOR)),
            )
            .map(self.swatch.clone(), |row, (color, label_text)| {
                row.child(rect().width(Size::px(5.)))
                    .child(color_swatch(color))
                    .child(rect().width(Size::px(5.)))
                    .child(
                        label()
                            .font_size(15.)
                            .color(Color::from_rgb(VALUE_COLOR.0, VALUE_COLOR.1, VALUE_COLOR.2))
                            .text(label_text),
                    )
            })
    }
}

/// A `name: value` row for fills, kept separate so gradients can wrap onto
/// multiple lines instead of being clipped.
#[derive(Clone, PartialEq)]
pub struct FillProperty {
    name: String,
    fill: Fill,
}

impl FillProperty {
    pub fn new(name: impl Into<String>, fill: Fill) -> Self {
        Self {
            name: name.into(),
            fill,
        }
    }
}

impl Component for FillProperty {
    fn render(&self) -> impl IntoElement {
        paragraph()
            .line_height(1.9)
            .font_size(15.)
            .span(Span::new(self.name.to_string()).color(NAME_COLOR))
            .span(Span::new(": ").color(SEPARATOR_COLOR))
            .span(Span::new(format!("{:?}", self.fill)).color(VALUE_COLOR))
    }
}
