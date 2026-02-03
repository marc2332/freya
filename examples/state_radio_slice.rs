use freya::prelude::*;
use freya_radio::prelude::*;

#[derive(Default, Clone)]
#[allow(dead_code)]
struct AppState {
    count: i32,
    name: String,
    items: Vec<String>,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum AppChannel {
    Count,
    Name,
    Items,
}

impl RadioChannel<AppState> for AppChannel {}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    use_init_radio_station::<AppState, AppChannel>(AppState::default);

    let radio = use_radio::<AppState, AppChannel>(AppChannel::Count);

    let count_slice = radio.slice_current(|state| &state.count);
    let name_slice = radio.slice(AppChannel::Name, |state| &state.name);

    let mut count_slice_mut = radio.slice_mut_current(|state| &mut state.count);
    let mut name_slice_mut = radio.slice_mut(AppChannel::Name, |state| &mut state.name);

    rect()
        .spacing(10.0)
        .padding(20.0)
        .child(CountDisplay(count_slice))
        .child(NameDisplay(name_slice))
        .child(
            Button::new()
                .on_press(move |_| {
                    name_slice_mut.write().push('!');
                })
                .child("Change Name"),
        )
        .child(
            Button::new()
                .on_press(move |_| {
                    *count_slice_mut.write() += 1;
                })
                .child("Name Count"),
        )
}

#[derive(PartialEq)]
struct CountDisplay(RadioSlice<AppState, i32, AppChannel>);
impl Component for CountDisplay {
    fn render(&self) -> impl IntoElement {
        println!("Rendering CountDisplay");
        rect()
            .background((100, 150, 200))
            .padding(10.0)
            .child(format!("Count: {}", self.0.read()))
    }
}

#[derive(PartialEq)]
struct NameDisplay(RadioSlice<AppState, String, AppChannel>);
impl Component for NameDisplay {
    fn render(&self) -> impl IntoElement {
        println!("Rendering NameDisplay");
        rect()
            .background((200, 100, 150))
            .padding(10.0)
            .child(format!("Name: {}", self.0.read()))
    }
}
