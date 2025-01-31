use std::{
    collections::{
        HashMap,
        HashSet,
    },
    time::Duration,
};

use bytes::Bytes;
use dioxus_core::{
    prelude::{
        current_scope_id,
        spawn_forever,
        ScopeId,
        Task,
    },
    schedule_update_any,
};
use dioxus_hooks::{
    use_context,
    use_context_provider,
};
use dioxus_signals::{
    Readable,
    Signal,
    Writable,
};
use tokio::time::sleep;
use tracing::info;

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
    asset_bytes: Bytes,
}

#[derive(Clone, Copy, Default)]
pub struct AssetCacher {
    registry: Signal<HashMap<AssetConfiguration, AssetState>>,
}

impl AssetCacher {
    /// Cache the given [`AssetConfiguration`]. If it already exists and has a pending clear-task, it will get cancelled.
    pub fn cache_asset(
        &mut self,
        asset_config: AssetConfiguration,
        asset_bytes: Bytes,
        subscribe: bool,
    ) {
        // Invalidate previous caches
        if let Some(asset_state) = self.registry.write().remove(&asset_config) {
            if let AssetUsers::ClearTask(task) = asset_state.users {
                task.cancel();
                info!("Clear task of asset with ID '{}' has been cancelled as the asset has been revalidated", asset_config.id);
            }
        }

        // Insert the asset into the cache
        let current_scope_id = current_scope_id().unwrap();

        self.registry.write().insert(
            asset_config.clone(),
            AssetState {
                asset_bytes,
                users: AssetUsers::Scopes(if subscribe {
                    HashSet::from([current_scope_id])
                } else {
                    HashSet::default()
                }),
            },
        );

        schedule_update_any()(current_scope_id);
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
                let entry = registry.get_mut(&asset_config).unwrap();
                entry.users = AssetUsers::ClearTask(clear_task);
            }
        }
    }

    /// Start using an Asset. Your scope will get subscribed, to stop using an asset use [`Self::unuse_asset`]
    pub fn use_asset(&mut self, asset_config: &AssetConfiguration) -> Option<Bytes> {
        let mut registry = self.registry.write();
        if let Some(asset_state) = registry.get_mut(asset_config) {
            match &mut asset_state.users {
                AssetUsers::ClearTask(task) => {
                    // Cancel clear-task
                    task.cancel();
                    info!(
                        "Clear task of asset with ID '{}' has been cancelled",
                        asset_config.id
                    );

                    // Start using this asset
                    asset_state.users =
                        AssetUsers::Scopes(HashSet::from([current_scope_id().unwrap()]));
                }
                AssetUsers::Scopes(scopes) => {
                    // Start using this asset
                    scopes.insert(current_scope_id().unwrap());
                }
            }

            // Reruns those subscribed components
            if let AssetUsers::Scopes(scopes) = &asset_state.users {
                let schedule = schedule_update_any();
                for scope in scopes {
                    schedule(*scope);
                }
                info!(
                    "Reran {} scopes subscribed to asset with id '{}'",
                    scopes.len(),
                    asset_config.id
                );
            }
        }

        registry.get(asset_config).map(|s| s.asset_bytes.clone())
    }

    /// Read the size of the cache registry.
    pub fn size(&self) -> usize {
        self.registry.read().len()
    }
}

/// Get access to the global cache of assets.
pub fn use_asset_cacher() -> AssetCacher {
    use_context()
}

/// Initialize the global cache of assets.
///
/// This is a **low level** hook that **runs by default** in all Freya apps, you don't need it.
pub fn use_init_asset_cacher() {
    use_context_provider(AssetCacher::default);
}
