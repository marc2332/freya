#![cfg(all(feature = "radio", feature = "fs", feature = "json"))]

use std::{
    cell::Cell,
    fs,
    io,
    rc::Rc,
    task::Waker,
};

use freya_core::prelude::*;
use freya_persisted::prelude::*;
use freya_radio::prelude::*;

struct AppState {
    count: u32,
    step: u32,
}

impl AppState {
    fn apply_step(&mut self, step: u32) -> Option<Channel> {
        let channel = (self.step != step).then_some(Channel::Reloaded);
        self.step = step;
        channel
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Channel {
    Count,
    Step,
    Save,
    Reloaded,
}

impl RadioChannel<AppState> for Channel {
    fn derive_channel(self, _state: &AppState) -> Vec<Self> {
        match self {
            Self::Step => vec![Self::Step, Self::Save],
            Self::Reloaded => vec![Self::Step],
            channel => vec![channel],
        }
    }
}

#[test]
fn projected_autosave_and_reload() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("settings.json");
    let storage = Persisted::new(PersistedLocal::new(&path), PersistedJson);
    let mut station = RadioStation::create_global(AppState { count: 0, step: 1 });
    assert_eq!(
        storage.reload_radio(station, AppState::apply_step).unwrap(),
        None
    );

    let globals = GlobalContexts::register();
    let tasks = globals.insert_context(GlobalTasks::new(Waker::noop().clone()));
    let saves = Rc::new(Cell::new(0));
    let effect = Effect::create_global({
        let storage = storage.clone();
        let saves = saves.clone();
        move || {
            station.read_channel(Channel::Save);
            storage.save_radio(station, |state| &state.step).unwrap();
            saves.set(saves.get() + 1);
        }
    });
    tasks.poll();
    assert_eq!(saves.get(), 1);

    station.write_channel(Channel::Count).count = 7;
    tasks.poll();
    assert_eq!(saves.get(), 1);

    station.write_channel(Channel::Step).step = 2;
    tasks.poll();
    assert_eq!(saves.get(), 2);
    assert_eq!(storage.load::<u32>().unwrap(), Some(2));
    assert_eq!(
        storage.reload_radio(station, AppState::apply_step).unwrap(),
        None
    );

    storage.save(&3u32).unwrap();
    assert_eq!(
        storage.reload_radio(station, AppState::apply_step).unwrap(),
        Some(Channel::Reloaded)
    );
    assert_eq!(
        storage.reload_radio(station, AppState::apply_step).unwrap(),
        None
    );
    tasks.poll();
    assert_eq!((station.peek().count, station.peek().step), (7, 3));
    assert_eq!(saves.get(), 2);

    fs::write(&path, br#"{"step":4}"#).unwrap();
    assert_eq!(
        storage
            .reload_radio_migrated(
                station,
                AppState::apply_step,
                |raw| -> io::Result<serde_json::Value> { Ok(raw["step"].clone()) }
            )
            .unwrap(),
        Some(Channel::Reloaded)
    );
    tasks.poll();
    assert_eq!((station.peek().count, station.peek().step), (7, 4));
    assert_eq!(saves.get(), 2);

    fs::write(path, "invalid json").unwrap();
    assert!(storage.reload_radio(station, AppState::apply_step).is_err());
    assert_eq!((station.peek().count, station.peek().step), (7, 4));

    effect.cancel();
    globals.unregister();
}
