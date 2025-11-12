use freya::prelude::*;
use freya_devtools::{
    IncomingMessage,
    IncomingMessageAction,
};
use freya_radio::hooks::use_radio;
use tungstenite::Message;

use crate::state::DevtoolsChannel;

#[derive(PartialEq)]
pub struct Misc;
impl Render for Misc {
    fn render(&self) -> Element {
        let mut radio = use_radio(DevtoolsChannel::Misc);

        use_side_effect(move || {
            let radio = radio.read();

            let client = radio.client.clone();
            let animation_speed = AnimationClock::MAX_SPEED / 100. * radio.animation_speed;
            let message = Message::Text(
                serde_json::to_string(&IncomingMessage {
                    action: IncomingMessageAction::SetSpeedTo {
                        speed: animation_speed,
                    },
                })
                .unwrap()
                .into(),
            );
            spawn(async move {
                if let Some(client) = client.lock().await.as_mut() {
                    client.send(message).await.ok();
                }
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
            .into()
    }
}
