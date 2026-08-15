use std::hash::{
    Hash,
    Hasher,
};

use hashbrown::{
    Equivalent,
    HashMap,
};

struct CacheEntry<T> {
    resource: T,
    generation: usize,
}

pub(crate) struct GenerationalCache<K, T>
where
    K: Eq + Hash,
{
    resources: HashMap<K, CacheEntry<T>>,
    current_generation: usize,
    max_age: usize,
}

impl<K, T> Default for GenerationalCache<K, T>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            resources: Default::default(),
            current_generation: 0,
            max_age: 2,
        }
    }
}

impl<K, T> GenerationalCache<K, T>
where
    K: Eq + Hash,
{
    pub(crate) fn new(max_age: usize) -> Self {
        GenerationalCache {
            resources: HashMap::default(),
            current_generation: 0,
            max_age,
        }
    }

    pub(crate) fn next_gen(&mut self) {
        self.resources.retain(|_, entry| {
            self.current_generation.wrapping_sub(entry.generation) < self.max_age
        });

        self.current_generation = self.current_generation.wrapping_add(1);
    }

    #[allow(unused)]
    pub(crate) fn contains_key<Q>(&mut self, key: &Q) -> bool
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        self.resources.contains_key(key)
    }

    pub(crate) fn hit<Q>(&mut self, key: &Q) -> Option<&T>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        if let Some(entry) = self.resources.get_mut(key) {
            entry.generation = self.current_generation;
            Some(&entry.resource)
        } else {
            None
        }
    }

    pub(crate) fn insert(&mut self, key: K, resource: T) {
        let entry = CacheEntry {
            resource,
            generation: self.current_generation,
        };
        self.resources.insert(key, entry);
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct NormalizedTypefaceCacheKey {
    pub(crate) typeface_id: u64,
    pub(crate) typeface_index: u32,
    pub(crate) normalized_coords: Vec<i16>,
}

impl Hash for NormalizedTypefaceCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.typeface_id.hash(state);
        self.typeface_index.hash(state);
        for &coord in &self.normalized_coords {
            coord.hash(state);
        }
    }
}

pub(crate) struct NormalizedTypefaceCacheKeyBorrowed<'a> {
    pub(crate) typeface_id: u64,
    pub(crate) typeface_index: u32,
    pub(crate) normalized_coords: &'a [i16],
}

impl<'a> Hash for NormalizedTypefaceCacheKeyBorrowed<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.typeface_id.hash(state);
        self.typeface_index.hash(state);
        for &coord in self.normalized_coords {
            coord.hash(state);
        }
    }
}

impl Equivalent<NormalizedTypefaceCacheKey> for NormalizedTypefaceCacheKeyBorrowed<'_> {
    fn equivalent(&self, key: &NormalizedTypefaceCacheKey) -> bool {
        self.typeface_id == key.typeface_id
            && self.typeface_index == key.typeface_index
            && self.normalized_coords == key.normalized_coords
    }
}

pub(crate) struct FontCacheKeyBorrowed<'a> {
    pub(crate) typeface_id: u64,
    pub(crate) typeface_index: u32,
    pub(crate) normalized_coords: &'a [i16],
    pub(crate) font_size: u32,
    pub(crate) hint: bool,
}

impl<'a> Hash for FontCacheKeyBorrowed<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.typeface_id.hash(state);
        self.typeface_index.hash(state);
        for &coord in self.normalized_coords {
            coord.hash(state);
        }
        self.font_size.hash(state);
        self.hint.hash(state);
    }
}

#[derive(PartialEq, Eq)]
pub(crate) struct FontCacheKey {
    pub(crate) typeface_id: u64,
    pub(crate) typeface_index: u32,
    pub(crate) normalized_coords: Vec<i16>,
    pub(crate) font_size: u32,
    pub(crate) hint: bool,
}

impl Hash for FontCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.typeface_id.hash(state);
        self.typeface_index.hash(state);
        for &coord in &self.normalized_coords {
            coord.hash(state);
        }
        self.font_size.hash(state);
        self.hint.hash(state);
    }
}

impl Equivalent<FontCacheKey> for FontCacheKeyBorrowed<'_> {
    fn equivalent(&self, key: &FontCacheKey) -> bool {
        self.typeface_id == key.typeface_id
            && self.typeface_index == key.typeface_index
            && self.font_size == key.font_size
            && self.hint == key.hint
            && self.normalized_coords == key.normalized_coords
    }
}
