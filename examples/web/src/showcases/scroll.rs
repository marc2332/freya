use freya::prelude::*;

use crate::showcases::heading;

#[derive(PartialEq)]
pub struct ScrollShowcase;

impl Component for ScrollShowcase {
    fn render(&self) -> impl IntoElement {
        rect()
            .spacing(20.)
            .expanded()
            .child(heading("Virtual Scroll", "Ten thousand rows"))
            .child(
                VirtualScrollView::new(|item, _| {
                    rect()
                        .key(item.index)
                        .height(Size::px(item.size))
                        .padding(4.)
                        .child(
                            rect()
                                .expanded()
                                .padding((0., 12., 0., 12.))
                                .corner_radius(10.)
                                .main_align(Alignment::center())
                                .color((255, 255, 255))
                                .background(if item.index % 2 == 0 {
                                    (79, 70, 229)
                                } else {
                                    (99, 102, 241)
                                })
                                .child(format!("Row {}", item.index)),
                        )
                        .into()
                })
                .length(10_000usize)
                .item_size(48.)
                .expanded(),
            )
    }
}
