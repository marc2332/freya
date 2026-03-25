use freya::prelude::*;
use freya_devtools::IncomingMessageAction;
use freya_radio::hooks::use_radio;

use crate::state::DevtoolsChannel;

#[derive(PartialEq)]
pub struct Misc;
impl Component for Misc {
    fn render(&self) -> impl IntoElement {
        let mut radio = use_radio(DevtoolsChannel::Misc);

        use_side_effect(move || {
            let radio = radio.read();
            let animation_speed = AnimationClock::MAX_SPEED / 100. * radio.animation_speed;
            radio.send_action(IncomingMessageAction::SetSpeedTo {
                speed: animation_speed,
            });
        });
        let speed = radio.read().animation_speed;
        let normalized_speed = AnimationClock::MAX_SPEED / 100. * speed;

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .padding(8.)
            .spacing(6.)
            .child("Animation Speed")
            .child(
                rect()
                    .horizontal()
                    .child(
                        Slider::new(move |p| {
                            radio.write().animation_speed = p as f32;
                        })
                        .size(Size::px(200.))
                        .value(speed as f64),
                    )
                    .child(format!("{normalized_speed:.2}x")),
            )
            .child(
                Button::new()
                    .on_press(move |_| {
                        radio.write().animation_speed = 1. / AnimationClock::MAX_SPEED * 100.
                    })
                    .child("Reset"),
            )
    }
}
