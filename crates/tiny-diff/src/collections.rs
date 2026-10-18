pub use alloc::{
    boxed::Box,
    collections::VecDeque,
    vec::Vec,
};

#[cfg(not(feature = "std"))]
pub type HashMap<Key, Value> = hashbrown::HashMap<Key, Value, rustc_hash::FxBuildHasher>;
#[cfg(feature = "std")]
pub type HashMap<Key, Value> = std::collections::HashMap<Key, Value, rustc_hash::FxBuildHasher>;
