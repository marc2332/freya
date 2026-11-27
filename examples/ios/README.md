# Experimental iOS Support

> [!WARNING]
> iOS support is highly experimental. See [Known limitations](#known-limitations).

This example runs the same demo app as the [Android example](../android) (scroll view, widgets, portal, code editor and markdown) on iPhone, rendered with Metal.

## Prerequisites

### Xcode

iOS apps can only be built on macOS. Install [Xcode](https://developer.apple.com/xcode/) from the App Store, then select it as the active developer directory:

```sh
sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
```

Open Xcode once to accept the license and install an iOS Simulator runtime (**Settings > Components**). Check that you have at least one iPhone simulator available:

```sh
xcrun simctl list devices available | grep iPhone
```

### Rust tooling

Install the iOS targets:

```sh
rustup target add aarch64-apple-ios-sim aarch64-apple-ios
```

`aarch64-apple-ios-sim` is the iOS Simulator on Apple Silicon Macs, and `aarch64-apple-ios` is for physical devices.

## Running on the iOS Simulator

From the repository root:

```sh
./examples/ios/run-simulator.sh
```

The script uses the first available iPhone simulator. To pick a different one, pass its name or UDID:

```sh
./examples/ios/run-simulator.sh "iPhone Air"
```

The script performs these steps:

1. [`bundle.sh`](./bundle.sh) builds the `ios_example` binary for `aarch64-apple-ios-sim`, wraps it with [`Info.plist`](./Info.plist) in `target/aarch64-apple-ios-sim/debug/Freya.app` and ad-hoc signs it. The simulator accepts that signature without a developer account.
2. The script boots the simulator, then installs and launches the app with its logs streamed to your terminal.

Press `Ctrl+C` to stop the app.

> [!NOTE]
> `Info.plist` declares an empty `UILaunchScreen`. Without it iOS runs the app in a legacy compatibility mode with a letterboxed, smaller window.

## Soft keyboard

Register `IosPlugin` to get the soft keyboard with IME composition (for example Japanese or Chinese input):

```rust
let config = LaunchConfig::new().with_window(WindowConfig::new(app));

#[cfg(target_os = "ios")]
let config = config.with_plugin(freya::ios::IosPlugin::default());

launch(config)
```

Password inputs (`InputMode::Hidden`) use a secure keyboard without suggestions.

Without the plugin, winit still shows a basic keyboard, but it only sends plain characters and Backspace.

### Keyboard on the simulator

The simulator connects your Mac's keyboard by default, and while it is connected iOS hides the on-screen keyboard when an input is focused. To use the on-screen keyboard, turn off **I/O > Keyboard > Connect Hardware Keyboard** in the Simulator menu (`Cmd+Shift+K`). With the hardware keyboard connected, you can still show the on-screen keyboard with `Cmd+K`.

## Smoke test

[`smoke-test.sh`](./smoke-test.sh) launches the app on a simulator and fails if it does not start, exits early or panics:

```sh
./examples/ios/smoke-test.sh
```

It waits up to 120 seconds for the simulator to launch the app (override with `LAUNCH_TIMEOUT`), keeps it running for 20 seconds (override with `SMOKE_SECONDS`) and saves a screenshot to `target/ios-smoke-test.png` (override with `SCREENSHOT`).

CI runs it on every pull request in [`rust_ios.yml`](../../.github/workflows/rust_ios.yml), together with Clippy for the `aarch64-apple-ios` target and a device build. The screenshot is uploaded as the `ios-smoke-test` artifact.

## Running on a physical iPhone

Physical devices require the app to be signed with an Apple Developer certificate and a provisioning profile, which this example does not automate yet. Build the binary with:

```sh
cargo build -p ios --target aarch64-apple-ios
```

Then bundle it as above, sign it with your own identity and profile, and install it with `xcrun devicectl device install app`.

## Running on Desktop

This example also compiles as a regular desktop app:

```sh
cargo run -p ios
```

## Known limitations

- Autocorrect, autocapitalization and predictive text are disabled. The plugin's native text field only holds the text being composed, so iOS has no surrounding text to base them on.
- The app is not told the keyboard's height, so the keyboard can cover inputs near the bottom of the screen.
- The app draws behind the status bar, Dynamic Island and home indicator. Safe areas are not exposed yet, so the demo keeps clear of them with fixed padding.
- The first frames on the simulator can log `Compilation took longer than 1000 ms` from Skia while it compiles Metal shaders. It is a warning and the app keeps rendering normally.
