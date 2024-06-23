use freya_common::ParagraphElements;
use freya_engine::prelude::*;
use freya_native_core::{
    attributes::AttributeName,
    exports::shipyard::Component,
    node::OwnedAttributeValue,
    node_ref::NodeView,
    prelude::{
        AttributeMaskBuilder,
        Dependancy,
        NodeMaskBuilder,
        State,
    },
    tags::TagName,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    CursorMode,
    CursorReference,
    CustomAttributeValues,
    HighlightMode,
    Parse,
};

#[derive(Clone, Debug, PartialEq, Component)]
pub struct CursorState {
    pub position: Option<i32>,
    pub color: Color,
    pub mode: CursorMode,
    pub cursor_id: Option<usize>,
    pub highlights: Option<Vec<(usize, usize)>>,
    pub highlight_color: Color,
    pub highlight_mode: HighlightMode,
    pub cursor_ref: Option<CursorReference>,
}

impl Default for CursorState {
    fn default() -> Self {
        Self {
            position: None,
            color: Color::BLACK,
            mode: CursorMode::None,
            cursor_id: None,
            highlights: None,
            highlight_color: Color::from_rgb(87, 108, 188),
            highlight_mode: HighlightMode::default(),
            cursor_ref: None,
        }
    }
}

#[partial_derive_state]
impl State<CustomAttributeValues> for CursorState {
    type ParentDependencies = (Self,);

    type ChildDependencies = ();

    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> = NodeMaskBuilder::new()
        .with_attrs(AttributeMaskBuilder::Some(&[
            AttributeName::CursorIndex,
            AttributeName::CursorColor,
            AttributeName::CursorMode,
            AttributeName::CursorId,
            AttributeName::Highlights,
            AttributeName::HighlightColor,
            AttributeName::HighlightMode,
            AttributeName::CursorReference,
        ]))
        .with_tag();

    fn update<'a>(
        &mut self,
        node_view: NodeView<CustomAttributeValues>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let paragraphs = context.get::<ParagraphElements>().unwrap();
        let mut cursor = parent.map(|(p,)| p.clone()).unwrap_or_default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                match attr.attribute {
                    AttributeName::CursorIndex => {
                        let value = attr.value.as_text().unwrap();
                        if value != "none" {
                            let new_cursor_index = value.parse().unwrap();
                            cursor.position = Some(new_cursor_index);
                        }
                    }
                    AttributeName::CursorColor => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(color) = Color::parse(value) {
                                cursor.color = color;
                            }
                        }
                    }
                    AttributeName::CursorMode => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(mode) = CursorMode::parse(value) {
                                cursor.mode = mode;
                            }
                        }
                    }
                    AttributeName::CursorId => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(id) = value.parse() {
                                cursor.cursor_id = Some(id);
                            }
                        }
                    }
                    AttributeName::Highlights => {
                        if let Some(CustomAttributeValues::TextHighlights(highlights)) =
                            attr.value.as_custom()
                        {
                            cursor.highlights = Some(highlights.clone());
                        }
                    }
                    AttributeName::HighlightColor => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(highlight_color) = Color::parse(value) {
                                cursor.highlight_color = highlight_color;
                            }
                        }
                    }
                    AttributeName::HighlightMode => {
                        if let Some(value) = attr.value.as_text() {
                            if let Ok(highlight_mode) = HighlightMode::parse(value) {
                                cursor.highlight_mode = highlight_mode;
                            }
                        }
                    }
                    AttributeName::CursorReference => {
                        if let OwnedAttributeValue::Custom(
                            CustomAttributeValues::CursorReference(reference),
                        ) = attr.value
                        {
                            cursor.cursor_ref = Some(reference.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        let changed = &cursor != self;

        if changed && CursorMode::Editable == cursor.mode {
            if let Some((tag, cursor_ref)) = node_view.tag().zip(cursor.cursor_ref.as_ref()) {
                if *tag == TagName::Paragraph {
                    paragraphs.insert_paragraph(node_view.node_id(), cursor_ref.text_id)
                }
            }
        }

        *self = cursor;
        changed
    }
}
