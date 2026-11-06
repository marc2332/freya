use blitz_traits::net::Bytes;
use freya_core::prelude::GlobalContexts;
use freya_html::ResourceCache;
use url::Url;

fn url(name: &str) -> Url {
    Url::parse(&format!("https://example.com/{name}")).unwrap()
}

fn assert_cached(cache: &ResourceCache, name: &str, expected: &[u8]) {
    assert_eq!(cache.read(&url(name)).as_deref(), Some(expected), "{name}");
}

#[test]
fn reads_refresh_eviction_order_across_clones() {
    let cache = ResourceCache::new(6);
    let shared = cache.clone();
    cache.set(url("first"), Bytes::from_static(b"abc"));
    cache.set(url("second"), Bytes::from_static(b"def"));
    for _ in 0..2 {
        assert_cached(&shared, "first", b"abc");
    }
    assert!(shared.read(&url("missing")).is_none());
    shared.set(url("third"), Bytes::from_static(b"ghi"));

    assert!(cache.read(&url("second")).is_none());
    assert_cached(&cache, "first", b"abc");
    assert_cached(&cache, "third", b"ghi");
}

#[test]
fn replacements_update_the_byte_budget() {
    let cache = ResourceCache::new(6);
    cache.set(url("first"), Bytes::from_static(b"abcd"));
    cache.set(url("second"), Bytes::from_static(b"ef"));
    cache.set(url("first"), Bytes::from_static(b"a"));
    cache.set(url("third"), Bytes::from_static(b"ghi"));
    assert_cached(&cache, "second", b"ef");
    assert_cached(&cache, "first", b"a");
    assert_cached(&cache, "third", b"ghi");

    cache.set(url("first"), Bytes::from_static(b"abcdef"));
    assert!(cache.read(&url("second")).is_none());
    assert!(cache.read(&url("third")).is_none());
    assert_cached(&cache, "first", b"abcdef");
}

#[test]
fn oversized_and_empty_bodies_are_not_cached() {
    let cache = ResourceCache::new(3);
    cache.set(url("first"), Bytes::from_static(b"a"));
    cache.set(url("second"), Bytes::from_static(b"b"));
    cache.set(url("first"), Bytes::from_static(b"abcd"));
    assert!(cache.read(&url("first")).is_none());
    assert!(cache.read(&url("second")).is_some());

    cache.set(url("second"), Bytes::new());
    assert!(cache.read(&url("second")).is_none());

    let disabled = ResourceCache::new(0);
    disabled.set(url("first"), Bytes::from_static(b"a"));
    assert!(disabled.read(&url("first")).is_none());
}

#[test]
fn invalidate_and_clear_release_the_budget_without_invalidating_readers() {
    let cache = ResourceCache::new(4);
    cache.set(url("first"), Bytes::from_static(b"ab"));
    cache.set(url("second"), Bytes::from_static(b"cd"));
    let retained = cache.read(&url("first")).unwrap();
    cache.invalidate(&url("first"));
    cache.invalidate(&url("missing"));
    cache.set(url("third"), Bytes::from_static(b"ef"));
    assert!(cache.read(&url("second")).is_some());
    assert!(cache.read(&url("third")).is_some());
    assert_eq!(retained.as_ref(), b"ab");

    cache.clone().clear();
    cache.set(url("fourth"), Bytes::from_static(b"ghij"));
    assert!(cache.read(&url("second")).is_none());
    assert!(cache.read(&url("third")).is_none());
    assert_cached(&cache, "fourth", b"ghij");
}

#[test]
fn application_context_uses_the_configured_budget() {
    let contexts = GlobalContexts::register();
    contexts.insert_context(ResourceCache::new(3));
    let first = ResourceCache::get();
    let second = ResourceCache::get();
    first.set(url("first"), Bytes::from_static(b"abc"));
    second.set(url("second"), Bytes::from_static(b"def"));
    assert!(first.read(&url("first")).is_none());
    assert_cached(&first, "second", b"def");
    contexts.unregister();
}
