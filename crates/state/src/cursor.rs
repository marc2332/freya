use std::sync::{
    Arc,
    Mutex,
};

use freya_common::{
    CompositorDirtyNodes,
    ParagraphElements,
};
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
        OwnedAttributeView,
        State,
    },
    tags::TagName,
    NodeId,
    SendAnyMap,
};
use freya_native_core_macro::partial_derive_state;

use crate::{
    CursorMode,
    CursorReference,
    CustomAttributeValues,
    HighlightMode,
    Parse,
    ParseAttribute,
    ParseError,
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

impl ParseAttribute for CursorState {
    fn parse_attribute(
        &mut self,
        attr: OwnedAttributeView<CustomAttributeValues>,
    ) -> Result<(), crate::ParseError> {
        match attr.attribute {
            AttributeName::CursorIndex => {
                if let Some(value) = attr.value.as_text() {
                    if value != "none" {
                        self.position = Some(value.parse().map_err(|_| ParseError)?);
                    }
                }
            }
            AttributeName::CursorColor => {
                if let Some(value) = attr.value.as_text() {
                    self.color = Color::parse(value)?;
                }
            }
            AttributeName::CursorMode => {
                if let Some(value) = attr.value.as_text() {
                    self.mode = CursorMode::parse(value)?;
                }
            }
            AttributeName::CursorId => {
                if let Some(value) = attr.value.as_text() {
                    self.cursor_id = Some(value.parse().map_err(|_| ParseError)?);
                }
            }
            AttributeName::Highlights => {
                if let Some(CustomAttributeValues::TextHighlights(highlights)) =
                    attr.value.as_custom()
                {
                    self.highlights = Some(highlights.clone());
                }
            }
            AttributeName::HighlightColor => {
                if let Some(value) = attr.value.as_text() {
                    self.highlight_color = Color::parse(value)?;
                }
            }
            AttributeName::HighlightMode => {
                if let Some(value) = attr.value.as_text() {
                    self.highlight_mode = HighlightMode::parse(value)?;
                }
            }
            AttributeName::CursorReference => {
                if let OwnedAttributeValue::Custom(CustomAttributeValues::CursorReference(
                    reference,
                )) = attr.value
                {
                    self.cursor_ref = Some(reference.clone());
                }
            }
            _ => {}
        }

        Ok(())
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
        let root_id = context.get::<NodeId>().unwrap();
        let paragraphs = context.get::<Arc<Mutex<ParagraphElements>>>().unwrap();
        let compositor_dirty_nodes = context.get::<Arc<Mutex<CompositorDirtyNodes>>>().unwrap();
        let mut cursor = parent.map(|(p,)| p.clone()).unwrap_or_default();

        if let Some(attributes) = node_view.attributes() {
            for attr in attributes {
                cursor.parse_safe(attr);
            }
        }
        let changed = &cursor != self;

        let is_orphan = node_view.height() == 0 && node_view.node_id() != *root_id;

        if changed && CursorMode::Editable == cursor.mode && !is_orphan {
            if let Some((tag, cursor_ref)) = node_view.tag().zip(cursor.cursor_ref.as_ref()) {
                if *tag == TagName::Paragraph {
                    paragraphs
                        .lock()
                        .unwrap()
                        .insert_paragraph(node_view.node_id(), cursor_ref.text_id)
                }
            }
            compositor_dirty_nodes
                .lock()
                .unwrap()
                .invalidate(node_view.node_id());
        }

        *self = cursor;
        changed
    }
}
