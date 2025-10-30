use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

#[derive(Clone, PartialEq)]
pub struct TickIcon {
    width: Size,
    height: Size,
    margin: Gaps,
    fill: Color,
}

impl Default for TickIcon {
    fn default() -> Self {
        Self::new()
    }
}

impl TickIcon {
    pub fn new() -> Self {
        Self {
            width: Size::px(10.),
            height: Size::px(10.),
            margin: Gaps::new_all(0.),
            fill: Color::BLACK,
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

    pub fn fill(mut self, fill: impl Into<Color>) -> Self {
        self.fill = fill.into();
        self
    }
}

impl Render for TickIcon {
    fn render(&self) -> Element {
        svg(Bytes::from_static(
            r#"
            <svg viewBox="0 0 333 263" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M304.109 0L333 28.8909L99.1812 262.71L70.2903 233.819L304.109 0Z"/>
                <path d="M0 163.53L27.1003 136.429L126.003 235.332L98.9029 262.433L0 163.53Z"/>
            </svg>
        "#
            .as_bytes(),
        ))
        .width(self.width.clone())
        .height(self.height.clone())
        .fill(self.fill)
        .into()
    }
}
