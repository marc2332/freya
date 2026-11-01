#[cfg(feature = "std")]
pub use std::{
    collections::HashMap,
    hash::DefaultHasher,
};

#[cfg(feature = "std")]
pub use rustc_hash::FxHashMap;

#[cfg(not(feature = "std"))]
pub type FxHashMap<Key, Value> = hashbrown::HashMap<Key, Value, rustc_hash::FxBuildHasher>;

#[cfg(not(feature = "std"))]
pub type HashMap<Key, Value> = FxHashMap<Key, Value>;

#[cfg(not(feature = "std"))]
pub use rustc_hash::FxHasher as DefaultHasher;
