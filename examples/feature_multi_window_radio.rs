#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    prelude::*,
    radio::*,
};

fn main() {
    let radio_station = RadioStation::create_global(Data::default());

    launch(LaunchConfig::new().with_window(WindowConfig::new_app(CounterApp { radio_station })))
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

struct CounterApp {
    radio_station: RadioStation<Data, DataChannel>,
}

impl App for CounterApp {
    fn render(&self) -> impl IntoElement {
        use_share_radio(move || self.radio_station);
        let mut radio = use_radio(DataChannel::Count);

        let radio_station = self.radio_station;
        let on_open = move |_| {
            spawn(async move {
                let _ = Platform::get()
                    .launch_window(WindowConfig::new_app(CounterApp { radio_station }))
                    .await;
            });
        };

        let on_increase = move |_| {
            radio.write().count += 1;
        };

        rect()
            .expanded()
            .center()
            .spacing(6.)
            .child(format!("Count: {}", radio.read().count))
            .child(Button::new().on_press(on_increase).child("Increase"))
            .child(Button::new().on_press(on_open).child("Open another window"))
    }
}
