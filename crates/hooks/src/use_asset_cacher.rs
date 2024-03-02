use dioxus_core::prelude::{provide_root_context, spawn, try_consume_context};
use dioxus_signals::Signal;
use dioxus_signals::{Readable, Writable};
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct AssetConfiguration {
    pub duration: Duration,
    pub id: String,
}

#[derive(Clone, Copy, Default)]
pub struct AssetCacher {
    registry: Signal<HashMap<AssetConfiguration, Signal<Vec<u8>>>>,
}

impl AssetCacher {
    /// Register an asset
    pub fn insert(&self, config: AssetConfiguration, asset: Vec<u8>) -> Signal<Vec<u8>> {
        let asset = Signal::new(asset);
        self.registry
            .try_write()
            .unwrap()
            .insert(config.clone(), asset);

        let registry = self.registry;

        spawn(async move {
            sleep(config.duration).await;
            registry.try_write().unwrap().remove(&config);
        });

        asset
    }

    /// Get an asset.
    pub fn get(&self, config: &AssetConfiguration) -> Option<Signal<Vec<u8>>> {
        self.registry.read().get(config).copied()
    }

    /// Remoe an asset.
    pub fn remove(&self, config: &AssetConfiguration) {
        self.registry.try_write().unwrap().remove(config);
    }

    /// Get the size of the registry;
    pub fn size(&self) -> usize {
        self.registry.read().len()
    }

    /// Clear all the assets from the registry.
    pub fn clear(&self) {
        self.registry.try_write().unwrap().clear();
    }
}

/// Global caching system for assets.
pub fn use_asset_cacher() -> AssetCacher {
    let asset_cacher = match try_consume_context() {
        Some(asset_cacher) => asset_cacher,
        None => provide_root_context(AssetCacher::default()),
    };

    asset_cacher
}
