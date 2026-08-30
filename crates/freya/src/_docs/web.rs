//! # Web
//!
//! Freya runs in the browser through [freya_web], enabled with the `web` feature.
//!
//! ## Requirements
//!
//! ```sh
//! rustup target add wasm32-unknown-emscripten
//! ```
//!
//! [Emscripten](https://emscripten.org/docs/getting_started/downloads.html) 4.0 or newer, with
//! `emcc` in `PATH`.
//!
//! ## Setup
//!
//! Add Freya without the default `winit` feature:
//!
//! ```toml
//! freya = { version = "...", default-features = false, features = ["web"] }
//! ```
//!
//! Then set the target and the linker flags in `.cargo/config.toml`:
//!
//! ```toml
//! [build]
//! target = "wasm32-unknown-emscripten"
//!
//! [env]
//! EMCC_CFLAGS = "-fwasm-exceptions"
//!
//! [target.wasm32-unknown-emscripten]
//! rustflags = [
//!   "-Clink-arg=-sMAX_WEBGL_VERSION=2",
//!   "-Clink-arg=-sALLOW_MEMORY_GROWTH=1",
//!   "-Clink-arg=-sSTACK_SIZE=16MB",
//!   "-Clink-arg=-sINITIAL_MEMORY=256MB",
//!   "-Clink-arg=-sEXPORTED_FUNCTIONS=['_main','_free']",
//! ]
//! ```
//!
//! ## Launching
//!
//! Fonts must be embedded, the first one registered becomes the default.
//!
//! ```rust, ignore
//! use freya::{prelude::*, web::*};
//!
//! const INTER: &[u8] = include_bytes!("./Inter.ttf");
//!
//! fn main() {
//!     launch(WebConfig::new(app).with_font("Inter", INTER));
//! }
//!
//! fn app() -> impl IntoElement {
//!     rect().expanded().center().child("Hello, Web!")
//! }
//! ```
//!
//! ## Web App
//!
//! Building emits a `.js` loader next to the `.wasm`, serve both. The app renders in the canvas
//! element with the `canvas` id.
//!
//! ```html
//! <canvas id="canvas" style="image-rendering: pixelated;"></canvas>
//! <script>
//!     var Module = { canvas: document.getElementById("canvas") };
//! </script>
//! <script src="./my_app.js"></script>
//! ```
//!
//! Use `image-rendering: pixelated` on the canvas to keep it sharp.
//!
//! See the [`web`](https://github.com/marc2332/freya/tree/main/examples/web) example, which is what
//! runs at [freyaui.dev/demo](https://freyaui.dev/demo).
