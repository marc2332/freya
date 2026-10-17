#!/usr/bin/env sh
# Usage: ./smoke-test.sh [device name or UDID], defaults to the first available iPhone.
# Environment: SMOKE_SECONDS (default 20), LAUNCH_TIMEOUT (default 120), SCREENSHOT (default target/ios-smoke-test.png).
set -eu

EXAMPLE_DIR="$(cd "$(dirname "$0")" && pwd)"
. "$EXAMPLE_DIR/simulator.sh"

DEVICE="${1:-$(first_iphone)}"
SECONDS_ALIVE="${SMOKE_SECONDS:-20}"
LAUNCH_TIMEOUT="${LAUNCH_TIMEOUT:-120}"
SCREENSHOT="${SCREENSHOT:-$EXAMPLE_DIR/../../target/ios-smoke-test.png}"
LOG="$(mktemp)"
APP="$("$EXAMPLE_DIR/bundle.sh" | tail -n1)"

boot "$DEVICE"
xcrun simctl install "$DEVICE" "$APP"
xcrun simctl launch --console --terminate-running-process "$DEVICE" "$BUNDLE_ID" >"$LOG" 2>&1 &
LAUNCHER=$!

PID=""
for _ in $(seq 1 "$LAUNCH_TIMEOUT"); do
    PID="$(pgrep -n -f "Freya.app/ios_exampl[e]" || true)"
    [ -n "$PID" ] && break
    sleep 1
done

if [ -z "$PID" ]; then
    cat "$LOG"
    if grep -q "^$BUNDLE_ID: " "$LOG"; then
        echo "Smoke test failed: the app launched but exited right away" >&2
    else
        echo "Smoke test failed: the simulator did not launch the app within ${LAUNCH_TIMEOUT}s" >&2
    fi
    exit 1
fi

sleep "$SECONDS_ALIVE"
xcrun simctl io "$DEVICE" screenshot "$SCREENSHOT" >/dev/null 2>&1 || true

if ! kill -0 "$PID" 2>/dev/null; then
    cat "$LOG"
    echo "Smoke test failed: the app exited before ${SECONDS_ALIVE}s" >&2
    exit 1
fi

if grep -q "panicked" "$LOG"; then
    cat "$LOG"
    echo "Smoke test failed: the app panicked" >&2
    exit 1
fi

xcrun simctl terminate "$DEVICE" "$BUNDLE_ID" >/dev/null 2>&1 || true
kill "$LAUNCHER" 2>/dev/null || true
echo "Smoke test passed: the app ran for ${SECONDS_ALIVE}s (screenshot at $SCREENSHOT)"
