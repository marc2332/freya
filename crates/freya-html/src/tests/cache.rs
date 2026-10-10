use std::sync::mpsc::{
    Sender,
    channel,
};

use blitz_traits::net::{
    Bytes,
    NetHandler,
    NetProvider,
    Request,
};
use futures_channel::mpsc::unbounded;
use url::Url;

use crate::{
    cache::ResourceCache,
    net::HttpNetProvider,
};

struct CaptureHandler(Sender<(String, Bytes)>);

impl NetHandler for CaptureHandler {
    fn bytes(self: Box<Self>, resolved_url: String, bytes: Bytes) {
        self.0.send((resolved_url, bytes)).unwrap();
    }
}

#[test]
fn cached_resources_are_delivered_immediately_across_providers() {
    let cache = ResourceCache::default();
    let (fetch, mut requests) = unbounded();
    let first = HttpNetProvider {
        fetch: fetch.clone(),
        cache: cache.clone(),
    };
    let second = HttpNetProvider { fetch, cache };
    let url = Url::parse("https://example.com/image.png").unwrap();
    let bytes = Bytes::from_static(b"image bytes");
    first.cache.set(url.clone(), bytes.clone());

    for provider in [first, second] {
        let (sender, received) = channel();
        provider.fetch(0, Request::get(url.clone()), Box::new(CaptureHandler(sender)));
        assert_eq!(received.try_recv().unwrap(), (url.to_string(), bytes.clone()));
    }
    assert!(requests.try_recv().is_err());
}

#[test]
fn uncached_resources_are_forwarded_without_delivering_bytes() {
    let (fetch, mut requests) = unbounded();
    let provider = HttpNetProvider {
        fetch,
        cache: ResourceCache::default(),
    };
    let url = Url::parse("https://example.com/style.css").unwrap();
    let (sender, received) = channel();

    provider.fetch(0, Request::get(url.clone()), Box::new(CaptureHandler(sender)));

    let (requested_url, handler) = requests.try_recv().unwrap();
    assert_eq!(requested_url, url);
    assert!(received.try_recv().is_err());
    let bytes = Bytes::from_static(b"body {}");
    handler.bytes(url.to_string(), bytes.clone());
    assert_eq!(received.try_recv().unwrap(), (url.to_string(), bytes));
    assert!(provider.cache.read(&url).is_none());
}
