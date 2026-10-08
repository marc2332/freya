#!/usr/bin/env sh
# Builds the example, installs it on an iOS Simulator and launches it with its logs attached.
# Usage: ./run-simulator.sh [device name or UDID] (defaults to the first available iPhone)
set -eu

EXAMPLE_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$EXAMPLE_DIR/simulator.sh"

DEVICE="${1:-$(first_iphone)}"
APP="$("$EXAMPLE_DIR/bundle.sh" | tail -n1)"

boot "$DEVICE"
open -a Simulator
xcrun simctl install "$DEVICE" "$APP"
xcrun simctl launch --console-pty --terminate-running-process "$DEVICE" "$BUNDLE_ID"
