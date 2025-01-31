use std::ops::{
    Deref,
    DerefMut,
};

use freya_engine::prelude::Image;
use rustc_hash::FxHashMap;

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
pub struct ImageCacheKey(pub String);

#[derive(Default)]
pub struct ImagesCache {
    cache: FxHashMap<ImageCacheKey, Image>,
}

impl Deref for ImagesCache {
    type Target = FxHashMap<ImageCacheKey, Image>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl DerefMut for ImagesCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cache
    }
}
