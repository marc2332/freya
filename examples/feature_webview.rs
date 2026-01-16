//! WebView example demonstrating a simple tabs system.
//!
//! Run with:
//! ```sh
//! cargo run --example feature_webview
//! ```

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
    id: usize,
    title: String,
    url: String,
}

fn app() -> impl IntoElement {
    use_init_root_theme(|| DARK_THEME);
    let mut tabs = use_state(|| {
        vec![Tab {
            id: 0,
            title: "Tab 1".to_string(),
            url: "https://duckduckgo.com".to_string(),
        }]
    });
    let mut active_tab = use_state(|| 0usize);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background((35, 35, 35))
        .vertical()
        .child(
            // Tab bar
            rect()
                .width(Size::fill())
                .height(Size::px(45.))
                .padding(Gaps::new(4., 8., 0., 8.))
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
                        .child(
                            rect().horizontal().child(title).child(
                                Button::new()
                                    .flat()
                                    .compact()
                                    .on_press(move |e: Event<PressEventData>| {
                                        e.prevent_default();
                                        e.stop_propagation();
                                        tabs.write().retain(|t| t.id != tab_id);
                                        active_tab.set(0);
                                        WebViewManager::close(WebViewId(tab_id as u64));
                                    })
                                    .child("X"),
                            ),
                        )
                        .into()
                }))
                .child(
                    // Add tab button
                    Button::new()
                        .on_press(move |_| {
                            let id = UseId::<WebViewPlugin>::get_in_hook();
                            tabs.write().push(Tab {
                                id,
                                title: format!("Tab {}", id + 1),
                                url: "https://duckduckgo.com".to_string(),
                            });
                            active_tab.set(id);
                        })
                        .background((60, 60, 60))
                        .color(Color::WHITE)
                        .corner_radius(4.)
                        .child("+"),
                ),
        )
        .child(
            // WebView content area
            rect()
                .expanded()
                .children(tabs.read().iter().filter_map(|tab| {
                    let is_active = *active_tab.read() == tab.id;
                    let url = tab.url.clone();

                    if is_active {
                        Some(
                            WebViewComponent::new(&url)
                                .width(Size::fill())
                                .height(Size::fill())
                                .id(WebViewId(tab.id as u64))
                                .into(),
                        )
                    } else {
                        None
                    }
                })),
        )
}
