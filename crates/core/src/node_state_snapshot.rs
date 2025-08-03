use freya_native_core::real_dom::NodeImmutable;

use crate::{
    dom::DioxusNode,
    states::{
        AccessibilityNodeState,
        CursorState,
        FontStyleState,
        LayoutState,
        StyleState,
        SvgState,
        TransformState,
    },
};

pub trait NodeStateSnapshot {
    fn state_snapshot(&self) -> NodeState;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub struct NodeState {
    pub cursor: CursorState,
    pub font_style: FontStyleState,
    pub size: LayoutState,
    pub style: StyleState,
    pub transform: TransformState,
    pub accessibility: AccessibilityNodeState,
    pub svg: SvgState,
}

impl NodeStateSnapshot for DioxusNode<'_> {
    fn state_snapshot(&self) -> NodeState {
        let cursor = self
            .get::<CursorState>()
            .as_deref()
            .cloned()
            .unwrap_or_default();
        let font_style = self
            .get::<FontStyleState>()
            .as_deref()
            .cloned()
            .unwrap_or_default();
        let size = self
            .get::<LayoutState>()
            .as_deref()
            .cloned()
            .unwrap_or_default();
        let style = self
            .get::<StyleState>()
            .as_deref()
            .cloned()
            .unwrap_or_default();
        let transform = self
            .get::<TransformState>()
            .as_deref()
            .cloned()
            .unwrap_or_default();
        let accessibility = self
            .get::<AccessibilityNodeState>()
            .as_deref()
            .cloned()
            .unwrap_or_default();
        let svg = self
            .get::<SvgState>()
            .as_deref()
            .cloned()
            .unwrap_or_default();

        NodeState {
            cursor,
            font_style,
            size,
            style,
            transform,
            accessibility,
            svg,
        }
    }
}
