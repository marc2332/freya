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
                    .child(portal_popup(show_popup))
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

fn portal_popup(mut show_popup: State<Option<i32>>) -> impl IntoElement {
    let show = show_popup().is_some();
    let i = show_popup().unwrap_or(0);

    Popup::new()
        .show(show)
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
