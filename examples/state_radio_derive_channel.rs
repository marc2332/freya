#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    radio::*,
};

#[derive(Default)]
struct Counters {
    items: Vec<i32>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum CountersChannel {
    WholeList,
    Specific(usize),
}

impl RadioChannel<Counters> for CountersChannel {
    fn derive_channel(self, state: &Counters) -> Vec<Self> {
        match self {
            CountersChannel::WholeList => {
                let mut channels = vec![CountersChannel::WholeList];
                channels.extend((0..state.items.len()).map(CountersChannel::Specific));
                channels
            }
            CountersChannel::Specific(_) => vec![self],
        }
    }
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    use_init_radio_station::<Counters, CountersChannel>(Counters::default);
    let mut radio = use_radio::<Counters, CountersChannel>(CountersChannel::WholeList);

    println!("Rendering app (WholeList)");

    let count = radio.read().items.len();

    rect()
        .spacing(10.0)
        .padding(20.0)
        .child(format!("Items: {count}"))
        .child(
            rect()
                .horizontal()
                .spacing(6.0)
                .child(
                    Button::new()
                        .on_press(move |_| radio.write().items.push(0))
                        .child("Add Counter"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            for v in radio.write().items.iter_mut() {
                                *v = 0;
                            }
                        })
                        .child("Reset All"),
                ),
        )
        .children((0..count).map(|i| CounterRow(i).into()))
}

#[derive(PartialEq)]
struct CounterRow(usize);
impl Component for CounterRow {
    fn render(&self) -> impl IntoElement {
        let i = self.0;
        let mut radio = use_radio::<Counters, CountersChannel>(CountersChannel::Specific(i));

        println!("Rendering CounterRow {i} (Specific)");

        let value = radio.read().items[i];

        rect()
            .horizontal()
            .spacing(6.0)
            .child(format!("#{i}: {value}"))
            .child(
                Button::new()
                    .on_press(move |_| radio.write().items[i] += 1)
                    .child("+"),
            )
            .child(
                Button::new()
                    .on_press(move |_| radio.write().items[i] -= 1)
                    .child("-"),
            )
    }
}
