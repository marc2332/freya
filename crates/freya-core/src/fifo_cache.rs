use std::collections::VecDeque;

use rustc_hash::FxHashMap;

pub struct FifoCache<K, V> {
    map: FxHashMap<K, V>,
    order: VecDeque<K>,
    capacity: usize,
}

impl<K, V> Default for FifoCache<K, V> {
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
            order: VecDeque::new(),
            capacity: 256,
        }
    }
}

impl<K, V> FifoCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
        } else {
            if self.order.len() >= self.capacity
                && let Some(old_key) = self.order.pop_front()
            {
                self.map.remove(&old_key);
            }
            self.order.push_back(key.clone());
            self.map.insert(key, value);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
    }
}

impl<K, V> std::fmt::Debug for FifoCache<K, V>
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FifoCache")
            .field("map", &self.map)
            .field("order", &self.order)
            .field("capacity", &self.capacity)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::FifoCache;

    #[test]
    fn fifo_cache() {
        let mut cache = FifoCache::<i32, i32>::new();

        for i in 0..256 {
            cache.insert(i, i * 10);
        }

        assert_eq!(cache.len(), 256);
        assert_eq!(cache.get(&0), Some(&0));
        assert_eq!(cache.get(&255), Some(&2550));

        // Insert one more, should remove 0
        cache.insert(256, 2560);
        assert_eq!(cache.len(), 256);
        assert_eq!(cache.get(&0), None);
        assert_eq!(cache.get(&1), Some(&10));
        assert_eq!(cache.get(&256), Some(&2560));

        // Update existing
        cache.insert(1, 100);
        assert_eq!(cache.get(&1), Some(&100));
        assert_eq!(cache.len(), 256);
    }
}
