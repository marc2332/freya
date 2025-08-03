use dioxus_radio::prelude::use_radio;
use freya::prelude::*;
use freya_core::animation_clock::AnimationClock;
use freya_devtools::{
    IncomingMessage,
    IncomingMessageAction,
};
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

use crate::state::DevtoolsChannel;

#[component]
pub fn Misc() -> Element {
    rsx!(
        rect {
            height: "fill",
            width: "fill",
            padding: "8",
            Animation {}
        }
    )
}

#[component]
pub fn Animation() -> Element {
    let mut radio = use_radio(DevtoolsChannel::Misc);

    use_effect(move || {
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

    rsx!(
        rect {
            label {
                "Animation Speed"
            }
            rect {
                direction: "horizontal",
                spacing: "4",
                Slider {
                    size: "150",
                    value: speed as f64,
                    onmoved: move |p| {
                        radio.write().animation_speed = p as f32;
                    }
                }
                label {
                    "{normalized_speed:.2}x"
                }
            }
        }
    )
}
