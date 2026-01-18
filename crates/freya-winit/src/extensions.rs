use freya_core::{
    elements::rect::Rect,
    prelude::{
        Event,
        EventHandlersExt,
        EventsCombos,
        Platform,
        PointerEventData,
        PressEventType,
        UserEvent,
    },
    user_event::SingleThreadErasedEvent,
};
use winit::window::{
    Window,
    WindowId,
};

use crate::{
    config::WindowConfig,
    renderer::{
        NativeWindowErasedEventAction,
        WithWindowCallback,
    },
};

pub trait WinitPlatformExt {
    /// Launch a new window with the given configuration.
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
    /// If `window_id` is `None`, the current window will be focused.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::{
    ///     prelude::*,
    ///     winit::window::WindowId,
    /// };
    ///
    /// fn focus_specific_window(window_id: WindowId) {
    ///     Platform::get().focus_window(Some(window_id));
    /// }
    ///
    /// fn focus_current_window() {
    ///     Platform::get().focus_window(None);
    /// }
    /// ```
    fn focus_window(&self, window_id: Option<WindowId>);

    /// Execute a callback with mutable access to a [`Window`].
    ///
    /// If `window_id` is `None`, the callback will be executed on the current window.
    /// This allows direct manipulation of the underlying winit [`Window`] for advanced use cases.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use freya::{
    ///     prelude::*,
    ///     winit::window::WindowId,
    /// };
    ///
    /// fn set_window_title(window_id: Option<WindowId>, title: &'static str) {
    ///     Platform::get().with_window(window_id, move |window| {
    ///         window.set_title(title);
    ///     });
    /// }
    ///
    /// fn minimize_current_window() {
    ///     Platform::get().with_window(None, |window| {
    ///         window.set_minimized(true);
    ///     });
    /// }
    /// ```
    fn with_window(
        &self,
        window_id: Option<WindowId>,
        callback: impl FnOnce(&mut Window) + 'static,
    );
}

pub trait WindowDragExt {
    fn window_drag(self) -> Self;
}

impl WindowDragExt for Rect {
    fn window_drag(self) -> Self {
        self.on_pointer_down(move |e: Event<PointerEventData>| {
            match EventsCombos::pressed(e.global_location()) {
                PressEventType::Single => {
                    Platform::get().with_window(None, |window| {
                        let _ = window.drag_window();
                    });
                }
                PressEventType::Double => {
                    Platform::get().with_window(None, |window| {
                        if window.is_maximized() {
                            window.set_maximized(false);
                        } else {
                            window.set_maximized(true);
                        }
                    });
                }
                _ => {}
            }
        })
    }
}

impl WinitPlatformExt for Platform {
    async fn launch_window(&self, window_config: WindowConfig) -> WindowId {
        let (tx, rx) = futures_channel::oneshot::channel();
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativeWindowErasedEventAction::LaunchWindow {
                window_config,
                ack: tx,
            },
        ))));
        rx.await.expect("Failed to create Window")
    }

    fn close_window(&self, window_id: WindowId) {
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativeWindowErasedEventAction::CloseWindow(window_id),
        ))));
    }

    fn focus_window(&self, window_id: Option<WindowId>) {
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativeWindowErasedEventAction::WithWindow {
                window_id,
                callback: WithWindowCallback(Box::new(|window| window.focus_window())),
            },
        ))));
    }

    fn with_window(
        &self,
        window_id: Option<WindowId>,
        callback: impl FnOnce(&mut Window) + 'static,
    ) {
        self.send(UserEvent::Erased(SingleThreadErasedEvent(Box::new(
            NativeWindowErasedEventAction::WithWindow {
                window_id,
                callback: WithWindowCallback(Box::new(callback)),
            },
        ))));
    }
}
