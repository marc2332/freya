use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    hash::{
        DefaultHasher,
        Hash,
        Hasher,
    },
    rc::Rc,
    time::Duration,
};

use async_io::Timer;
use freya_core::{
    integration::FxHashSet,
    prelude::*,
};

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
    /// The ID of the asset.
    pub id: u64,
}

impl AssetConfiguration {
    pub fn new(id: impl Hash, age: AssetAge) -> Self {
        let mut state = DefaultHasher::default();
        id.hash(&mut state);
        let id = state.finish();
        Self { id, age }
    }
}

enum AssetUsers {
    Listeners(Rc<RefCell<FxHashSet<ReactiveContext>>>),
    ClearTask(TaskHandle),
}

#[derive(Clone)]
pub enum Asset {
    /// Asset is cached.
    Cached(Rc<dyn Any>),
    /// Asset is currently being fetched.
    Loading,
    /// Asset has yet to be fetched.
    Pending,
    /// Failed to fetch asset.
    Error(String),
}

impl Asset {
    /// Try to get asset.
    pub fn try_get(&self) -> Option<&Rc<dyn Any>> {
        match self {
            Self::Cached(asset) => Some(asset),
            _ => None,
        }
    }
}

struct AssetState {
    users: AssetUsers,
    asset: Asset,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct AssetCacher {
    registry: State<HashMap<AssetConfiguration, AssetState>>,
}

impl AssetCacher {
    pub fn try_get() -> Option<Self> {
        try_consume_context()
    }

    pub fn get() -> Self {
        Self::try_get().unwrap()
    }

    /// Attempt to resolve a [Asset] given a [AssetConfiguration].
    pub fn read_asset(&self, asset_config: &AssetConfiguration) -> Option<Asset> {
        self.registry
            .peek()
            .get(asset_config)
            .map(|a| a.asset.clone())
    }

    /// Subscribes to a [Asset] given a [AssetConfiguration].
    pub fn subscribe_asset(&self, asset_config: &AssetConfiguration) -> Option<Asset> {
        if let Some(rc) = ReactiveContext::current() {
            self.listen(rc, asset_config.clone());
        }
        self.registry
            .peek()
            .get(asset_config)
            .map(|a| a.asset.clone())
    }

    /// Update an [Asset] given a [AssetConfiguration].
    pub fn update_asset(&mut self, asset_config: AssetConfiguration, new_asset: Asset) {
        let mut registry = self.registry.write();

        let asset = registry
            .entry(asset_config.clone())
            .or_insert_with(|| AssetState {
                asset: Asset::Pending,
                users: AssetUsers::Listeners(Rc::default()),
            });

        asset.asset = new_asset;

        // Reruns those listening components
        if let AssetUsers::Listeners(listeners) = &asset.users {
            for sub in listeners.borrow().iter() {
                sub.notify();
            }
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
                        listeners.borrow().is_empty()
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
                        Timer::after(duration).await;
                        registry.write().remove(&asset_config);
                    }
                });

                // Registry the clear-task
                let mut registry = registry.write();
                let entry = registry.get_mut(asset_config).unwrap();
                entry.users = AssetUsers::ClearTask(clear_task);
            }
        }
    }

    pub(crate) fn listen(&self, mut rc: ReactiveContext, asset_config: AssetConfiguration) {
        let mut registry = self.registry.write_unchecked();

        registry
            .entry(asset_config.clone())
            .or_insert_with(|| AssetState {
                asset: Asset::Pending,
                users: AssetUsers::Listeners(Rc::default()),
            });

        if let Some(asset) = registry.get(&asset_config) {
            match &asset.users {
                AssetUsers::Listeners(users) => {
                    rc.subscribe(users);
                }
                AssetUsers::ClearTask(clear_task) => {
                    clear_task.cancel();
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
pub fn use_asset(asset_config: &AssetConfiguration) -> Asset {
    let mut asset_cacher = use_hook(AssetCacher::get);

    use_drop({
        let asset_config = asset_config.clone();
        move || {
            // Try to clean in the next async tick, when this scope will already be dropped
            spawn_forever(async move {
                asset_cacher.try_clean(&asset_config);
            });
        }
    });

    let mut prev = use_state::<Option<AssetConfiguration>>(|| None);
    {
        let mut prev = prev.write();
        if prev.as_ref() != Some(asset_config) {
            if let Some(prev) = &*prev
                && prev != asset_config
            {
                // Try to clean the previous asset
                asset_cacher.try_clean(asset_config);
            }
            prev.replace(asset_config.clone());
        }
        if let Some(rc) = ReactiveContext::current() {
            asset_cacher.listen(rc, asset_config.clone());
        }
    }

    asset_cacher.read_asset(asset_config).unwrap()
}
