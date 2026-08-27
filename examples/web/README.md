# Web example

Runs a multi-route demo app in the browser through [`freya-web`](../../crates/freya-web).
This is what gets served at [freyaui.dev/demo](https://freyaui.dev/demo).

## Requirements

- The `wasm32-unknown-emscripten` Rust target: `rustup target add wasm32-unknown-emscripten`
- [Emscripten](https://emscripten.org/docs/getting_started/downloads.html) 4.0 or newer, with `emcc` in `PATH`. `emcc` is the linker, older versions
  reject the wasm features Rust emits and only release builds break.

## Run

The target, the linker flags and a runner that serves the result come from
`.cargo/config.toml`:

```shell
cargo run
```

It serves http://localhost:8771, set `FREYA_WEB_PORT` to use another port. Release builds
are much faster and are what the website ships:

```shell
cargo run --release
```

Run `just web-demo` at the root of the repository to build the copy the website serves
from `website/public/demo`.
