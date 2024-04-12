use bytes::Bytes;
use dioxus_core::{
    prelude::{current_scope_id, provide_root_context, spawn, try_consume_context, ScopeId, Task},
    Runtime,
};
use dioxus_signals::Signal;
use dioxus_signals::{Readable, Writable};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
use tokio::time::sleep;

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
    Scopes(HashSet<ScopeId>),
    ClearTask(Task),
}

struct AssetState {
    users: AssetUsers,
    asset_bytes: Signal<Bytes>,
}

#[derive(Clone, Copy, Default)]
pub struct AssetCacher {
    registry: Signal<HashMap<AssetConfiguration, AssetState>>,
}

impl AssetCacher {
    /// Cache the given [`AssetConfiguration`]
    pub fn cache(
        &mut self,
        asset_config: AssetConfiguration,
        asset_bytes: Bytes,
        subscribe: bool,
    ) -> Signal<Bytes> {
        // Cancel previous caches
        if let Some(mut asset_state) = self.registry.write().remove(&asset_config) {
            if let AssetUsers::ClearTask(task) = asset_state.users {
                task.cancel();
                asset_state.asset_bytes.take();
            }
        }

        // Insert the asset into the cache
        let asset_bytes = Signal::new(asset_bytes);
        self.registry.write().insert(
            asset_config.clone(),
            AssetState {
                asset_bytes,
                users: AssetUsers::Scopes(if subscribe {
                    HashSet::from([current_scope_id().unwrap()])
                } else {
                    HashSet::default()
                }),
            },
        );

        asset_bytes
    }

    /// Stop using an asset. It will get removed after the specified duration if it's not used until then.
    pub fn unuse_asset(&mut self, asset_config: AssetConfiguration) {
        let mut registry = self.registry;

        let spawn_clear_task = {
            let mut registry = registry.write();

            let entry = registry.get_mut(&asset_config);
            if let Some(asset_state) = entry {
                match &mut asset_state.users {
                    AssetUsers::Scopes(scopes) => {
                        // Unsub
                        scopes.remove(&current_scope_id().unwrap());

                        // Only spawn a clear-task if there are no more scopes using this asset
                        scopes.is_empty()
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
                // Why not use `spawn_forever`? Reason: https://github.com/DioxusLabs/dioxus/issues/2215
                let clear_task = Runtime::current().unwrap().on_scope(ScopeId(0), || {
                    spawn({
                        let asset_config = asset_config.clone();
                        async move {
                            sleep(duration).await;
                            if let Some(mut asset_state) = registry.write().remove(&asset_config) {
                                // Clear the asset
                                asset_state.asset_bytes.take();
                            }
                        }
                    })
                });

                // Registry the clear-task
                let mut registry = registry.write();
                let entry = registry.get_mut(&asset_config).unwrap();
                entry.users = AssetUsers::ClearTask(clear_task);
            }
        }
    }

    /// Start using an Asset. Your scope will get subscribed, to stop using an asset use [`Self::unuse_asset`]
    pub fn use_asset(&mut self, config: &AssetConfiguration) -> Option<Signal<Bytes>> {
        let mut registry = self.registry.write();
        if let Some(asset_state) = registry.get_mut(config) {
            match &mut asset_state.users {
                AssetUsers::ClearTask(task) => {
                    // Cancel clear-tasks
                    task.cancel();
                    asset_state.asset_bytes.take();

                    // Start using this asset
                    asset_state.users =
                        AssetUsers::Scopes(HashSet::from([current_scope_id().unwrap()]));
                }
                AssetUsers::Scopes(scopes) => {
                    // Start using this asset
                    scopes.insert(current_scope_id().unwrap());
                }
            }
        }

        registry.get(config).map(|s| s.asset_bytes)
    }

    /// Get the size of the cache registry.
    pub fn size(&self) -> usize {
        self.registry.read().len()
    }

    /// Clear all the assets from the cache registry.
    pub fn clear(&mut self) {
        self.registry.try_write().unwrap().clear();
    }
}

/// Global caching system for assets.
///
/// This is a "low level" hook, so you probably won't need it.
pub fn use_asset_cacher() -> AssetCacher {
    match try_consume_context() {
        Some(asset_cacher) => asset_cacher,
        None => provide_root_context(AssetCacher::default()),
    }
}
