#!/usr/bin/env sh

BUNDLE_ID="com.freya.iosapp"

# Prints the first available iPhone simulator's UDID.
first_iphone() {
    xcrun simctl list devices available \
        | grep -m1 -E '^ +iPhone' \
        | grep -oE '[0-9A-F]{8}(-[0-9A-F]{4}){3}-[0-9A-F]{12}'
}

boot() {
    xcrun simctl boot "$1" 2>/dev/null || true
    xcrun simctl bootstatus "$1" -b >/dev/null
}
