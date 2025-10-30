use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

#[derive(Clone, PartialEq)]
pub struct ArrowIcon {
    width: Size,
    height: Size,
    margin: Gaps,
    fill: Color,
    rotate: Option<f32>,
}

impl Default for ArrowIcon {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrowIcon {
    pub fn new() -> Self {
        Self {
            width: Size::px(10.),
            height: Size::px(10.),
            margin: Gaps::new_all(0.),
            fill: Color::BLACK,
            rotate: None,
        }
    }

    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.height = height.into();
        self
    }

    pub fn margin(mut self, margin: impl Into<Gaps>) -> Self {
        self.margin = margin.into();
        self
    }

    pub fn rotate(mut self, rotate: impl Into<f32>) -> Self {
        self.rotate = Some(rotate.into());
        self
    }

    pub fn fill(mut self, fill: impl Into<Color>) -> Self {
        self.fill = fill.into();
        self
    }
}

impl Render for ArrowIcon {
    fn render(&self) -> Element {
        svg(Bytes::from_static(r#"
            <svg viewBox="0 0 18 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M7.18177 9.58579L0 2.40401L1.81823 0.585785L9 7.76756L16.1818 0.585787L18 2.40402L10.8182 9.58579L10.8185 9.58601L9.00023 11.4042L9 11.404L8.99977 11.4042L7.18154 9.58602L7.18177 9.58579Z" fill="{fill}" stroke="{fill}" stroke-width="2"/>
            </svg>
        "#.as_bytes())).rotate(self.rotate).width(self.width.clone()).height(self.height.clone()).margin(self.margin).fill(self.fill).into()
    }
}
