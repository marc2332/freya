use freya_core::prelude::*;
use torin::gaps::Gaps;

use crate::{
    get_theme,
    icons::{
        IconTheme,
        IconThemePartial,
        IconThemePreference,
        icon_dimension,
    },
    svg_viewer::SvgViewer,
};

#[derive(Clone, PartialEq)]
pub struct TickIcon {
    layout: LayoutData,
    pub(crate) theme: Option<IconThemePartial>,
}

impl LayoutExt for TickIcon {
    fn layout_mut(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for TickIcon {}

impl Default for TickIcon {
    fn default() -> Self {
        Self::new()
    }
}

impl TickIcon {
    pub fn new() -> Self {
        Self {
            layout: LayoutData::default(),
            theme: None,
        }
    }

    pub fn margin(mut self, margin: impl Into<Gaps>) -> Self {
        self.layout.margin = margin.into();
        self
    }
}

impl Component for TickIcon {
    fn render(&self) -> impl IntoElement {
        let IconTheme { fill, size } = get_theme!(&self.theme, IconThemePreference, "icon");
        SvgViewer::new(
            r#"
            <svg viewBox="0 0 333 263" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M304.109 0L333 28.8909L99.1812 262.71L70.2903 233.819L304.109 0Z"/>
                <path d="M0 163.53L27.1003 136.429L126.003 235.332L98.9029 262.433L0 163.53Z"/>
            </svg>
        "#
            .as_bytes(),
        )
        .width(icon_dimension(&self.layout.width, size))
        .height(icon_dimension(&self.layout.height, size))
        .margin(self.layout.margin)
        .fill(fill)
    }
}
