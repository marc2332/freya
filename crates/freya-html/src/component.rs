use freya_core::prelude::*;

use crate::{
    element::Html,
    handle::HtmlHandle,
};

/// Embeds an HTML + CSS document rendered by [Blitz](https://github.com/DioxusLabs/blitz), driven by an [HtmlHandle].
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya_html::prelude::*;
/// # fn app() -> impl IntoElement {
/// // Inline HTML
/// let inline = use_hook(|| HtmlHandle::create(HtmlSource::html("<p>Hello <b>world</b></p>")));
/// HtmlViewer::new(inline);
/// // Or a remote document, sized explicitly
/// let remote = use_hook(|| HtmlHandle::create(HtmlSource::url("https://example.com")));
/// HtmlViewer::new(remote)
///     .width(Size::px(400.))
///     .height(Size::px(300.))
/// # }
/// ```
#[derive(PartialEq)]
pub struct HtmlViewer {
    handle: HtmlHandle,
    layout: LayoutData,
    key: DiffKey,
}

impl HtmlViewer {
    /// Render the document navigated by `handle`, fills its parent unless sized explicitly.
    pub fn new(handle: HtmlHandle) -> Self {
        Self {
            handle,
            layout: LayoutData::default(),
            key: DiffKey::None,
        }
        .expanded()
    }
}

impl LayoutExt for HtmlViewer {
    fn layout_mut(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for HtmlViewer {}

impl KeyExt for HtmlViewer {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for HtmlViewer {
    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }

    fn render(&self) -> impl IntoElement {
        let platform = Platform::get().current_window();
        let handle = self.handle;
        let state = handle.view();

        use_hook(move || {
            let platform = Platform::get().current_window();
            let wake = handle.wake.peek().clone();
            spawn(async move {
                while wake.recv().await.is_ok() {
                    platform.send(UserEvent::RequestRedraw);
                }
            });
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
            .on_touch_start({
                let state = state.clone();
                move |event: Event<TouchEventData>| {
                    a11y_id.request_focus();
                    state.borrow_mut().touch(&event, TouchPhase::Started);
                }
            })
            .on_touch_move({
                let state = state.clone();
                move |event: Event<TouchEventData>| {
                    state.borrow_mut().touch(&event, TouchPhase::Moved);
                }
            })
            .on_touch_end({
                let state = state.clone();
                move |event: Event<TouchEventData>| {
                    state.borrow_mut().touch(&event, TouchPhase::Ended);
                }
            })
            .on_touch_cancel({
                let state = state.clone();
                move |event: Event<TouchEventData>| {
                    state.borrow_mut().touch(&event, TouchPhase::Cancelled);
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
