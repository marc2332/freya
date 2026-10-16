use std::collections::VecDeque;

use rustc_hash::FxHashMap;

/// A [First In, First Out (FIFO)](https://en.wikipedia.org/wiki/Cache_replacement_policies#First_in_first_out_(FIFO)) cache for reusing expensive results.
/// Keeps up to your chosen limit and removes the oldest added result when full.
///
/// # Example
///
/// ```
/// use tiny_cache::FifoCache;
///
/// let mut cache = FifoCache::new(2);
/// cache.insert("first", 10);
/// cache.insert("second", 20);
/// assert_eq!(cache.get(&"first"), Some(&10));
///
/// cache.insert("third", 30);
/// assert!(cache.get(&"first").is_none());
/// assert_eq!(cache.get(&"third"), Some(&30));
/// assert_eq!(cache.len(), 2);
/// ```
pub struct FifoCache<K, V> {
    map: FxHashMap<K, V>,
    order: VecDeque<K>,
    capacity: usize,
}

impl<K, V> FifoCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: FxHashMap::default(),
            order: VecDeque::new(),
            capacity,
        }
    }
}

impl<K, V> FifoCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
{
    /// Changes how many results to keep, removing the oldest added results if the limit shrinks.
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = capacity;
        while self.order.len() > capacity {
            if let Some(old_key) = self.order.pop_front() {
                self.map.remove(&old_key);
            }
        }
    }

    /// Saves a result to reuse later, removing the oldest added result if there is no room.
    /// Replacing an existing result does not move it to the back of the line.
    pub fn insert(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return;
        }

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

    /// Lets you update a saved result without creating a replacement or changing its place in line.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub fn capacity(&self) -> usize {
        self.capacity
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
    use crate::FifoCache;

    #[test]
    fn custom_capacity() {
        let mut cache = FifoCache::new(2);
        cache.insert(1, "first");
        cache.insert(2, "second");
        cache.insert(1, "updated");
        assert_eq!(cache.get(&1), Some(&"updated"));
        assert_eq!(cache.len(), 2);

        cache.insert(3, "third");
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"second"));
        assert_eq!(cache.get(&3), Some(&"third"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn zero_capacity_keeps_nothing() {
        let mut cache = FifoCache::new(0);
        for key in 0..3 {
            cache.insert(key, key);
        }
        assert!(cache.is_empty());
        assert!(cache.order.is_empty());
        assert_eq!(cache.get(&0), None);
    }

    #[test]
    fn clear_preserves_custom_capacity() {
        let mut cache = FifoCache::new(1);
        cache.insert(1, "first");
        cache.clear();
        assert!(cache.is_empty());

        cache.insert(2, "second");
        cache.insert(3, "third");
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&"third"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn shrinking_removes_oldest_results() {
        let mut cache = FifoCache::new(4);
        for key in 0..4 {
            cache.insert(key, key * 10);
        }
        cache.insert(0, 100);

        cache.set_capacity(2);
        assert_eq!(cache.capacity(), 2);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&0), None);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&20));
        assert_eq!(cache.get(&3), Some(&30));

        cache.insert(4, 40);
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&30));
        assert_eq!(cache.get(&4), Some(&40));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn growing_preserves_saved_results() {
        let mut cache = FifoCache::new(1);
        cache.insert(1, "first");
        cache.set_capacity(3);
        cache.insert(2, "second");
        cache.insert(3, "third");

        assert_eq!(cache.capacity(), 3);
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&1), Some(&"first"));
        assert_eq!(cache.get(&2), Some(&"second"));
        assert_eq!(cache.get(&3), Some(&"third"));
    }

    #[test]
    fn shrinking_to_zero_then_growing() {
        let mut cache = FifoCache::new(2);
        cache.insert(1, "first");
        cache.insert(2, "second");
        cache.set_capacity(0);
        cache.insert(3, "third");

        assert_eq!(cache.capacity(), 0);
        assert!(cache.is_empty());
        assert!(cache.order.is_empty());

        cache.set_capacity(1);
        cache.insert(4, "fourth");
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&4), Some(&"fourth"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn unchanged_capacity_preserves_saved_results() {
        let mut cache = FifoCache::new(2);
        cache.insert(1, "first");
        cache.insert(2, "second");
        cache.set_capacity(2);

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&1), Some(&"first"));
        assert_eq!(cache.get(&2), Some(&"second"));
    }
}
