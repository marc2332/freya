use std::{
    thread::{
        self,
        sleep,
    },
    time::Duration,
};

use freya::prelude::*;
use freya_radio::prelude::*;
use futures_lite::StreamExt;

fn main() {
    let mut radio_station = RadioStation::create_global(Data::default());

    launch(
        LaunchConfig::new()
            .with_future(async move {
                // Run CPU intensive logic in a separate thread and use a channel to update the radio store

                let (tx, mut rx) = futures_channel::mpsc::unbounded::<()>();

                thread::spawn(move || {
                    loop {
                        sleep(Duration::from_secs(1));
                        let _ = tx.unbounded_send(());
                    }
                });

                while rx.next().await.is_some() {
                    radio_station.write_channel(DataChannel::Count).count += 1;
                }
            })
            .with_window(WindowConfig::new(FpRender::from_render(App {
                radio_station,
            }))),
    )
}

#[derive(Default)]
struct Data {
    pub count: i32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    Count,
}

impl RadioChannel<Data> for DataChannel {}

struct App {
    radio_station: RadioStation<Data, DataChannel>,
}

impl Render for App {
    fn render(&self) -> impl IntoElement {
        use_share_radio(move || self.radio_station);
        let mut radio = use_radio(DataChannel::Count);

        let on_press = move |_| {
            radio.write().count += 1;
        };

        rect()
            .expanded()
            .center()
            .spacing(6.)
            .child(format!("Value is {}", radio.read().count))
            .child(Button::new().on_press(on_press).child("Increase"))
    }
}
