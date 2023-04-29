use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::{
    node_ref::NodeView,
    prelude::{AttributeMaskBuilder, Dependancy, NodeMaskBuilder, State},
    SendAnyMap,
};
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::Color;

use crate::{parse_color, CustomAttributeValues};

#[derive(Clone, Debug, PartialEq, Eq, Component)]
pub struct CursorSettings {
    pub position: Option<i32>,
    pub color: Color,
    pub mode: CursorMode,
    pub cursor_id: Option<usize>,
    pub highlights: Option<Vec<(usize, usize)>>,
    pub highlight_color: Color,
}

#[partial_derive_state]
impl State<CustomAttributeValues> for CursorSettings {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            "cursor_index",
            "cursor_color",
            "cursor_mode",
            "cursor_id",
            "highlights",
            "highlight_color",
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut cursor = parent.map(|(p,)| p.clone()).unwrap_or_default();

        if let Some(attributes) = node_view.attributes() {
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
                                cursor.cursor_id = Some(new_cursor_id);
                            }
                        }
                    }
                    "highlights" => {
                        if let Some(CustomAttributeValues::TextHighlights(highlights)) =
                            attr.value.as_custom()
                        {
                            cursor.highlights = Some(highlights.clone());
                        }
                    }
                    "highlight_color" => {
                        if let Some(val) = attr.value.as_text() {
                            let new_highlight_color = parse_color(val);
                            if let Some(new_highlight_color) = new_highlight_color {
                                cursor.highlight_color = new_highlight_color;
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
            color: Color::BLACK,
            mode: CursorMode::None,
            cursor_id: None,
            highlights: None,
            highlight_color: Color::from_rgb(87, 108, 188),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CursorMode {
    None,
    Editable,
}
