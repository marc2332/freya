use freya::prelude::*;
use freya_radio::prelude::*;

fn main() {
    let radio_station = RadioStation::create_global(Data::default());

    launch(
        LaunchConfig::new().with_window(WindowConfig::new(FpRender::from_render(App {
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

        let radio_station = self.radio_station;
        let on_press = move |_| {
            spawn(async move {
                let _ = Platform::get()
                    .launch_window(WindowConfig::new(FpRender::from_render(SubApp {
                        radio_station,
                    })))
                    .await;
            });
        };

        rect()
            .expanded()
            .center()
            .child(Button::new().on_press(on_press).child("Open"))
    }
}

struct SubApp {
    radio_station: RadioStation<Data, DataChannel>,
}

impl Render for SubApp {
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
