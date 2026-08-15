use std::{
    cell::RefCell,
    rc::Rc,
};

use freya_core::prelude::*;
use futures_lite::StreamExt;
use reqwest::blocking::Client;

use crate::{
    element::Html,
    handle::{
        HtmlHandle,
        HtmlSource,
    },
    net::{
        fetch_html,
        http_client,
    },
    state::BlitzState,
};

async fn load_url(state: Rc<RefCell<BlitzState>>, url: String, client: Client) {
    let (fetched, url) = blocking::unblock(move || (fetch_html(&client, &url), url)).await;
    match fetched {
        Ok(html) => state.borrow_mut().load(&html, Some(url)),
        Err(err) => tracing::error!("Failed to load {url}: {err}"),
    }
}

/// Embeds an HTML + CSS document, rendered by [Blitz](https://github.com/DioxusLabs/blitz)
/// straight into Freya's Skia canvas. Its content is driven by an [HtmlHandle],
/// which also exposes history navigation and the current URL.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya_html::prelude::*;
/// # fn app() -> impl IntoElement {
/// // Inline HTML
/// let inline = use_html_handle(|| HtmlSource::html("<p>Hello <b>world</b></p>"));
/// HtmlView::new(inline)
///     .width(Size::px(400.))
///     .height(Size::px(300.));
/// // Or a remote document
/// let remote = use_html_handle(|| HtmlSource::url("https://example.com"));
/// HtmlView::new(remote)
///     .width(Size::px(400.))
///     .height(Size::px(300.))
/// # }
/// ```
#[derive(PartialEq)]
pub struct HtmlView {
    handle: HtmlHandle,
    layout: LayoutData,
}

impl HtmlView {
    /// Render the document navigated by `handle`, created with [use_html_handle](crate::use_html_handle).
    pub fn new(handle: HtmlHandle) -> Self {
        Self {
            handle,
            layout: LayoutData::default(),
        }
    }
}

impl LayoutExt for HtmlView {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for HtmlView {}

impl Component for HtmlView {
    fn render(&self) -> impl IntoElement {
        let platform = Platform::get();
        let client = use_hook(http_client);
        let mut handle = self.handle;

        let state = use_hook(move || {
            let platform = Platform::get();
            let (wake_tx, mut wake_rx) = futures_channel::mpsc::unbounded::<()>();
            let (nav_tx, mut nav_rx) = futures_channel::mpsc::unbounded::<String>();
            let state = Rc::new(RefCell::new(BlitzState::new(
                http_client(),
                wake_tx,
                nav_tx,
            )));

            spawn(async move {
                while wake_rx.next().await.is_some() {
                    platform.send(UserEvent::RequestRedraw);
                }
            });

            spawn(async move {
                while let Some(url) = nav_rx.next().await {
                    handle.navigate(url);
                }
            });

            state
        });

        use_side_effect_with_deps(&handle.location(), {
            let state = state.clone();
            move |location| {
                let Some((_, source)) = location else {
                    return;
                };
                match source {
                    HtmlSource::Url(url) => {
                        spawn(load_url(state.clone(), url.clone(), client.clone()));
                    }
                    HtmlSource::Html(html) => state.borrow_mut().load(html, None),
                }
            }
        });

        let a11y_id = use_hook(AccessibilityId::new_unique);

        Html::new(state.clone())
            .layout(self.layout.clone())
            .a11y_id(a11y_id)
            .a11y_focusable(true)
            .on_mouse_move({
                let state = state.clone();
                move |e: Event<MouseEventData>| {
                    state
                        .borrow_mut()
                        .mouse_move(e.element_location.x as f32, e.element_location.y as f32);
                }
            })
            .on_mouse_down({
                let state = state.clone();
                move |e: Event<MouseEventData>| {
                    a11y_id.request_focus();
                    state.borrow_mut().mouse_button(
                        e.element_location.x as f32,
                        e.element_location.y as f32,
                        e.button,
                        true,
                    );
                }
            })
            .on_mouse_up({
                let state = state.clone();
                move |e: Event<MouseEventData>| {
                    state.borrow_mut().mouse_button(
                        e.element_location.x as f32,
                        e.element_location.y as f32,
                        e.button,
                        false,
                    );
                }
            })
            .on_wheel({
                let state = state.clone();
                move |e: Event<WheelEventData>| {
                    let scale = *platform.scale_factor.read();
                    state.borrow_mut().wheel(
                        e.element_location.x as f32,
                        e.element_location.y as f32,
                        e.delta_x / scale,
                        e.delta_y / scale,
                    );
                }
            })
            .on_key_down({
                let state = state.clone();
                move |e: Event<KeyboardEventData>| {
                    state.borrow_mut().key(&e.key, &e.code, e.modifiers, true);
                }
            })
            .on_key_up(move |e: Event<KeyboardEventData>| {
                state.borrow_mut().key(&e.key, &e.code, e.modifiers, false);
            })
    }
}
