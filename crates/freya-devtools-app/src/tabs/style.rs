use freya::prelude::*;
use freya_core::integration::NodeId;
use freya_devtools::NodeStateAttributes;

use crate::{
    components::attribute::attribute_element,
    hooks::use_node_info,
};

#[derive(PartialEq)]
pub struct NodeInspectorStyle {
    pub node_id: NodeId,
    pub window_id: u64,
}

impl Component for NodeInspectorStyle {
    fn render(&self) -> impl IntoElement {
        let Some(node) = use_node_info(self.node_id, self.window_id) else {
            return rect().into_element();
        };

        ScrollView::new()
            .children_iter(
                node.state
                    .style_attributes()
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, (name, attribute))| {
                        let background = if i % 2 == 0 {
                            Color::from_af32rgb(0.1, 255, 255, 255)
                        } else {
                            Color::TRANSPARENT
                        };

                        let element = attribute_element(name, attribute)?;

                        Some(
                            rect()
                                .key(i)
                                .background(background)
                                .padding((5., 16.))
                                .child(element)
                                .into(),
                        )
                    }),
            )
            .into()
    }
}
