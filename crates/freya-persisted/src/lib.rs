//! Synchronous state persistence with pluggable storage and encoding.
//!
//! [`Persisted`] combines a [`PersistenceTransport`] for bytes with a [`PersistenceFormat`]
//! for values. Custom transports and formats work without filesystem access or Serde.
//!
//! # Features
//!
//! No features are enabled by default. Freya exposes them as `persisted-*` through `freya::persisted`.
//!
//! - `fs`: `PersistedLocal` stores files at an explicit path or in the OS config directory.
//! - `json`: `PersistedJson` encodes types implementing Serde's serialize and deserialize traits.
//! - `fs-watch`: file change notifications, including creation and atomic replacement.
//! - `radio`: save selected Radio state and merge reloads with channel notifications.
//!
//! # Save and load
//!
//! ```rust,no_run
//! # #[cfg(all(feature = "fs", feature = "json"))]
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     use freya_persisted::prelude::*;
//!
//!     #[derive(Default, serde::Deserialize, serde::Serialize)]
//!     struct Settings {
//!         dark_mode: bool,
//!     }
//!
//!     let persisted = Persisted::new(
//!         PersistedLocal::for_app("my-app", "settings.json")?,
//!         PersistedJson,
//!     );
//!     let mut settings: Settings = persisted.load()?.unwrap_or_default();
//!     settings.dark_mode = true;
//!     persisted.save(&settings)?;
//!     Ok(())
//! }
//! # #[cfg(not(all(feature = "fs", feature = "json")))]
//! # fn main() {}
//! ```
//!
//! Missing data returns `None`. Storage and decoding failures return errors.
//! Local writes create parent directories and atomically replace files. Use `PersistedLocal::new` for custom paths.
//!
//! [`PersistedStateExt`] adds `save_state(&state)` and `reload_state(&mut state)` for `State<T>` and `Writable<T>`.
//! `load_migrated` transforms raw data before decoding without writing it back. Save explicitly to persist a migration.
//!
//! # Radio and file watching
//!
//! With `radio`, `PersistedRadioExt::save_radio(station, to)` borrows selected data through `to`.
//! `reload_radio(station, from)` merges decoded data and returns the channel selected by `from`, or `None` for silence.
//!
//! Missing data returns `None`, and failed loads leave state unchanged. Use `reload_radio_migrated` for migrations.
//! Pass the station and mapper on each call. Different files can save different parts of the same station.
//!
//! Change state through Radio. For autosaving, subscribe an effect to a dedicated save channel with `read_channel`.
//! Use `derive_channel` to notify UI subscribers on reload without triggering another save.
//!
//! `PersistedLocal::watch` coalesces pending file notifications without loading data.
//! Receive events and reload explicitly. Dropping the watcher stops watching.
//!
//! Examples: [application-wide persistence](https://github.com/marc2332/freya/blob/main/examples/state_persisted.rs)
//! and [per-window persistence](https://github.com/marc2332/freya/blob/main/examples/state_persisted_per_window.rs).

mod error;
mod format;
mod persisted;
mod state;
mod transport;

pub mod prelude {
    #[cfg(feature = "json")]
    pub use crate::PersistedJson;
    #[cfg(feature = "fs")]
    pub use crate::PersistedLocal;
    #[cfg(feature = "fs-watch")]
    pub use crate::PersistedLocalWatcher;
    #[cfg(feature = "radio")]
    pub use crate::PersistedRadioExt;
    pub use crate::{
        Persisted,
        PersistedStateExt,
        PersistenceError,
        PersistenceFormat,
        PersistenceMigrationError,
        PersistenceTransport,
    };
}

pub use crate::{
    error::{
        PersistenceError,
        PersistenceMigrationError,
    },
    format::PersistenceFormat,
    persisted::Persisted,
    state::PersistedStateExt,
    transport::PersistenceTransport,
};

#[cfg(feature = "fs")]
mod fs;
#[cfg(feature = "fs")]
pub use crate::fs::PersistedLocal;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use crate::json::PersistedJson;

#[cfg(feature = "radio")]
mod radio;
#[cfg(feature = "radio")]
pub use crate::radio::PersistedRadioExt;

#[cfg(feature = "fs-watch")]
mod watch;
#[cfg(feature = "fs-watch")]
pub use crate::watch::PersistedLocalWatcher;
