use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::{
        Arc,
        Mutex,
    },
    time::Duration,
};

use bytes::Bytes;
use dioxus_core::{
    spawn_forever,
    ReactiveContext,
    Task,
};
use dioxus_hooks::{
    use_context,
    use_context_provider,
    use_effect,
    use_reactive,
    use_signal,
};
use dioxus_signals::{
    Readable,
    Signal,
    Writable,
};
use tokio::time::sleep;
use tracing::info;

use crate::use_drop::use_drop;

/// Defines the duration for which an Asset will remain cached after it's user has stopped using it.
/// The default is 1h (3600s).
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum AssetAge {
    /// Asset will be cached for the specified duration
    Duration(Duration),
    /// Asset will be cached until app is closed
    Unspecified,
}

impl Default for AssetAge {
    fn default() -> Self {
        Self::Duration(Duration::from_secs(3600)) // 1h
    }
}

impl From<Duration> for AssetAge {
    fn from(value: Duration) -> Self {
        Self::Duration(value)
    }
}

/// Configuration for a given Asset.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AssetConfiguration {
    /// Asset age.
    pub age: AssetAge,
    /// The ID of the asset. For example: For images their source URL or path can be used as ID.
    pub id: String,
}

enum AssetUsers {
    Listeners(Arc<Mutex<HashSet<ReactiveContext>>>),
    ClearTask(Task),
}

#[derive(Clone)]
pub enum AssetBytes {
    /// Asset is cached.
    Cached(Bytes),
    /// Asset is currently being fetched.
    Loading,
    /// Asset has yet to be fetched.
    Pending,
    /// Failed to fetch asset.
    Error(String),
}

impl AssetBytes {
    /// Try to get [AssetBytes] as bytes.
    pub fn try_as_bytes(&self) -> Option<&Bytes> {
        match self {
            Self::Cached(bytes) => Some(bytes),
            _ => None,
        }
    }
}

struct AssetState {
    users: AssetUsers,
    asset_bytes: AssetBytes,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct AssetCacher {
    registry: Signal<HashMap<AssetConfiguration, AssetState>>,
}

impl AssetCacher {
    /// Attempt to resolve a [AssetBytes] given a [AssetConfiguration].
    pub fn read_asset(&self, asset_config: &AssetConfiguration) -> Option<AssetBytes> {
        self.registry
            .peek_unchecked()
            .get(asset_config)
            .map(|a| a.asset_bytes.clone())
    }

    /// Update an [AssetBytes] given a [AssetConfiguration].
    pub fn update_asset(&mut self, asset_config: AssetConfiguration, asset_bytes: AssetBytes) {
        let mut registry = self.registry.write();

        let asset = registry
            .entry(asset_config.clone())
            .or_insert_with(|| AssetState {
                asset_bytes: AssetBytes::Pending,
                users: AssetUsers::Listeners(Arc::default()),
            });

        asset.asset_bytes = asset_bytes;

        // Reruns those listening components
        if let AssetUsers::Listeners(listeners) = &asset.users {
            for sub in listeners.lock().unwrap().iter() {
                sub.mark_dirty();
            }
            info!(
                "Marked as dirty {} reactive contexts listening to asset with id '{}'",
                listeners.lock().unwrap().len(),
                asset_config.id
            );
        }
    }

    /// Try to clean an asset with no more listeners given a [AssetConfiguration].
    pub fn try_clean(&mut self, asset_config: &AssetConfiguration) {
        let mut registry = self.registry;

        let spawn_clear_task = {
            let mut registry = registry.write();

            let entry = registry.get_mut(asset_config);
            if let Some(asset_state) = entry {
                match &mut asset_state.users {
                    AssetUsers::Listeners(listeners) => {
                        // Only spawn a clear-task if there are no more listeners using this asset
                        listeners.lock().unwrap().is_empty()
                    }
                    AssetUsers::ClearTask(task) => {
                        // This case should never happen but... we leave it here anyway.
                        task.cancel();
                        true
                    }
                }
            } else {
                false
            }
        };

        if spawn_clear_task {
            // Only clear the asset if a duration was specified
            if let AssetAge::Duration(duration) = asset_config.age {
                let clear_task = spawn_forever({
                    let asset_config = asset_config.clone();
                    async move {
                        info!("Waiting asset with ID '{}' to be cleared", asset_config.id);
                        sleep(duration).await;
                        registry.write().remove(&asset_config);
                        info!("Cleared asset with ID '{}'", asset_config.id);
                    }
                })
                .unwrap();

                // Registry the clear-task
                let mut registry = registry.write();
                let entry = registry.get_mut(asset_config).unwrap();
                entry.users = AssetUsers::ClearTask(clear_task);
            }
        }
    }

    pub(crate) fn listen(&self, rc: ReactiveContext, asset_config: AssetConfiguration) {
        let mut registry = self.registry.write_unchecked();

        registry
            .entry(asset_config.clone())
            .or_insert_with(|| AssetState {
                asset_bytes: AssetBytes::Pending,
                users: AssetUsers::Listeners(Arc::default()),
            });

        if let Some(asset) = registry.get(&asset_config) {
            match &asset.users {
                AssetUsers::Listeners(users) => {
                    rc.subscribe(users.clone());
                }
                AssetUsers::ClearTask(clear_task) => {
                    clear_task.cancel();
                    info!(
                        "Clear task of asset with ID '{}' has been cancelled",
                        asset_config.id
                    );
                }
            }
        }
    }

    /// Read the size of the cache registry.
    pub fn size(&self) -> usize {
        self.registry.read().len()
    }
}

/// Start listening to an asset given a [AssetConfiguration].
///
/// Use in combination with [use_asset_cacher].
pub fn use_asset(asset_config: AssetConfiguration) -> AssetBytes {
    let mut asset_cacher = use_asset_cacher();

    if let Some(rc) = ReactiveContext::current() {
        asset_cacher.listen(rc, asset_config.clone());
    }

    use_drop({
        let asset_config = asset_config.clone();
        move || {
            // Try to clean in the next async tick, when this scope will already be dropped
            spawn_forever(async move {
                asset_cacher.try_clean(&asset_config);
            });
        }
    });

    let mut prev = use_signal(|| None);

    use_effect(use_reactive!(|asset_config| {
        if let Some(prev) = &*prev.peek_unchecked() {
            if prev != &asset_config {
                // Try to clean the previous asset
                asset_cacher.try_clean(&asset_config);
            }
        }

        prev.write().replace(asset_config.clone());
    }));

    asset_cacher.read_asset(&asset_config).unwrap()
}

/// Get access to the global cache of assets.
///
/// Use in combination with [use_asset].
pub fn use_asset_cacher() -> AssetCacher {
    use_context()
}

/// Initialize the global cache of assets.
///
/// This is a **low level** hook that **runs by default** in all Freya apps.
pub(crate) fn use_init_asset_cacher() {
    use_context_provider(AssetCacher::default);
}
