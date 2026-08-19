# Web example

Runs a multi-route demo app in the browser through [`freya-web`](../../crates/freya-web).

## Requirements

- The `wasm32-unknown-emscripten` Rust target: `rustup target add wasm32-unknown-emscripten`
- Emscripten 4.0 or newer, activated with `source <emsdk>/emsdk_env.sh` so that `emcc` is in `PATH`.
  Older versions ship a Binaryen that rejects the wasm features Rust emits, which only breaks release builds.
- Until Skia ships prebuilt WebAssembly binaries, a local checkout of
  [rust-skia](https://github.com/marc2332/rust-skia) with its submodules, pointed at by
  `SKIA_SOURCE_DIR`. The first build compiles Skia from source and takes a few minutes.

```shell
source <emsdk>/emsdk_env.sh
export SKIA_SOURCE_DIR=<rust-skia>/skia-bindings/skia
```

## Build

The target and the linker flags come from `.cargo/config.toml`, so building is just:

```shell
cargo build
```

## Run

The artifacts land in the target directory, link them next to `index.html` and serve it:

```shell
cd web
ln -sf <target-dir>/wasm32-unknown-emscripten/debug/web_example.js .
ln -sf <target-dir>/wasm32-unknown-emscripten/debug/web_example.wasm .
python3 -m http.server 8080 --directory .
```
