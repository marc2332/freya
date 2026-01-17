#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_webview::prelude::*;

fn main() {
    tracing_subscriber::fmt::init();

    launch(
        LaunchConfig::new()
            .with_plugin(WebViewPlugin::new())
            .with_window(WindowConfig::new(app).with_size(1024., 768.)),
    )
}

#[derive(Clone, PartialEq)]
struct Webview {
    id: WebViewId,
    url: String,
}

fn app() -> impl IntoElement {
    use_init_root_theme(|| DARK_THEME);
    let mut webviews = use_state(|| {
        vec![Webview {
            id: WebViewId::new(),
            url: "https://duckduckgo.com".to_string(),
        }]
    });

    rect()
        .expanded()
        .background((35, 35, 35))
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(45.))
                .padding(4.)
                .background((50, 50, 50))
                .horizontal()
                .cross_align(Alignment::Center)
                .spacing(4.)
                .child(
                    Button::new()
                        .on_press(move |_| {
                            webviews.write().push(Webview {
                                id: WebViewId::new(),
                                url: "https://duckduckgo.com".to_string(),
                            });
                        })
                        .background((60, 60, 60))
                        .color(Color::WHITE)
                        .corner_radius(4.)
                        .child(label().text("+").font_size(16.)),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            if webviews.read().len() > 1 {
                                let removed = webviews.write().pop().unwrap();
                                WebViewManager::close(removed.id);
                            }
                        })
                        .background((60, 60, 60))
                        .color(Color::WHITE)
                        .corner_radius(4.)
                        .child(label().text("-").font_size(16.)),
                ),
        )
        .child(
            ResizableContainer::new()
                .direction(Direction::Horizontal)
                .panels_iter(webviews.read().iter().enumerate().map(|(i, webview)| {
                    ResizablePanel::new(50.).key(&webview.id).order(i).child(
                        WebViewComponent::new(&webview.url)
                            .expanded()
                            .id(webview.id),
                    )
                })),
        )
}
