use std::{
    cell::RefCell,
    io::ErrorKind,
    net::TcpListener,
    rc::Rc,
    time::Duration,
};

use freya_html::{
    HtmlContent,
    HtmlHandle,
    HtmlSource,
    HtmlViewer,
    ResourceCache,
    ResourceResponse,
};
use freya_testing::prelude::*;
use url::Url;

const CSS: &[u8] = include_bytes!("assets/style.css");
const CACHED_CSS: &[u8] = b"body { margin: 0; background: rgb(255, 0, 0); }";

fn assert_stylesheet_resolution(
    base_url: &str,
    response: ResourceResponse,
    cache_stylesheet: bool,
) {
    let expected_css = match &response {
        ResourceResponse::Bytes(bytes) => bytes.clone(),
        ResourceResponse::Block => Bytes::new(),
        ResourceResponse::Fetch => Bytes::from_static(CACHED_CSS),
    };
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
            if cache_stylesheet {
                ResourceCache::get().set(stylesheet_url.clone(), Bytes::from_static(CACHED_CSS));
            }
            let requests = Rc::new(RefCell::new(Vec::<Url>::new()));
            let captured_requests = requests.clone();
            let handle = runner.provide_root_context(|| {
                let mut handle = HtmlHandle::create().resource_interceptor(move |url| {
                    captured_requests.borrow_mut().push(url.clone());
                    if url.path() == "/style.css" {
                        response.clone()
                    } else {
                        ResourceResponse::Fetch
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

    handle.load_html(format!(
        "<html><head><style>{}</style></head><body></body></html>",
        std::str::from_utf8(&expected_css).unwrap()
    ));
    runner.sync_and_update();
    runner.poll_n(Duration::ZERO, 8);
    assert_eq!(actual.as_bytes(), runner.render().as_bytes());
    if cache_stylesheet {
        assert_eq!(
            ResourceCache::get().read(&stylesheet_url).unwrap().as_ref(),
            CACHED_CSS
        );
    } else {
        assert!(ResourceCache::get().read(&stylesheet_url).is_none());
    }
    contexts.unregister();
}

#[test]
fn embedded_relative_stylesheet_overrides_shared_cache() {
    assert_stylesheet_resolution(
        "https://test.local/",
        ResourceResponse::Bytes(Bytes::from_static(CSS)),
        true,
    );
}

#[test]
fn embedded_stylesheet_supports_custom_schemes() {
    assert_stylesheet_resolution(
        "asset://app/",
        ResourceResponse::Bytes(Bytes::from_static(CSS)),
        true,
    );
}

#[test]
fn unhandled_stylesheet_falls_back_to_shared_cache() {
    assert_stylesheet_resolution("https://test.local/", ResourceResponse::Fetch, true);
}

#[test]
fn blocked_stylesheet_ignores_shared_cache() {
    assert_stylesheet_resolution("https://test.local/", ResourceResponse::Block, true);
}

#[test]
fn blocked_stylesheet_does_not_fetch() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let base_url = format!("http://{}/", listener.local_addr().unwrap());
    assert_stylesheet_resolution(&base_url, ResourceResponse::Block, false);
    assert_eq!(listener.accept().unwrap_err().kind(), ErrorKind::WouldBlock);
}

#[test]
fn blocked_link_keeps_document_and_forward_history() {
    let (mut runner, (mut handle, requests)) = TestingRunner::new(
        || HtmlViewer::new(use_consume::<HtmlHandle>()),
        (100., 100.).into(),
        |runner| {
            let requests = Rc::new(RefCell::new(Vec::new()));
            let captured_requests = requests.clone();
            let handle = runner.provide_root_context(|| {
                HtmlHandle::create().resource_interceptor(move |url| {
                    captured_requests.borrow_mut().push(url);
                    ResourceResponse::Block
                })
            });
            (handle, requests)
        },
        1.,
    );
    let contexts = GlobalContexts::get();
    let destination = "https://test.local/blocked";
    handle.load_html(format!(
        "<body style='margin:0'><a style='display:block;height:30px' href='{destination}'>link</a></body>"
    ));
    handle.load_html("<body style='background:red'></body>");
    handle.back();
    runner.sync_and_update();
    let before = runner.render();

    runner.click_cursor((10., 10.));
    runner.poll_n(Duration::ZERO, 8);
    assert_eq!(*requests.borrow(), [Url::parse(destination).unwrap()]);
    assert_eq!(handle.current_url(), None);
    assert!(!handle.can_go_back());
    assert!(handle.can_go_forward());
    assert_eq!(before.as_bytes(), runner.render().as_bytes());
    contexts.unregister();
}

#[test]
fn document_interceptor_supplies_bytes_and_rechecks_history() {
    let (mut runner, mut handle) = TestingRunner::new(
        || HtmlViewer::new(use_consume::<HtmlHandle>()),
        (100., 100.).into(),
        |runner| runner.provide_root_context(HtmlHandle::create),
        1.,
    );
    let contexts = GlobalContexts::get();
    let url = "asset://app/page.html";
    let html = "<body style='background:red'></body>";
    handle = handle.resource_interceptor(move |requested| {
        assert_eq!(requested.as_str(), url);
        ResourceResponse::Bytes(Bytes::from_static(html.as_bytes()))
    });
    handle.load_html(html);
    runner.sync_and_update();
    let expected = runner.render();
    handle.replace_html("<body style='background:blue'></body>");
    handle.navigate(url);
    runner.sync_and_update();
    assert_eq!(handle.current_url().as_deref(), Some(url));
    assert_eq!(expected.as_bytes(), runner.render().as_bytes());

    let requests = Rc::new(RefCell::new(Vec::new()));
    let captured_requests = requests.clone();
    handle = handle.resource_interceptor(move |url| {
        captured_requests.borrow_mut().push(url);
        ResourceResponse::Block
    });
    handle.reload();
    runner.sync_and_update();
    assert_eq!(expected.as_bytes(), runner.render().as_bytes());
    handle.back();
    handle.forward();
    assert_eq!(*requests.borrow(), vec![Url::parse(url).unwrap(); 2]);
    assert_eq!(handle.current_url(), None);
    assert!(handle.can_go_forward());
    contexts.unregister();
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
