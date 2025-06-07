use std::any::{
    Any,
    TypeId,
};

use rustc_hash::FxHashMap;

/// A map of types that can be sent between threads
#[derive(Debug)]
pub struct SendAnyMap {
    map: FxHashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>,
}

impl Default for SendAnyMap {
    fn default() -> Self {
        Self::new()
    }
}

impl SendAnyMap {
    pub fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|any| any.downcast_ref::<T>())
    }

    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }
}
