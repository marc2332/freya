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
struct Tab {
    id: WebViewId,
    title: String,
    url: String,
}

fn app() -> impl IntoElement {
    use_init_root_theme(|| DARK_THEME);
    let mut tabs = use_state(|| {
        vec![Tab {
            id: WebViewId::new(),
            title: "Tab 1".to_string(),
            url: "https://duckduckgo.com".to_string(),
        }]
    });
    let mut active_tab = use_state(|| tabs.read()[0].id);

    rect()
        .expanded()
        .height(Size::fill())
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
                .children(tabs.read().iter().map(|tab| {
                    let tab_id = tab.id;
                    let is_active = *active_tab.read() == tab_id;
                    let title = tab.title.clone();

                    Button::new()
                        .on_press(move |_| {
                            active_tab.set(tab_id);
                        })
                        .background(if is_active {
                            (70, 70, 70)
                        } else {
                            (45, 45, 45)
                        })
                        .color(if is_active {
                            Color::WHITE
                        } else {
                            Color::from_rgb(180, 180, 180)
                        })
                        .corner_radius(4.)
                        .width(Size::px(120.))
                        .child(
                            rect()
                                .horizontal()
                                .expanded()
                                .content(Content::flex())
                                .cross_align(Alignment::Center)
                                .font_size(14.)
                                .child(
                                    label()
                                        .width(Size::flex(1.))
                                        .max_lines(1)
                                        .font_size(14.)
                                        .text_overflow(TextOverflow::Ellipsis)
                                        .text(title),
                                )
                                .child(
                                    Button::new()
                                        .flat()
                                        .compact()
                                        .rounded_full()
                                        .on_press(move |e: Event<PressEventData>| {
                                            e.prevent_default();
                                            e.stop_propagation();
                                            tabs.write().retain(|t| t.id != tab_id);
                                            WebViewManager::close(tab_id);
                                            if !tabs.read().is_empty() {
                                                active_tab.set(tabs.read()[0].id);
                                            }
                                        })
                                        .child("X"),
                                ),
                        )
                        .into()
                }))
                .child(
                    Button::new()
                        .on_press(move |_| {
                            let id = WebViewId::new();
                            tabs.write().push(Tab {
                                id,
                                title: format!("Tab {:?}", id),
                                url: "https://duckduckgo.com".to_string(),
                            });
                            active_tab.set(id);
                        })
                        .background((60, 60, 60))
                        .color(Color::WHITE)
                        .rounded_full()
                        .child(label().text("New").font_size(14.)),
                ),
        )
        .child(
            rect()
                .expanded()
                .children(tabs.read().iter().filter_map(|tab| {
                    let is_active = *active_tab.read() == tab.id;
                    let url = tab.url.clone();

                    if is_active {
                        Some(
                            WebView::new(&url)
                                .expanded()
                                .id(tab.id)
                                .close_on_drop(false)
                                .into(),
                        )
                    } else {
                        None
                    }
                })),
        )
}
