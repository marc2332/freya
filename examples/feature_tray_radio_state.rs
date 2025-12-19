use freya::{
    prelude::*,
    tray::{
        TrayEvent,
        TrayIconBuilder,
        menu::{
            Menu,
            MenuEvent,
            MenuItem,
        },
    },
};
use freya_radio::prelude::*;

const ICON: &[u8] = include_bytes!("./freya_icon.png");

fn main() {
    let mut radio_station = RadioStation::create_global(Data::default());

    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(FpRender::from_render(App {
                radio_station,
            })))
            .with_tray(
                move || {
                    let tray_menu = Menu::new();
                    let _ = tray_menu.append(&MenuItem::new("Decrease", true, None));
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu))
                        .with_tooltip("Freya Tray")
                        .with_icon(LaunchConfig::tray_icon(ICON))
                        .build()
                        .unwrap()
                },
                move |ev, _| match ev {
                    TrayEvent::Menu(MenuEvent { id }) if id == "3" => {
                        radio_station.write_channel(DataChannel::Count).count -= 1;
                    }
                    _ => {}
                },
            ),
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
