# tiny-cache

Avoid doing the same work again when you already have the result. Save results for later, with a limit on how much you keep.

- **`FifoCache`** keeps a chosen number of results, removing the oldest added result when full. You can change the limit as your needs change.
- **`LRUCache`** lets several users share results they recently used. Once nobody needs a saved result, the cache lets it go.

## Count once, reuse later

For example, if you count the words in the same text several times, save the count rather than read through the text every time.

```rust
use tiny_cache::FifoCache;

fn count_words(saved: &mut FifoCache<String, usize>, text: String) -> usize {
    if let Some(&count) = saved.get(&text) {
        return count;
    }

    let count = text.split_whitespace().count();
    saved.insert(text, count);
    count
}

let mut saved = FifoCache::new(100);

assert_eq!(count_words(&mut saved, "Save results for later".into()), 4);
assert_eq!(count_words(&mut saved, "Save results for later".into()), 4);

saved.set_capacity(20);
```

The first call does the counting. The second reuses the saved count. Lowering the limit keeps the newest added results and removes any that no longer fit.

## Share results between users

Choose `LRUCache` when different parts of your program need the same result. In Freya, several labels can reuse the work of preparing the same text for drawing. Removing one label should not discard work another label still needs.

Each user keeps its last two results by default. Call `utilize` to look for a saved result, `insert` if you need to create it, and `remove` when a user no longer needs its results.

Run the example showing both caches:

```sh
cargo run -p tiny-cache --example caches
```
