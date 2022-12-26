use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::ParentDepState;
use dioxus_native_core_macro::sorted_str_slice;
use skia_safe::Color;

use crate::parse_color;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CursorSettings {
    pub position: Option<i32>,
    pub color: Color,
    pub mode: CursorMode,
    pub id: Option<usize>,
}

impl ParentDepState for CursorSettings {
    type Ctx = ();
    type DepState = (Self,);

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
        parent: Option<(&'a Self,)>,
        _ctx: &Self::Ctx,
    ) -> bool {
        let mut cursor = parent.map(|(p,)| p.clone()).unwrap_or_default();

        if let Some(attributes) = node.attributes() {
            for attr in attributes {
                match attr.attribute.name.as_str() {
                    "cursor_index" => {
                        let text = attr.value.as_text().unwrap();
                        if text != "none" {
                            let new_cursor_index = text.parse().unwrap();
                            cursor.position = Some(new_cursor_index);
                        }
                    }
                    "cursor_color" => {
                        if let Some(val) = attr.value.as_text() {
                            let new_cursor_color = parse_color(val);
                            if let Some(new_cursor_color) = new_cursor_color {
                                cursor.color = new_cursor_color;
                            }
                        }
                    }
                    "cursor_mode" => {
                        if let Some(val) = attr.value.as_text() {
                            cursor.mode = parse_cursor(val);
                        }
                    }
                    "cursor_id" => {
                        if let Some(val) = attr.value.as_text() {
                            if let Ok(new_cursor_id) = val.parse() {
                                cursor.id = Some(new_cursor_id);
                            }
                        }
                    }
                    _ => {}
                }
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
