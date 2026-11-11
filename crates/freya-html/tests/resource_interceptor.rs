use std::{
    cell::RefCell,
    rc::Rc,
    time::Duration,
};

use freya_html::{
    HtmlContent,
    HtmlHandle,
    HtmlSource,
    HtmlViewer,
    ResourceCache,
};
use freya_testing::prelude::*;
use url::Url;

const CSS: &[u8] = include_bytes!("assets/style.css");
const CACHED_CSS: &[u8] = b"body { margin: 0; background: rgb(255, 0, 0); }";

fn assert_stylesheet_resolution(base_url: &str, intercept: bool) {
    let stylesheet_url = Url::parse(base_url).unwrap().join("./style.css").unwrap();
    let content = HtmlContent::new(
        r#"<html><head><link rel="stylesheet" href="./style.css"></head><body></body></html>"#,
    )
    .with_base_url(base_url)
    .unwrap();

    let (mut runner, (mut handle, requests, mut visible)) = TestingRunner::new(
        || {
            let handle = use_consume::<HtmlHandle>();
            let visible = use_consume::<State<bool>>();
            rect()
                .expanded()
                .maybe_child((*visible.read()).then(|| HtmlViewer::new(handle)))
        },
        (100., 100.).into(),
        |runner| {
            ResourceCache::get().set(stylesheet_url.clone(), Bytes::from_static(CACHED_CSS));
            let requests = Rc::new(RefCell::new(Vec::<Url>::new()));
            let captured_requests = requests.clone();
            let handle = runner.provide_root_context(|| {
                let mut handle = HtmlHandle::create().resource_interceptor(move |url| {
                    captured_requests.borrow_mut().push(url.clone());
                    if intercept && url.path() == "/style.css" {
                        Some(Bytes::from_static(CSS))
                    } else {
                        None
                    }
                });
                handle.load(HtmlSource::html(content));
                handle
            });
            let visible = runner.provide_root_context(|| State::create(false));
            (handle, requests, visible)
        },
        1.,
    );
    let contexts = GlobalContexts::get();
    runner.sync_and_update();
    runner.poll_n(Duration::ZERO, 8);
    assert_eq!(*requests.borrow(), vec![stylesheet_url.clone()]);
    visible.set(true);
    runner.sync_and_update();
    runner.poll_n(Duration::ZERO, 8);
    let actual = runner.render();

    visible.set(false);
    runner.sync_and_update();
    visible.set(true);
    runner.sync_and_update();
    runner.poll_n(Duration::ZERO, 8);
    assert_eq!(actual.as_bytes(), runner.render().as_bytes());
    assert_eq!(*requests.borrow(), vec![stylesheet_url.clone()]);

    let expected_css = if intercept { CSS } else { CACHED_CSS };
    handle.load_html(format!(
        "<html><head><style>{}</style></head><body></body></html>",
        std::str::from_utf8(expected_css).unwrap()
    ));
    runner.sync_and_update();
    runner.poll_n(Duration::ZERO, 8);
    assert_eq!(actual.as_bytes(), runner.render().as_bytes());
    assert_eq!(
        ResourceCache::get().read(&stylesheet_url).unwrap().as_ref(),
        CACHED_CSS
    );
    contexts.unregister();
}

#[test]
fn embedded_relative_stylesheet_overrides_shared_cache() {
    assert_stylesheet_resolution("https://test.local/", true);
}

#[test]
fn embedded_stylesheet_supports_custom_schemes() {
    assert_stylesheet_resolution("asset://app/", true);
}

#[test]
fn unhandled_stylesheet_falls_back_to_shared_cache() {
    assert_stylesheet_resolution("https://test.local/", false);
}

#[test]
fn loading_documents_updates_history() {
    let (mut runner, mut handle) = TestingRunner::new(
        || HtmlViewer::new(use_consume::<HtmlHandle>()),
        (100., 100.).into(),
        |runner| runner.provide_root_context(HtmlHandle::create),
        1.,
    );
    let contexts = GlobalContexts::get();
    runner.sync_and_update();
    assert!(!handle.can_go_back());
    assert!(!handle.can_go_forward());
    assert_eq!(handle.current_url(), None);

    handle.load(HtmlSource::html("<body style='background:red'></body>"));
    runner.sync_and_update();
    let first = runner.render();
    assert!(!handle.can_go_back());

    handle.load(HtmlSource::html("<body style='background:blue'></body>"));
    runner.sync_and_update();
    let second = runner.render();
    assert_ne!(first.as_bytes(), second.as_bytes());
    assert!(handle.can_go_back());

    handle.back();
    runner.sync_and_update();
    assert_eq!(first.as_bytes(), runner.render().as_bytes());
    assert!(handle.can_go_forward());

    handle.forward();
    runner.sync_and_update();
    assert_eq!(second.as_bytes(), runner.render().as_bytes());

    handle.back();
    handle.load(HtmlSource::html("<body style='background:green'></body>"));
    runner.sync_and_update();
    assert!(handle.can_go_back());
    assert!(!handle.can_go_forward());
    contexts.unregister();
}
