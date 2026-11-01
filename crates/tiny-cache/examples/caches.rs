use std::rc::Rc;

use tiny_cache::{
    FifoCache,
    LRUCache,
};

fn main() {
    let mut fifo = FifoCache::new(2);
    for key in 0..2 {
        fifo.insert(key, format!("Value {key}"));
    }
    fifo.insert(2, "Newest value".to_owned());
    assert!(fifo.get(&0).is_none());
    assert_eq!(fifo.len(), 2);
    fifo.set_capacity(1);
    assert_eq!(fifo.get(&1), None);
    assert_eq!(fifo.len(), 1);

    let mut shared = LRUCache::<String, u64>::default();
    let first = shared
        .utilize(1, &"greeting")
        .unwrap_or_else(|| shared.insert(1, &"greeting", "Hello".to_owned()));
    let second = shared
        .utilize(2, &"greeting")
        .unwrap_or_else(|| shared.insert(2, &"greeting", "Hello".to_owned()));
    assert!(Rc::ptr_eq(&first, &second));

    shared.remove(&1);
    assert_eq!(shared.len(), 1);
    shared.remove(&2);
    assert!(shared.is_empty());
    assert_eq!(first.as_str(), "Hello");
}
