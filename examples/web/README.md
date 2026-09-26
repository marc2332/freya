# Experimental Web Support

> [!WARNING]
> Web support is experimental.

## Setup

The repository's Nix development shell provides the `wasm32-unknown-emscripten` target and [Emscripten](https://emscripten.org/docs/getting_started/downloads.html), including `emcc`.

Without Nix, install the Rust target and Emscripten 4.0 or newer with `emcc` in `PATH`:

```sh
rustup target add wasm32-unknown-emscripten
```

## Web

```sh
cargo web
cargo web --release
```

Served at http://localhost:8771, `FREYA_WEB_PORT` changes the port.

## Desktop

```sh
cargo desktop
```

## Website demo

```sh
just web-demo
```
