use dioxus_core::prelude::{provide_root_context, spawn, try_consume_context};
use dioxus_signals::Signal;
use dioxus_signals::{Readable, Writable};
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;

pub enum AssetAge {
    Duration(Duration),
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

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AssetConfiguration {
    pub age: AssetAge,
    /// The ID of the asset. For example: For images their source URL or path can be used as ID.
    pub id: String,
}

#[derive(Clone, Copy, Default)]
pub struct AssetCacher {
    registry: Signal<HashMap<AssetConfiguration, Signal<Vec<u8>>>>,
}

impl AssetCacher {
    /// Cache the given [`AssetConfiguration`]
    pub fn cache(&self, asset_config: AssetConfiguration, asset: Vec<u8>) -> Signal<Vec<u8>> {
        let asset = Signal::new(asset);
        self.registry
            .try_write()
            .unwrap()
            .insert(asset_config.clone(), asset);

        let registry = self.registry;

        // Only clear the asset if a duration was specified
        if let AssetAge::Duration(duration) = asset_config.age {
            spawn(async move {
                sleep(duration).await;
                registry.try_write().unwrap().remove(&asset_config);
            });
        }
        asset
    }

    /// Get an asset Signal.
    pub fn get(&self, config: &AssetConfiguration) -> Option<Signal<Vec<u8>>> {
        self.registry.read().get(config).copied()
    }

    /// Remove an asset from the cache.
    pub fn remove(&self, config: &AssetConfiguration) {
        self.registry.try_write().unwrap().remove(config);
    }

    /// Get the size of the cache registry.
    pub fn size(&self) -> usize {
        self.registry.read().len()
    }

    /// Clear all the assets from the cache registry.
    pub fn clear(&self) {
        self.registry.try_write().unwrap().clear();
    }
}

/// Global caching system for assets.
pub fn use_asset_cacher() -> AssetCacher {
    match try_consume_context() {
        Some(asset_cacher) => asset_cacher,
        None => provide_root_context(AssetCacher::default()),
    }
}
