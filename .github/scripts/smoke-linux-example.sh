#!/usr/bin/env bash
set -euo pipefail

mkdir -p artifacts
rm -f artifacts/linux-smoke.png
unset WAYLAND_DISPLAY
"./target/release/examples/$EXAMPLE_NAME" &
example_pid=$!
trap 'kill "$example_pid" 2>/dev/null || true; wait "$example_pid" 2>/dev/null || true' EXIT

window_id=$(timeout 20s xdotool search --sync --onlyvisible --all --limit 1 --pid "$example_pid" --name .)
sleep 5
kill -0 "$example_pid"
import -window "$window_id" artifacts/linux-smoke.png
kill -0 "$example_pid"
test -s artifacts/linux-smoke.png
identify artifacts/linux-smoke.png
