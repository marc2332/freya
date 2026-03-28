# Experimental Android Support

> [!WARNING]
> Android support is highly experimental. Soft keyboard and IME support are not yet implemented, so Input components and anything relying on keyboard input will not work properly on Android.

## Prerequisites

### Android SDK

Install [Android Studio](https://developer.android.com/studio) (recommended) or the [Android command-line tools](https://developer.android.com/studio#command-line-tools-only). API level 36 is required.

Set the following environment variables:

```sh
export ANDROID_HOME="<PATH_TO_SDK>"
```

If you installed Android Studio, the SDK is typically at `~/Android/Sdk` (Linux), `~/Library/Android/sdk` (macOS), or `%LOCALAPPDATA%\Android\Sdk` (Windows).

### Android NDK (r26d)

Download and extract the NDK for your platform:

- [Linux](https://dl.google.com/android/repository/android-ndk-r26d-linux.zip)
- [macOS](https://dl.google.com/android/repository/android-ndk-r26d-darwin.dmg)
- [Windows](https://dl.google.com/android/repository/android-ndk-r26d-windows.zip)

Then set the following environment variables:

```sh
export ANDROID_NDK_HOME="<PATH_TO_NDK>/android-ndk-r26d"
export ANDROID_NDK="<PATH_TO_NDK>/android-ndk-r26d"
```

### Rust tooling

Install `cargo-ndk` and the required Android targets:

```sh
cargo install cargo-ndk
rustup target add aarch64-linux-android x86_64-linux-android
```

## Building for Android

### With Android Studio

Open the `./AndroidApp/` project in Android Studio and click **Run**. If you have a device connected with ADB debugging enabled, the app will launch after a successful build.

### Without Android Studio

From the `./AndroidApp/` directory:

```sh
./gradlew assembleDebug
```

The APK will be at `./AndroidApp/app/build/outputs/apk/debug/app-debug.apk`. Install it via `adb install` or by copying it to your device.

## Running on Desktop

This example also compiles as a regular desktop app:

```sh
cargo run
```

## Attribution

This experiment is inspired by an older attempt to add Android support to Freya by [rebecca-src](https://github.com/rebecca-src). You can see their original branch here: https://github.com/rebecca-src/freya/tree/experiment/android/examples/android
