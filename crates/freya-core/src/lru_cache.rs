use std::{
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use rustc_hash::{
    FxHashMap,
    FxHasher,
};
use smallvec::SmallVec;

/// Cache of values shared between users, where each user keeps its last `USER_CAPACITY`
/// used values alive. A value is dropped once no user uses it.
pub struct LRUCache<V, ID: Hash, const USER_CAPACITY: usize = 2> {
    map: FxHashMap<u64, (i32, Rc<V>)>,
    pub users: FxHashMap<ID, SmallVec<[u64; 2]>>,
}

impl<V, ID: Hash, const USER_CAPACITY: usize> Default for LRUCache<V, ID, USER_CAPACITY> {
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
            users: FxHashMap::default(),
        }
    }
}

impl<V, ID: Hash + Eq, const USER_CAPACITY: usize> LRUCache<V, ID, USER_CAPACITY> {
    fn hash_of<H: Hash>(hash_value: &H) -> u64 {
        let mut hasher = FxHasher::default();
        hash_value.hash(&mut hasher);
        hasher.finish()
    }

    /// Marks `hash_value` as the last value used by `id` and returns it if cached.
    /// When not cached it returns `None` and leaves a free slot for the [LRUCache::insert] that follows.
    pub fn utilize<H: Hash>(&mut self, id: ID, hash_value: &H) -> Option<Rc<V>> {
        let hash = Self::hash_of(hash_value);
        let entry = self.map.get_mut(&hash);
        let cached_value = entry.as_ref().map(|(_, value)| value.clone());

        let hashes = self.users.entry(id).or_default();

        if let Some(position) = hashes.iter().position(|used| *used == hash) {
            hashes.remove(position);
            hashes.push(hash);

            return cached_value;
        }

        if let Some((users, _)) = entry {
            *users += 1;
            hashes.push(hash);
        }

        let keep = if cached_value.is_some() {
            USER_CAPACITY
        } else {
            USER_CAPACITY.saturating_sub(1)
        };
        let extra = hashes.len().saturating_sub(keep);

        for old_hash in hashes.drain(0..extra) {
            let Some((users, _)) = self.map.get_mut(&old_hash) else {
                continue;
            };

            *users -= 1;

            if *users == 0 {
                self.map.remove(&old_hash);
            }
        }

        cached_value
    }

    /// Saves `value` for `hash_value` and marks it as used by `id`.
    pub fn insert<H: Hash>(&mut self, id: ID, hash_value: &H, value: V) -> Rc<V> {
        let hash = Self::hash_of(hash_value);
        let value = Rc::new(value);

        let (users, _) = self.map.entry(hash).or_insert_with(|| (0, value.clone()));
        let user_hashes = self.users.entry(id).or_default();

        if !user_hashes.contains(&hash) {
            user_hashes.push(hash);
            *users += 1;
        }

        value
    }

    pub fn remove(&mut self, id: &ID) {
        let Some(hashes) = self.users.remove(id) else {
            return;
        };

        for hash in hashes {
            let Some((users, _)) = self.map.get_mut(&hash) else {
                continue;
            };

            *users -= 1;

            if *users == 0 {
                self.map.remove(&hash);
            }
        }
    }

    pub fn reset(&mut self) {
        self.map.clear();
        self.users.clear();
    }
}

impl<V, ID: Hash, const USER_CAPACITY: usize> std::fmt::Debug for LRUCache<V, ID, USER_CAPACITY> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LRUCache")
            .field("cached_values", &self.map.len())
            .field("cached_users", &self.users.len())
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::lru_cache::LRUCache;

    type Cache = LRUCache<i32, u64>;

    #[test]
    fn lru_cache() {
        let mut cache = Cache::default();

        cache
            .utilize(1, &50)
            .unwrap_or_else(|| cache.insert(1, &50, 500));
        assert_eq!(cache.utilize(1, &50), Some(Rc::new(500)));

        cache
            .utilize(1, &60)
            .unwrap_or_else(|| cache.insert(1, &60, 600));
        assert_eq!(cache.utilize(1, &60), Some(Rc::new(600)));
        assert_eq!(cache.utilize(1, &50), Some(Rc::new(500)));

        cache
            .utilize(1, &70)
            .unwrap_or_else(|| cache.insert(1, &70, 700));
        assert!(!cache.map.contains_key(&Cache::hash_of(&60)));
        assert!(cache.map.contains_key(&Cache::hash_of(&50)));

        cache.remove(&1);
        assert!(!cache.map.contains_key(&Cache::hash_of(&50)));
        assert!(!cache.map.contains_key(&Cache::hash_of(&70)));

        cache
            .utilize(1, &70)
            .unwrap_or_else(|| cache.insert(1, &70, 700));
        assert!(cache.utilize(2, &70).is_some());

        cache.remove(&1);
        assert!(cache.map.contains_key(&Cache::hash_of(&70)));
        cache.remove(&2);
        assert!(!cache.map.contains_key(&Cache::hash_of(&70)));
    }

    #[test]
    fn keeps_both_hashes_when_another_user_had_one() {
        let mut cache = Cache::default();

        cache
            .utilize(1, &10)
            .unwrap_or_else(|| cache.insert(1, &10, 100));
        cache
            .utilize(2, &20)
            .unwrap_or_else(|| cache.insert(2, &20, 200));
        cache
            .utilize(1, &20)
            .unwrap_or_else(|| cache.insert(1, &20, 200));

        assert_eq!(cache.users[&1].len(), 2);
        assert!(cache.map.contains_key(&Cache::hash_of(&10)));
        assert!(cache.map.contains_key(&Cache::hash_of(&20)));
    }

    #[test]
    fn only_keeps_two_hashes_per_user() {
        let mut cache = Cache::default();

        for hash in [10, 20, 30, 40] {
            let id = 1000 + hash as u64;
            cache
                .utilize(id, &hash)
                .unwrap_or_else(|| cache.insert(id, &hash, hash * 10));
        }

        for hash in [10, 20, 30, 40] {
            cache
                .utilize(1, &hash)
                .unwrap_or_else(|| cache.insert(1, &hash, hash * 10));
        }

        assert_eq!(cache.users[&1].len(), 2);
    }

    #[test]
    fn using_a_hash_again_keeps_it() {
        let mut cache = Cache::default();

        for hash in [10, 20, 10, 30] {
            cache
                .utilize(1, &hash)
                .unwrap_or_else(|| cache.insert(1, &hash, hash * 10));
        }

        assert!(cache.map.contains_key(&Cache::hash_of(&10)));
        assert!(!cache.map.contains_key(&Cache::hash_of(&20)));
        assert!(cache.map.contains_key(&Cache::hash_of(&30)));
    }
}
