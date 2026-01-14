use freya::prelude::*;
use torin::gaps::Gaps;

pub fn computed_layout(inner_area: String, padding: Gaps, margin: Gaps) -> impl IntoElement {
    rect().width(Size::fill()).max_width(Size::px(300.)).child(
        rect()
            .width(Size::fill())
            .height(Size::px(220.))
            .main_align(Alignment::center())
            .cross_align(Alignment::center())
            .background((40, 40, 40))
            .content(Content::Flex)
            .corner_radius(CornerRadius::new_all(5.))
            .child(
                TooltipContainer::new(Tooltip::new("Top margin")).child(
                    label()
                        .text_align(TextAlign::Center)
                        .width(Size::px(25.))
                        .height(Size::px(25.))
                        .text(format!("{}", margin.top())),
                ),
            )
            .child(
                rect()
                    .direction(Direction::Horizontal)
                    .height(Size::flex(1.))
                    .width(Size::fill())
                    .cross_align(Alignment::center())
                    .content(Content::Flex)
                    .child(
                        TooltipContainer::new(Tooltip::new("Left margin")).child(
                            label()
                                .text_align(TextAlign::Center)
                                .width(Size::px(25.))
                                .height(Size::px(25.))
                                .text(format!("{}", margin.left())),
                        ),
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
                                TooltipContainer::new(Tooltip::new("Top padding")).child(
                                    label()
                                        .text_align(TextAlign::Center)
                                        .width(Size::px(25.))
                                        .height(Size::px(25.))
                                        .text(format!("{}", padding.top())),
                                ),
                            )
                            .child(
                                rect()
                                    .direction(Direction::Horizontal)
                                    .height(Size::flex(1.))
                                    .content(Content::Flex)
                                    .cross_align(Alignment::center())
                                    .child(
                                        TooltipContainer::new(Tooltip::new("Left padding")).child(
                                            label()
                                                .text_align(TextAlign::Center)
                                                .width(Size::px(25.))
                                                .height(Size::px(25.))
                                                .text(format!("{}", padding.left())),
                                        ),
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
                                                    .child(label().text(inner_area)),
                                            ),
                                    )
                                    .child(
                                        TooltipContainer::new(Tooltip::new("Right padding")).child(
                                            label()
                                                .text_align(TextAlign::Center)
                                                .width(Size::px(25.))
                                                .height(Size::px(25.))
                                                .text(format!("{}", padding.right())),
                                        ),
                                    ),
                            )
                            .child(
                                TooltipContainer::new(Tooltip::new("Bottom padding")).child(
                                    label()
                                        .text_align(TextAlign::Center)
                                        .width(Size::px(25.))
                                        .height(Size::px(25.))
                                        .text(format!("{}", padding.bottom())),
                                ),
                            ),
                    )
                    .child(
                        TooltipContainer::new(Tooltip::new("Right margin")).child(
                            label()
                                .text_align(TextAlign::Center)
                                .width(Size::px(25.))
                                .height(Size::px(25.))
                                .text(format!("{}", margin.right())),
                        ),
                    ),
            )
            .child(
                TooltipContainer::new(Tooltip::new("Bottom margin")).child(
                    label()
                        .text_align(TextAlign::Center)
                        .width(Size::px(25.))
                        .height(Size::px(25.))
                        .text(format!("{}", margin.bottom())),
                ),
            ),
    )
}
