#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_radio::prelude::*;

#[derive(Default, Clone)]
struct MyState {
    count: i32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum AppChannel {
    CounterA,
    CounterB,
}

impl RadioChannel<MyState> for AppChannel {}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    use_init_radio_station::<MyState, AppChannel>(MyState::default);

    let radio = use_radio::<MyState, AppChannel>(AppChannel::CounterA);
    let local_state = use_state(|| MyState { count: 10 });

    let slice_state = radio.slice_mut_current(|s| &mut s.count);

    rect()
        .spacing(20.0)
        .child(
            rect()
                .padding(8.0)
                .background((240, 240, 240))
                .child("Slice of Radio")
                .on_press({
                    let mut slice_state = slice_state.clone();
                    move |_| {
                        *slice_state.write() += 1;
                    }
                })
                .child(CounterDisplay {
                    count: slice_state.into_readable(),
                }),
        )
        .child(
            rect()
                .padding(8.0)
                .background((230, 245, 255))
                .child("Local State")
                .child(StateDisplay {
                    state: local_state.into_writable(),
                }),
        )
}

#[derive(PartialEq)]
struct CounterDisplay {
    count: Readable<i32>,
}

impl Component for CounterDisplay {
    fn render(&self) -> impl IntoElement {
        format!("-> {}", self.count.read())
    }
}

#[derive(PartialEq)]
struct StateDisplay {
    state: Writable<MyState>,
}

impl Component for StateDisplay {
    fn render(&self) -> impl IntoElement {
        let mut state = self.state.clone();
        rect()
            .on_press(move |_| {
                state.write().count += 1;
            })
            .child(format!("-> {}", self.state.read().count))
    }
}
