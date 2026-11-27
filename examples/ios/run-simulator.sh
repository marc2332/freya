#!/usr/bin/env sh
# Usage: ./run-simulator.sh [device name or UDID], defaults to the first available iPhone.
set -eu

EXAMPLE_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$EXAMPLE_DIR/simulator.sh"

DEVICE="${1:-$(first_iphone)}"
APP="$("$EXAMPLE_DIR/bundle.sh" | tail -n1)"

boot "$DEVICE"
open -a Simulator
xcrun simctl install "$DEVICE" "$APP"
xcrun simctl launch --console-pty --terminate-running-process "$DEVICE" "$BUNDLE_ID"
