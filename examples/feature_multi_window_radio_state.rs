use freya::prelude::*;
use freya_radio::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
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

fn app() -> Element {
    let radio_station = use_init_radio_station::<Data, DataChannel>(Data::default);

    let on_press = move |_| {
        EventNotifier::get().launch_window(WindowConfig::new(move || {
            use_share_radio(move || radio_station);
            sub_app()
        }));
    };

    rect()
        .expanded()
        .center()
        .child(Button::new().on_press(on_press).child("Open"))
        .into()
}

fn sub_app() -> Element {
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
        .into()
}
