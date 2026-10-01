use freya_core::{
    elements::rect::Rect,
    prelude::{
        Event,
        EventHandlersExt,
        EventsCombos,
        GlobalUserEvent,
        MouseButton,
        Platform,
        PlatformWindow,
        PointerEventData,
        UserEvent,
    },
    user_event::SingleThreadErasedEvent,
};
use freya_engine::prelude::Surface as SkiaSurface;
use winit::window::{
    Window,
    WindowId,
};

use crate::{
    config::WindowConfig,
    renderer::{
        NativePlatformErasedEventAction,
        NativeWindowErasedEventAction,
        RendererContext,
    },
};

/// Extension trait that adds winit-specific window management capabilities to [`Platform`].
pub trait WinitPlatformExt {
    /// Get the [`WindowId`] of the window from whose context this is called.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::prelude::*;
    ///
    /// fn close_current_window() {
    ///     Platform::get().close_window(Platform::window_id());
    /// }
    /// ```
    fn window_id() -> WindowId;

    /// Dynamically launch a new window at runtime with the given configuration.
    ///
    /// This is meant to create windows on the fly after the application has started,
    /// as opposed to the initial windows registered via [`crate::config::LaunchConfig`].
    ///
    /// Returns the [`WindowId`] of the newly created window once it has been created.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::prelude::*;
    ///
    /// async fn open_new_window() {
    ///     let window_id = Platform::get()
    ///         .launch_window(WindowConfig::new(my_app).with_title("New Window"))
    ///         .await;
    /// }
    /// # fn my_app() -> impl IntoElement { rect() }
    /// ```
    fn launch_window(&self, window_config: WindowConfig) -> impl Future<Output = WindowId>;

    /// Close an existing window by its [`WindowId`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::{
    ///     prelude::*,
    ///     winit::window::WindowId,
    /// };
    ///
    /// fn close_window(window_id: WindowId) {
    ///     Platform::get().close_window(window_id);
    /// }
    /// ```
    fn close_window(&self, window_id: WindowId);

    /// Focus a window by its [`WindowId`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::prelude::*;
    ///
    /// fn focus_current_window() {
    ///     Platform::get().focus_window(Platform::window_id());
    /// }
    /// ```
    fn focus_window(&self, window_id: WindowId);

    /// Set the title of a window, also updating its accessibility label.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::prelude::*;
    ///
    /// fn rename_current_window() {
    ///     Platform::get().set_window_title(Platform::window_id(), "New Title");
    /// }
    /// ```
    fn set_window_title(&self, window_id: WindowId, title: impl Into<String>);

    /// Execute a callback with mutable access to a [`Window`].
    ///
    /// This allows direct manipulation of the underlying winit [`Window`] for advanced use cases.
    ///
    /// To create new windows dynamically, see [`WinitPlatformExt::launch_window()`].
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::prelude::*;
    ///
    /// fn minimize_current_window() {
    ///     Platform::get().with_window(Platform::window_id(), |window| {
    ///         window.set_minimized(true);
    ///     });
    /// }
    /// ```
    fn with_window(&self, window_id: WindowId, callback: impl FnOnce(&mut Window) + 'static);

    /// Queue a renderer callback without an originating window.
    /// The returned receiver can be awaited for its result or dropped.
    fn post_callback<F, T: 'static>(&self, callback: F) -> futures_channel::oneshot::Receiver<T>
    where
        F: FnOnce(&mut RendererContext) -> T + 'static;
}

/// Winit APIs for the selected window, not necessarily the focused window.
pub trait WinitPlatformWindowExt {
    /// Queue a callback on this window's next render pass, after rendering and before presenting.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::prelude::*;
    ///
    /// async fn take_screenshot() {
    ///     let image = Platform::get()
    ///         .current_window()
    ///         .post_render_callback(|surface| surface.image_snapshot())
    ///         .await;
    /// }
    /// ```
    fn post_render_callback<F, T: 'static>(
        &self,
        callback: F,
    ) -> futures_channel::oneshot::Receiver<T>
    where
        F: FnOnce(&mut SkiaSurface) -> T + 'static;
}

#[derive(Clone, Copy, PartialEq)]
struct WindowDragGesture;

pub trait WindowDragExt {
    /// Drag on left press and move, or toggle maximize on a double press.
    fn window_drag(self) -> Self;
}

impl WindowDragExt for Rect {
    fn window_drag(self) -> Self {
        self.on_pointer_down(|event: Event<PointerEventData>| {
            if event.button() != Some(MouseButton::Left) {
                return;
            }
            if EventsCombos::<WindowDragGesture>::pressed(event.global_location()).is_double() {
                Platform::get().with_window(Platform::window_id(), |window| {
                    window.set_maximized(!window.is_maximized())
                });
            }
        })
        .on_global_pointer_move(|event: Event<PointerEventData>| {
            if EventsCombos::<WindowDragGesture>::moved(event.global_location()) {
                Platform::get().with_window(Platform::window_id(), |window| {
                    let _ = window.drag_window();
                });
            }
        })
        .on_global_pointer_up(|_: Event<PointerEventData>| {
            EventsCombos::<WindowDragGesture>::released();
        })
    }
}

impl WinitPlatformExt for Platform {
    fn window_id() -> WindowId {
        Platform::get().current_window().id.into()
    }

    async fn launch_window(&self, window_config: WindowConfig) -> WindowId {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.send(GlobalUserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativePlatformErasedEventAction::LaunchWindow {
                window_config: Box::new(window_config),
                ack: sender,
            },
        ))));
        receiver.await.expect("Failed to create window")
    }

    fn close_window(&self, window_id: WindowId) {
        self.send(GlobalUserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativePlatformErasedEventAction::CloseWindow(window_id),
        ))));
    }

    fn focus_window(&self, window_id: WindowId) {
        self.with_window(window_id, |window| window.focus_window());
    }

    fn set_window_title(&self, window_id: WindowId, title: impl Into<String>) {
        let title = title.into();
        let _ = self.post_callback(move |context| {
            if let Some(app) = context.windows.get_mut(&window_id) {
                app.set_title(&title);
            }
        });
    }

    fn with_window(&self, window_id: WindowId, callback: impl FnOnce(&mut Window) + 'static) {
        let _ = self.post_callback(move |context| {
            if let Some(app) = context.windows.get_mut(&window_id) {
                callback(&mut app.window);
            }
        });
    }

    fn post_callback<F, T: 'static>(&self, callback: F) -> futures_channel::oneshot::Receiver<T>
    where
        F: FnOnce(&mut RendererContext) -> T + 'static,
    {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.send(GlobalUserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativePlatformErasedEventAction::RendererCallback(Box::new(move |context| {
                let _ = sender.send(callback(context));
            })),
        ))));
        receiver
    }
}

impl WinitPlatformWindowExt for PlatformWindow {
    fn post_render_callback<F, T: 'static>(
        &self,
        callback: F,
    ) -> futures_channel::oneshot::Receiver<T>
    where
        F: FnOnce(&mut SkiaSurface) -> T + 'static,
    {
        let (sender, receiver) = futures_channel::oneshot::channel();
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativeWindowErasedEventAction::RendererCallback(Box::new(move |window_id, context| {
                if let Some(app) = context.windows.get_mut(&window_id) {
                    app.render_callbacks.push(Box::new(move |surface| {
                        let _ = sender.send(callback(surface));
                    }));
                    app.window.request_redraw();
                }
            })),
        ))));
        receiver
    }
}
