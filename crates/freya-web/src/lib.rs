//! Web renderer for Freya, rendering with Skia on WebGL inside a `<canvas id="canvas">`.
#![cfg(target_os = "emscripten")]

use std::cell::RefCell;

use freya_core::integration::AppComponent;

mod app;
mod config;
mod emscripten;
mod events;
mod surface;

pub use crate::config::WebConfig;
use crate::{
    app::WebApp,
    emscripten::emscripten_set_main_loop,
};

thread_local! {
    static APP: RefCell<Option<WebApp>> = const { RefCell::new(None) };
}

/// Runs a Freya app in the browser.
pub fn launch(app: impl Into<AppComponent>) {
    launch_cfg(WebConfig::new(app))
}

/// Runs a Freya app in the browser with a custom configuration.
pub fn launch_cfg(config: WebConfig) {
    let Some(app) = WebApp::new(config) else {
        return;
    };

    APP.with_borrow_mut(|slot| *slot = Some(app));

    unsafe { emscripten_set_main_loop(frame, 0, 0) };
}

extern "C" fn frame() {
    APP.with_borrow_mut(|app| {
        if let Some(app) = app.as_mut() {
            app.frame();
        }
    });
}
