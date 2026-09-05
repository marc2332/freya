# Experimental Web Support

> [!WARNING]
> Web support is experimental. Apps run in a `canvas` element through [`freya-web`](../../crates/freya-web), compiled to WebAssembly with Emscripten.

This example runs a multi-route demo app in the browser. It is what gets served at [freyaui.dev/demo](https://freyaui.dev/demo).

## Prerequisites

### Rust target

```sh
rustup target add wasm32-unknown-emscripten
```

### Emscripten

Install [Emscripten](https://emscripten.org/docs/getting_started/downloads.html) 4.0 or newer and make sure `emcc` is in `PATH`. It is the linker Rust uses for this target, older versions reject the wasm features Rust emits and only release builds break.

## Project setup

Add Freya without the default `winit` feature:

```toml
freya = { version = "...", default-features = false, features = ["web"] }
```

Set the target and the linker flags in `.cargo/config.toml`, this example's [`.cargo/config.toml`](./.cargo/config.toml) has them:

```toml
[build]
target = "wasm32-unknown-emscripten"

[env]
EMCC_CFLAGS = "-fwasm-exceptions"

[target.wasm32-unknown-emscripten]
rustflags = [
  "-Clink-arg=-sMAX_WEBGL_VERSION=2",
  "-Clink-arg=-sALLOW_MEMORY_GROWTH=1",
  "-Clink-arg=-sSTACK_SIZE=16MB",
  "-Clink-arg=-sINITIAL_MEMORY=256MB",
]
```

Fonts must be embedded, the first one registered becomes the default:

```rust
use freya::{prelude::*, web::*};

const INTER: &[u8] = include_bytes!("./Inter.ttf");

fn main() {
    launch(WebConfig::new(app).with_font("Inter", INTER));
}

fn app() -> impl IntoElement {
    rect().expanded().center().child("Hello, Web!")
}
```

Building emits a `.js` loader next to the `.wasm`, serve both. The app renders in the canvas element with the `canvas` id, which needs `tabindex` to receive keyboard input:

```html
<canvas id="canvas" tabindex="0" style="image-rendering: pixelated;"></canvas>
<script>
  var Module = { canvas: document.getElementById("canvas") };
</script>
<script src="./my_app.js"></script>
```

Use `image-rendering: pixelated` on the canvas to keep it sharp. See [`serve.py`](./serve.py) for the full page this example uses.

## Running

The target, the linker flags and a runner that serves the result come from `.cargo/config.toml`:

```sh
cargo run
```

It serves http://localhost:8771, set `FREYA_WEB_PORT` to use another port. Release builds are much faster and are what the website ships:

```sh
cargo run --release
```

## Building the website demo

Run `just web-demo` at the root of the repository to build the copy the website serves from `website/public/demo`.
