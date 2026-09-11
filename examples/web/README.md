# Experimental Web Support

> [!WARNING]
> Web support is experimental.

## Setup

```sh
rustup target add wasm32-unknown-emscripten
```

[Emscripten](https://emscripten.org/docs/getting_started/downloads.html) 4.0 or newer, with `emcc` in `PATH`.

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
