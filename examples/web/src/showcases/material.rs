use freya::{
    material_design::*,
    prelude::*,
};

use crate::showcases::heading;

const TABS: [&str; 3] = ["Overview", "Details", "Activity"];

#[derive(PartialEq)]
pub struct MaterialShowcase;

impl Component for MaterialShowcase {
    fn render(&self) -> impl IntoElement {
        let mut selected = use_state(|| 0usize);

        rect()
            .spacing(20.)
            .child(heading(
                "Material Design",
                "Tabs, cards and a ripple under your cursor",
            ))
            .child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .children(TABS.iter().enumerate().map(|(index, name)| {
                        FloatingTab::new().ripple().child(
                            rect()
                                .center()
                                .padding((6., 14., 6., 14.))
                                .on_press(move |_| selected.set(index))
                                .child(*name),
                        )
                    })),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .corner_radius(8.)
                    .overflow(Overflow::Clip)
                    .child(
                        Ripple::new().width(Size::fill()).child(
                            Card::new().child(
                                rect()
                                    .spacing(8.)
                                    .width(Size::fill())
                                    .child(rect().font_size(18.).child(TABS[selected()]))
                                    .child("Click anywhere on this card."),
                            ),
                        ),
                    ),
            )
            .child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .child(Button::new().ripple().child("Press here"))
                    .child(Button::new().outline().ripple().child("Or here")),
            )
    }
}
