#!/usr/bin/env bash
# AC-4 — uninstalling removes both names.
#
# Behavioural: a sandboxed HOME with both binaries present, run uninstall, assert
# both are gone. A stale `omni` after an uninstall is worse than never shipping
# it — it keeps working, silently, against data the user believes is deleted.
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"

tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/.cargo/bin" "$tmp/.omniscient"
for name in omniscient omni; do
    printf '#!/bin/sh\n' > "$tmp/.cargo/bin/$name"
    chmod +x "$tmp/.cargo/bin/$name"
done
# A shell rc for the hook-removal step to work on.
printf '# nothing\n' > "$tmp/.zshrc"

# "y" to proceed with the uninstall, "n" to keep the history data: this test is
# about binaries, and deleting a user's data to check a file was removed would
# be a poor trade.
printf 'y\nn\n' | HOME="$tmp" SHELL=/bin/zsh bash "$root/uninstall.sh" >/dev/null 2>&1 || true

for name in omniscient omni; do
    if [[ -e "$tmp/.cargo/bin/$name" ]]; then
        echo "FAIL: uninstall left '$name' behind at $tmp/.cargo/bin/$name" >&2
        exit 1
    fi
done
echo "PASS: uninstall removes both names"
