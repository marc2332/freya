# Experimental Android Support

> [!WARNING]
> Android support is still highly experimental, and it's currently missing soft keyboard and IME support. This means that you won't be able to use Input components, or anything that might rely on keyboard input on Android.

___

## Dependencies
You will need to download the following dependencies before you can build for Android.

### Install cargo ndk:
```
cargo install cargo-ndk
```

### Install Android Studio (optional):
[Android Studio](https://developer.android.com/studio)

### Download Android NDK (recommended: r26d)
- Linux: https://dl.google.com/android/repository/android-ndk-r26d-linux.zip
- MacOS: https://dl.google.com/android/repository/android-ndk-r26d-darwin.dmg
- Windows: https://dl.google.com/android/repository/android-ndk-r26d-darwin.dmg

### Extract NDK and add to Environment variables:
```
export ANDROID_NDK_HOME="<PATH_TO_UNPACKED_NDK>/android-ndk-r26d"
export ANDROID_NDK="<PATH_TO_UNPACKED_NDK>/android-ndk-r26d"
```

### Install required ABIs
```
rustup target add aarch64-linux-android x86_64-linux-android
```

___

## Building for Android
#### With Android Studio 
All you have to do is open the included Android project (`./AndroidApp/`) in Android Studio, and click 'Run'. If you have a device connected (with ADB Debugging enabled), the app should launch after a successful build.

#### Without Android Studio
Run the following command in the Android project (`./AndroidApp/`):
```
./gradlew assembleDebug
```

This will build the .apk file, and place it in `./AndroidApp/app/build/outputs/apk/debug/app-debug.apk`. 
You can then install this either via `adb`, or by copying it to your phone directly.

___

You don't have to build the Rust library separately, as the Android project's Gradle config already includes the command for building the native libraries. However, if you still want to build them manually, you can do so with the following command (in this directory):
```
cargo ndk -o AndroidApp/app/src/main/jniLibs/ -t arm64-v8a build -t x86_64-linux-android --lib --release
```

___

## Running on Desktop
You can compile and run this example directly for Desktop targets without any modifications or feature toggles. Simply run the following command from this directory:
```
cargo run
```

___

## Attribution
This experiment is inspired by an older attempt to add Android support to Freya by [rebecca-src](https://github.com/rebecca-src). You can see their original branch here: https://github.com/rebecca-src/freya/tree/experiment/android/examples/android