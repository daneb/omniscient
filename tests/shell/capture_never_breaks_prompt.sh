#!/usr/bin/env sh
# AC-5 — a capture that cannot be persisted still exits 0.
#
# The hook runs capture on every prompt. If a storage problem made it exit
# non-zero, $? would be clobbered for anything downstream and a broken database
# would become a broken shell.
set -u
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
bin="$root/target/debug/omniscient"
[ -x "$bin" ] || cargo build --quiet --manifest-path "$root/Cargo.toml" >/dev/null 2>&1

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# A home whose .omniscient path is occupied by a regular file, so the database
# directory cannot be created and storage cannot open.
mkdir -p "$tmp/home"
printf 'not a directory\n' > "$tmp/home/.omniscient"

HOME="$tmp/home" "$bin" capture --exit-code 0 --duration 1 "echo hello" >/dev/null 2>&1
status=$?

if [ "$status" -ne 0 ]; then
    echo "FAIL: capture exited $status when storage was unavailable" >&2
    exit 1
fi
echo "PASS: capture exits 0 even when storage is unavailable"
