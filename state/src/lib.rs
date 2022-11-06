use dioxus_native_core::state::{NodeDepState, ParentDepState, State};
use dioxus_native_core_macro::State;
use skia_safe::Color;

mod cursor;
mod font_style;
mod references;
mod scroll;
mod size;
mod style;

pub use cursor::*;
pub use font_style::*;
pub use references::*;
pub use scroll::*;
pub use size::*;
pub use style::*;

#[derive(Clone, State, Default)]
pub struct NodeState {
    #[parent_dep_state(cursor_settings)]
    pub cursor_settings: CursorSettings,
    #[parent_dep_state(references)]
    pub references: References,
    #[parent_dep_state(size, Arc<Mutex<LayoutManager>>)]
    pub size: Size,
    #[parent_dep_state(scroll, Arc<Mutex<LayoutManager>>)]
    pub scroll: Scroll,
    #[node_dep_state()]
    pub style: Style,
    #[parent_dep_state(font_style)]
    pub font_style: FontStyle,
}

impl NodeState {
    pub fn set_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

pub fn parse_rgb(color: &str) -> Option<Color> {
    let color = color.replace("rgb(", "").replace(')', "");
    let mut colors = color.split(',');

    let r = colors.next()?.trim().parse().ok()?;
    let g = colors.next()?.trim().parse().ok()?;
    let b = colors.next()?.trim().parse().ok()?;
    let a: Option<&str> = colors.next();
    if let Some(a) = a {
        let a = a.trim().parse::<u8>().ok()?;
        Some(Color::from_argb(a, r, g, b))
    } else {
        Some(Color::from_rgb(r, g, b))
    }
}

pub fn parse_color(color: &str) -> Option<Color> {
    match color {
        "red" => Some(Color::RED),
        "green" => Some(Color::GREEN),
        "blue" => Some(Color::BLUE),
        "yellow" => Some(Color::YELLOW),
        "black" => Some(Color::BLACK),
        "gray" => Some(Color::GRAY),
        "white" => Some(Color::WHITE),
        _ => parse_rgb(color),
    }
}
