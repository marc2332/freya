#![cfg(all(feature = "fs", feature = "json"))]

use std::{
    fs,
    io,
};

use freya_core::prelude::*;
use freya_persisted::prelude::*;

#[test]
fn save_and_load() {
    let directory = tempfile::tempdir().unwrap();
    let transport = PersistedLocal::new(directory.path().join("nested/settings.json"));
    let persisted = Persisted::new(transport.clone(), PersistedJson);
    assert_eq!(persisted.load::<u32>().unwrap(), None);

    persisted.save(&1u32).unwrap();
    persisted.save(&2u32).unwrap();
    assert_eq!(persisted.load::<u32>().unwrap(), Some(2));

    fs::write(transport.path(), "invalid json").unwrap();
    assert!(persisted.load::<u32>().is_err());
}

#[test]
fn save_and_reload_state() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("settings.json");
    let persisted = Persisted::new(PersistedLocal::new(&path), PersistedJson);
    let mut state = State::create_global(1u32);
    assert!(!persisted.reload_state(&mut state).unwrap());

    persisted.save_state(&state).unwrap();
    state.set(2);
    assert!(persisted.reload_state(&mut state).unwrap());
    assert_eq!(*state.peek(), 1);

    let mut writable: Writable<u32> = state.into();
    writable.set(3);
    persisted.save_state(&writable).unwrap();
    assert_eq!(persisted.load::<u32>().unwrap(), Some(3));

    fs::write(path, "invalid json").unwrap();
    assert!(persisted.reload_state(&mut writable).is_err());
    assert_eq!(*state.peek(), 3);
}

#[test]
fn migration_decodes_legacy_data_without_overwriting_it() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("settings.json");
    let legacy = br#"{"step":3}"#;
    fs::write(&path, legacy).unwrap();
    let persisted = Persisted::new(PersistedLocal::new(&path), PersistedJson);

    assert!(persisted.load::<u32>().is_err());
    assert_eq!(
        persisted
            .load_migrated::<u32, io::Error>(|raw| Ok(raw["step"].clone()))
            .unwrap(),
        Some(3)
    );
    assert_eq!(fs::read(path).unwrap(), legacy);
}
