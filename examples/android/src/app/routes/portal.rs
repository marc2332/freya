use std::time::Duration;

use freya::{
    animation::*,
    material_design::Ripple,
    prelude::*,
};

#[derive(PartialEq)]
pub struct PortalDemo;

impl Component for PortalDemo {
    fn render(&self) -> impl IntoElement {
        let mut show_popup = use_state::<Option<i32>>(|| None);

        ScrollView::new()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                rect()
                    .width(Size::fill())
                    .spacing(12.)
                    .padding(12.)
                    .maybe_child(show_popup().map(|i| portal_popup(i, show_popup)))
                    .children((0..5).map(|i| {
                        rect()
                            .key(i)
                            .spacing(6.)
                            .width(Size::fill())
                            .child(
                                Portal::new(i)
                                    .key(show_popup())
                                    .show(show_popup() != Some(i))
                                    .width(Size::fill())
                                    .height(Size::px(120.))
                                    .function(Function::Expo)
                                    .duration(Duration::from_millis(500))
                                    .child(portal_card(i)),
                            )
                            .on_press(move |_| show_popup.set(Some(i)))
                            .into()
                    })),
            )
    }
}

fn portal_card(i: i32) -> impl IntoElement {
    Ripple::new().color((255, 255, 255)).child(
        rect()
            .expanded()
            .background((103, 80, 164))
            .corner_radius(16.)
            .center()
            .color(Color::WHITE)
            .child(format!("Card {}", i)),
    )
}

fn portal_popup(i: i32, mut show_popup: State<Option<i32>>) -> impl IntoElement {
    Popup::new()
        .on_close_request(move |_| show_popup.set(None))
        .width(Size::px(350.))
        .child(PopupTitle::new(format!("Card {i}")))
        .child(
            PopupContent::new().child(
                Portal::new(i)
                    .width(Size::px(250.))
                    .height(Size::px(150.))
                    .function(Function::Expo)
                    .duration(Duration::from_millis(500))
                    .child(portal_card(i)),
            ),
        )
        .child(
            PopupButtons::new()
                .child(
                    Button::new()
                        .expanded()
                        .rounded_full()
                        .on_press(move |_| show_popup.set(None))
                        .child("Close"),
                )
                .child(
                    Button::new()
                        .filled()
                        .expanded()
                        .rounded_full()
                        .on_press(move |_| show_popup.set(None))
                        .child("Accept"),
                ),
        )
}
