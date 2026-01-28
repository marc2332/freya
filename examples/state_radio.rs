#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    prelude::*,
    radio::*,
};

#[derive(Default)]
struct Data {
    pub lists: Vec<Vec<String>>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    ListCreation,
    SpecificListItemUpdate(usize),
}

impl RadioChannel<Data> for DataChannel {}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}
fn app() -> impl IntoElement {
    use_init_radio_station::<Data, DataChannel>(Data::default);
    let mut radio = use_radio::<Data, DataChannel>(DataChannel::ListCreation);

    let on_press = move |_| {
        radio.write().lists.push(Vec::default());
    };

    println!("Running DataChannel::ListCreation");

    rect()
        .horizontal()
        .child(Button::new().on_press(on_press).child("Add new list"))
        .children(
            radio
                .read()
                .lists
                .iter()
                .enumerate()
                .map(|(list_n, _)| ListComp(list_n).into()),
        )
}

#[derive(PartialEq)]
struct ListComp(usize);
impl Component for ListComp {
    fn render(&self) -> impl IntoElement {
        let list_n = self.0;
        let mut radio = use_radio::<Data, DataChannel>(DataChannel::SpecificListItemUpdate(list_n));

        println!("Running DataChannel::SpecificListItemUpdate({list_n})");

        rect()
            .child(
                Button::new()
                    .on_press(move |_| radio.write().lists[list_n].push("Hello, World".to_string()))
                    .child("New Item"),
            )
            .children(
                radio.read().lists[list_n]
                    .iter()
                    .enumerate()
                    .map(move |(i, item)| label().key(i).text(item.clone()).into()),
            )
    }
}
