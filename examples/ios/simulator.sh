#!/usr/bin/env sh
# Shared helpers for the simulator scripts.

BUNDLE_ID="com.freya.iosapp"

# Prints the UDID of the first available iPhone simulator.
first_iphone() {
    xcrun simctl list devices available \
        | grep -m1 -E '^ +iPhone' \
        | grep -oE '[0-9A-F]{8}(-[0-9A-F]{4}){3}-[0-9A-F]{12}'
}

# Boots the given simulator and waits until it is ready.
boot() {
    xcrun simctl boot "$1" 2>/dev/null || true
    xcrun simctl bootstatus "$1" -b >/dev/null
}
