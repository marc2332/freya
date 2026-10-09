#[cfg(feature = "std")]
pub use std::collections::HashSet;

#[cfg(feature = "std")]
pub use rustc_hash::{
    FxHashMap,
    FxHashSet,
};

#[cfg(not(feature = "std"))]
pub type FxHashMap<Key, Value> = hashbrown::HashMap<Key, Value, rustc_hash::FxBuildHasher>;

#[cfg(not(feature = "std"))]
pub type FxHashSet<Value> = hashbrown::HashSet<Value, rustc_hash::FxBuildHasher>;

#[cfg(not(feature = "std"))]
pub type HashSet<Value> = FxHashSet<Value>;
