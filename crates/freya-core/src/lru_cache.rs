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

pub struct LRUCache<V, ID: Hash> {
    map: FxHashMap<u64, (i32, Rc<V>)>,
    pub users: FxHashMap<ID, SmallVec<[u64; 2]>>,
}

impl<V, ID: Hash> Default for LRUCache<V, ID> {
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
            users: FxHashMap::default(),
        }
    }
}

impl<V, ID: Hash + Eq> LRUCache<V, ID> {
    pub fn get<H: Hash>(&self, hash_value: &H) -> Option<Rc<V>> {
        let mut hasher = FxHasher::default();
        hash_value.hash(&mut hasher);
        let hash = hasher.finish();
        let value = self.map.get(&hash);

        value.as_ref().map(|v| v.1.clone())
    }

    pub fn utilize<H: Hash>(&mut self, id: ID, hash_value: &H) -> Option<Rc<V>> {
        let mut hasher = FxHasher::default();
        hash_value.hash(&mut hasher);
        let hash = hasher.finish();
        let mut value = self.map.get_mut(&hash);

        let cache_value = value.as_ref().map(|v| v.1.clone());

        let hashes = self.users.entry(id).or_default();

        // New hashed value
        if !hashes.contains(&hash) {
            if let Some(value) = &mut value {
                value.0 += 1;
                hashes.push(hash);
            }

            let index = match hashes.len() {
                ..=1 => 0,
                2 => 1,
                len => len - 2,
            };
            // Clean the first current hash
            for old_hash in hashes.drain(0..index) {
                let Some(entry) = self.map.get_mut(&old_hash) else {
                    continue;
                };

                entry.0 -= 1;

                if entry.0 == 0 {
                    self.map.remove(&old_hash);
                }
            }
        }

        cache_value
    }

    pub fn insert<H: Hash>(&mut self, id: ID, hash_value: &H, value: V) -> Rc<V> {
        let mut hasher = FxHasher::default();
        hash_value.hash(&mut hasher);
        let hash = hasher.finish();
        let value = Rc::new(value);

        self.map.entry(hash).or_insert_with(|| (0, value.clone()));

        let user_hashes = self.users.entry(id).or_default();
        if !user_hashes.contains(&hash) {
            user_hashes.push(hash);
            self.map.get_mut(&hash).unwrap().0 += 1;
        }

        value
    }

    pub fn remove(&mut self, id: &ID) {
        let Some(hashes) = self.users.remove(id) else {
            return;
        };

        for hash in hashes.iter() {
            let Some(entry) = self.map.get_mut(hash) else {
                continue;
            };

            entry.0 -= 1;

            if entry.0 == 0 {
                self.map.remove(hash);
            }
        }
    }

    pub fn reset(&mut self) {
        self.map.clear();
        self.users.clear();
    }

    pub fn print_metrics(&self) {
        println!("Cached Values {}", self.map.len());
        println!("Cache Users {}", self.users.len());
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::lru_cache::LRUCache;

    #[test]
    fn lru_cache() {
        let mut cache = LRUCache::<i32, u64>::default();

        cache
            .utilize(1, &50)
            .unwrap_or_else(|| cache.insert(1, &50, 5000));

        assert_eq!(cache.utilize(1, &50), Some(Rc::new(5000)));

        cache
            .utilize(1, &60)
            .unwrap_or_else(|| cache.insert(1, &60, 6000));
        assert_eq!(cache.utilize(1, &60), Some(Rc::new(6000)));
        assert_eq!(cache.utilize(1, &50), Some(Rc::new(5000)));

        cache
            .utilize(1, &70)
            .unwrap_or_else(|| cache.insert(1, &70, 7000));
        assert!(cache.get(&50).is_none());

        assert_eq!(cache.utilize(1, &60), Some(Rc::new(6000)));
        assert_eq!(cache.utilize(1, &70), Some(Rc::new(7000)));
        assert_eq!(cache.utilize(1, &60), Some(Rc::new(6000)));
        assert_eq!(cache.utilize(1, &70), Some(Rc::new(7000)));

        cache.remove(&1);
        assert!(cache.get(&60).is_none());
        assert!(cache.get(&70).is_none());

        cache
            .utilize(1, &70)
            .unwrap_or_else(|| cache.insert(1, &70, 7000));
        assert!(cache.utilize(2, &70).is_some());

        cache.remove(&1);
        assert!(cache.get(&70).is_some());
        cache.remove(&2);
        assert!(cache.get(&70).is_none());
    }
}
