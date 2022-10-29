use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use skia_safe::Color;

use crate::parse_color;

impl ParentDepState for CursorSettings {
    type Ctx = ();
    type DepState = Self;

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "cursor_index",
            "cursor_color",
            "cursor_mode",
            "cursor_id",
        ])));

    fn reduce<'a>(
        &mut self,
        node: NodeView,
        parent: Option<&'a Self::DepState>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut cursor = parent.cloned().unwrap_or_default();

        for attr in node.attributes() {
            match attr.name {
                "cursor_index" => {
                    let text = attr.value.as_text().unwrap();
                    if text != "none" {
                        let new_cursor_index = text.parse().unwrap();
                        cursor.position = Some(new_cursor_index);
                    }
                }
                "cursor_color" => {
                    let new_cursor_color = parse_color(&attr.value.to_string());
                    if let Some(new_cursor_color) = new_cursor_color {
                        cursor.color = new_cursor_color;
                    }
                }
                "cursor_mode" => {
                    cursor.mode = parse_cursor(&attr.value.to_string());
                }
                "cursor_id" => {
                    let new_cursor_id = attr.value.to_string().parse();
                    if let Ok(new_cursor_id) = new_cursor_id {
                        cursor.id = Some(new_cursor_id);
                    }
                }
                _ => {}
            }
        }
        let changed = &cursor != self;
        *self = cursor;
        changed
    }
}

fn parse_cursor(cursor: &str) -> CursorMode {
    match cursor {
        "editable" => CursorMode::Editable,
        _ => CursorMode::None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CursorSettings {
    pub position: Option<i32>,
    pub color: Color,
    pub mode: CursorMode,
    pub id: Option<usize>,
}

impl Default for CursorSettings {
    fn default() -> Self {
        Self {
            position: None,
            color: Color::WHITE,
            mode: CursorMode::None,
            id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorMode {
    None,
    Editable,
}
