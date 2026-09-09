#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_size(500., 400.)
                .with_title("Events Combos"),
        ),
    )
}

fn app() -> impl IntoElement {
    let mut press_type = use_state(|| None::<PressEventType>);

    let (text, background, presses) = match *press_type.read() {
        Some(PressEventType::Single) => ("Single press", (70, 90, 150), 1),
        Some(PressEventType::Double) => ("Double press", (60, 130, 110), 2),
        Some(PressEventType::Triple) => ("Triple press", (160, 120, 50), 3),
        Some(PressEventType::Quadruple) => ("Quadruple press", (150, 70, 110), 4),
        None => ("Press me", (60, 60, 60), 0),
    };

    rect()
        .expanded()
        .center()
        .spacing(20.)
        .background((30, 30, 30))
        .color(Color::WHITE)
        .child(
            rect()
                .width(Size::px(280.))
                .height(Size::px(160.))
                .center()
                .corner_radius(12.)
                .background(background)
                .font_size(22.)
                .child(text)
                .on_pointer_down(move |e: Event<PointerEventData>| {
                    if e.is_primary() {
                        press_type.set(Some(EventsCombos::pressed(e.global_location())));
                    }
                }),
        )
        .child(
            rect()
                .horizontal()
                .spacing(10.)
                .children((0..4).map(|index| {
                    rect()
                        .width(Size::px(14.))
                        .height(Size::px(14.))
                        .corner_radius(7.)
                        .background(if index < presses {
                            (230, 230, 230)
                        } else {
                            (70, 70, 70)
                        })
                        .into_element()
                })),
        )
        .child(
            rect()
                .center()
                .spacing(4.)
                .font_size(13.)
                .color((160, 160, 160))
                .child("Up to four presses in a row form a combo")
                .child("Within 500ms of each other and without moving the pointer away"),
        )
}
