use std::sync::{
    Arc,
    Mutex,
};

use blitz_dom::DocumentConfig;
use blitz_html::HtmlDocument;
use freya_html::{
    HtmlHandle,
    HtmlSource,
    HtmlViewer,
};
use freya_testing::prelude::*;
use parley::fontique::{
    Blob,
    FontInfoOverride,
};
use skia_safe::graphics;

static FONT_CACHE_LOCK: Mutex<()> = Mutex::new(());
const HTML: &str = r#"<body style="margin:0;font-family:system-ui">Hello notification
    <b>Bold</b><i>Italic</i><span style="font-variation-settings:'wght' 650">Variable</span></body>"#;

fn runner() -> (TestingRunner, (HtmlHandle, State<bool>)) {
    TestingRunner::new(
        || {
            let handle = use_consume::<HtmlHandle>();
            let visible = use_consume::<State<bool>>();
            rect()
                .expanded()
                .maybe_child((*visible.read()).then(|| HtmlViewer::new(handle)))
        },
        (300., 100.).into(),
        |runner| {
            let handle = runner.provide_root_context(|| HtmlHandle::create(HtmlSource::html(HTML)));
            let visible = runner.provide_root_context(|| State::create(true));
            (handle, visible)
        },
        1.,
    )
}

fn assert_font_reuse(mut update: impl FnMut(&mut TestingRunner, HtmlHandle, State<bool>)) {
    let _guard = FONT_CACHE_LOCK.lock().unwrap();
    graphics::purge_all_caches();
    let (mut runner, (handle, visible)) = runner();
    let contexts = GlobalContexts::get();
    runner.sync_and_update();
    let expected = runner.render();
    let glyph_cache_entries = graphics::font_cache_count_used();
    assert!(glyph_cache_entries > 0);

    for _ in 0..32 {
        update(&mut runner, handle, visible);
        runner.sync_and_update();
        assert_eq!(runner.render().as_bytes(), expected.as_bytes());
        assert_eq!(graphics::font_cache_count_used(), glyph_cache_entries);
    }

    contexts.unregister();
}

#[test]
fn reloaded_documents_reuse_system_fonts() {
    assert_font_reuse(|_, mut handle, _| handle.load_html(HTML));
}

#[test]
fn recreated_viewers_reuse_system_fonts() {
    assert_font_reuse(|runner, _, mut visible| {
        visible.set(false);
        runner.sync_and_update();
        visible.set(true);
    });
}

#[test]
fn document_font_registrations_do_not_modify_other_documents() {
    let _guard = FONT_CACHE_LOCK.lock().unwrap();
    let (mut runner, _) = runner();
    let contexts = GlobalContexts::get();
    runner.sync_and_update();
    let mut first = contexts.get_context::<blitz_dom::FontContext>();
    let mut second = contexts.get_context::<blitz_dom::FontContext>();
    let family_name = "FreyaDocumentOnlyFont";
    let registered = first.collection.register_fonts(
        Blob::new(Arc::new(blitz_dom::BULLET_FONT)),
        Some(FontInfoOverride {
            family_name: Some(family_name),
            ..Default::default()
        }),
    );
    assert!(!registered.is_empty());
    assert!(first.collection.family_id(family_name).is_some());
    assert!(second.collection.family_id(family_name).is_none());

    let mut document = HtmlDocument::from_html(
        HTML,
        DocumentConfig {
            font_ctx: Some(first),
            ..Default::default()
        },
    );
    document.resolve(0.);
    let mut system = contexts.get_context::<blitz_dom::FontContext>();
    assert!(system.collection.family_id(family_name).is_none());
    assert!(second.collection.family_id(family_name).is_none());

    contexts.unregister();
}
