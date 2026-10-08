#!/usr/bin/env sh
# Bundles the simulator example with an ad-hoc signature and prints its path to stdout.
set -eu

TARGET="aarch64-apple-ios-sim"
EXAMPLE_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET_DIR="$(cd "$EXAMPLE_DIR/../.." && pwd)/target"
APP="$TARGET_DIR/$TARGET/debug/Freya.app"

cargo build --manifest-path "$EXAMPLE_DIR/Cargo.toml" --target "$TARGET" >&2

rm -rf "$APP"
mkdir -p "$APP"
cp "$TARGET_DIR/$TARGET/debug/ios_example" "$APP/"
cp "$EXAMPLE_DIR/Info.plist" "$APP/"
codesign --force --sign - "$APP" >&2

echo "$APP"
