use std::{
    sync::Arc,
    thread::sleep,
    time::{
        Duration,
        Instant,
    },
};

use blitz_dom::DocumentConfig;
use blitz_html::HtmlDocument;
use blitz_paint::paint_scene;
use blitz_traits::{
    net::{
        Bytes,
        NetHandler,
        NetProvider,
        Request,
    },
    shell::{
        ColorScheme,
        Viewport,
    },
};
use freya_engine::prelude::{
    Color,
    raster_n32_premul,
};
use freya_html::anyrender::{
    SkiaSceneCache,
    SkiaScenePainter,
};

const HTML: &str = r#"<html><head><link rel="stylesheet" href="https://test.local/style.css"></head>
    <body style="margin:0"><div class="probe" style="width:200px;height:100px"></div></body></html>"#;
const CSS: &str = ".probe{transition-property:background-color;transition-duration:.15s;background-color:rgb(14 165 233)}";

struct DelayedCss;

impl NetProvider for DelayedCss {
    fn fetch(&self, _doc_id: usize, request: Request, handler: Box<dyn NetHandler>) {
        blocking::unblock(move || {
            sleep(Duration::from_millis(100));
            handler.bytes(request.url.to_string(), Bytes::from_static(CSS.as_bytes()));
        })
        .detach();
    }
}

/// A late stylesheet starts CSS transitions, resolving with an advancing clock must settle them.
#[test]
fn late_stylesheet_with_transition_settles() {
    let config = DocumentConfig {
        base_url: Some("https://test.local/".into()),
        net_provider: Some(Arc::new(DelayedCss)),
        ..Default::default()
    };
    let mut document = HtmlDocument::from_html(HTML, config);
    document.set_viewport(Viewport::new(200, 200, 1.0, ColorScheme::Light));

    let created = Instant::now();
    document.resolve(created.elapsed().as_secs_f64());
    sleep(Duration::from_millis(200));
    for _ in 0..100 {
        document.resolve(created.elapsed().as_secs_f64());
        if !document.is_animating() {
            break;
        }
        sleep(Duration::from_millis(16));
    }
    assert!(!document.is_animating(), "transition never settled");

    let mut surface = raster_n32_premul((200, 200)).unwrap();
    let mut cache = SkiaSceneCache::new();
    let mut painter = SkiaScenePainter::new(surface.canvas(), &mut cache);
    paint_scene(&mut painter, &mut document, 1.0, 200, 200, 0, 0);

    let color = surface.peek_pixels().unwrap().get_color((100, 50));
    assert_eq!(color, Color::new(0xFF0EA5E9));
}
