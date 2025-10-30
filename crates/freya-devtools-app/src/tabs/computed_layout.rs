use freya::prelude::*;
use freya_core::integration::NodeId;

use crate::hooks::use_node_info;

#[derive(PartialEq)]
pub struct NodeInspectorComputedLayout {
    pub node_id: NodeId,
    pub window_id: u64,
}

impl Render for NodeInspectorComputedLayout {
    fn render(&self) -> Element {
        let Some(node) = use_node_info(self.node_id, self.window_id) else {
            return rect().into();
        };

        let inner_area = format!(
            "{}x{}",
            node.inner_area.width().round(),
            node.inner_area.height().round()
        );
        let area = format!(
            "{}x{}",
            node.area.width().round(),
            node.area.height().round()
        );
        let paddings = node.state.layout.padding;
        let margins = node.state.layout.margin;

        ScrollView::new()
            .show_scrollbar(true)
            .child(
                rect()
                    .padding(20.)
                    .cross_align(Alignment::center())
                    .width(Size::fill())
                    .child(
                        rect()
                            .width(Size::fill())
                            .max_width(Size::px(300.))
                            .child(
                                label()
                                    .height(Size::px(25.))
                                    .text(format!("Area: {area}"))
                            )
                            .child(
                                rect()
                                    .width(Size::fill())
                                    .height(Size::px(250.))
                                    .main_align(Alignment::center())
                                    .cross_align(Alignment::center())
                                    .background((197, 46, 139))
                                    .content(Content::Flex)
                                    .corner_radius(CornerRadius::new_all(5.))
                                    .child(
                                        // Top margin
                                        TooltipContainer::new(Tooltip::new("Top margin"))
                                            .child(
                                                label()
                                                    .text_align(TextAlign::Center)
                                                    .width(Size::px(25.))
                                                    .height(Size::px(25.))
                                                    .text(format!("{}", margins.top()))
                                            )
                                    )
                                    .child(
                                        rect()
                                            .direction(Direction::Horizontal)
                                            .height(Size::flex(1.))
                                            .width(Size::fill())
                                            .cross_align(Alignment::center())
                                            .content(Content::Flex)
                                            .child(
                                                // Left margin
                                                TooltipContainer::new(Tooltip::new("Left margin"))
                                                    .child(
                                                        label()
                                                            .text_align(TextAlign::Center)
                                                            .width(Size::px(25.))
                                                            .height(Size::px(25.))
                                                            .text(format!("{}", margins.left()))
                                                    )
                                            )
                                            .child(
                                                rect()
                                                    .width(Size::flex(1.))
                                                    .height(Size::fill())
                                                    .content(Content::Flex)
                                                    .cross_align(Alignment::Center)
                                                    .background((71, 180, 240))
                                                    .corner_radius(CornerRadius::new_all(5.))
                                                    .child(
                                                        // Top padding
                                                        TooltipContainer::new(Tooltip::new("Top padding"))
                                                            .child(
                                                                label()
                                                                    .text_align(TextAlign::Center)
                                                                    .width(Size::px(25.))
                                                                    .height(Size::px(25.))
                                                                    .text(format!("{}", paddings.top()))
                                                            )
                                                    )
                                                    .child(
                                                        rect()
                                                            .direction(Direction::Horizontal)
                                                            .height(Size::flex(1.))
                                                            .content(Content::Flex)
                                                            .cross_align(Alignment::center())
                                                            .child(
                                                                // Left padding
                                                                TooltipContainer::new(Tooltip::new("Left padding"))
                                                                    .child(
                                                                        label()
                                                                            .text_align(TextAlign::Center)
                                                                            .width(Size::px(25.))
                                                                            .height(Size::px(25.))
                                                                            .text(format!("{}", paddings.left()))
                                                                    )
                                                            )
                                                            .child(
                                                                rect()
                                                                    .width(Size::flex(1.))
                                                                    .height(Size::fill())
                                                                    .main_align(Alignment::center())
                                                                    .cross_align(Alignment::center())
                                                                    .background((40, 40, 40))
                                                                    .corner_radius(CornerRadius::new_all(5.))
                                                                    .child(
                                                                        TooltipContainer::new(Tooltip::new("Inner area"))
                                                                            .child(
                                                                                label()
                                                                                    .text(inner_area.clone())
                                                                            )
                                                                    )
                                                            )
                                                            .child(
                                                                // Right padding
                                                               TooltipContainer::new(Tooltip::new("Right padding"))
                                                                    .child(
                                                                        label()
                                                                            .text_align(TextAlign::Center)
                                                                            .width(Size::px(25.))
                                                                            .height(Size::px(25.))
                                                                            .text(format!("{}", paddings.right()))
                                                                    )
                                                            )
                                                    )
                                                    .child(
                                                        // Bottom padding
                                                       TooltipContainer::new(Tooltip::new("Bottom padding"))
                                                            .child(
                                                                label()
                                                                    .text_align(TextAlign::Center)
                                                                    .width(Size::px(25.))
                                                                    .height(Size::px(25.))
                                                                    .text(format!("{}", paddings.bottom()))
                                                            )
                                                    )
                                            )
                                            .child(
                                                // Right margin
                                                TooltipContainer::new(Tooltip::new("Right margin"))
                                                    .child(
                                                        label()
                                                            .text_align(TextAlign::Center)
                                                            .width(Size::px(25.))
                                                            .height(Size::px(25.))
                                                            .text(format!("{}", margins.right()))
                                                    )
                                            )
                                    )
                                    .child(
                                        // Bottom margin
                                        TooltipContainer::new(Tooltip::new("Bottom margin"))
                                            .child(
                                                label()
                                                            .text_align(TextAlign::Center)
                                                    .width(Size::px(25.))
                                                    .height(Size::px(25.))
                                                    .text(format!("{}", margins.bottom())),
                                            )
                                    )
                            )
                    )
            )
            .into()
    }
}
