//! Save costly results so your program can reuse them instead of doing the same work again.
//! Choose [`FifoCache`] for a fixed limit, or [`LRUCache`] when users should share their recent results.

pub mod fifo_cache;
pub mod lru_cache;

pub use crate::{
    fifo_cache::FifoCache,
    lru_cache::LRUCache,
};
