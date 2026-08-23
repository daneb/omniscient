#!/usr/bin/env bash
# AC-3 — installing places both names.
#
# install.sh fetches and builds from git, so a full end-to-end run is not a
# reasonable test. What is testable, and is the actual contract, is the binary
# list it iterates: adding a name to Cargo.toml without adding it here means the
# short name works only for whoever built from source.
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"

list=$(grep -E '^OMNISCIENT_BINARIES=' "$root/install.sh" | head -1 | cut -d'"' -f2)
if [[ -z "$list" ]]; then
    echo "FAIL: install.sh declares no OMNISCIENT_BINARIES list" >&2
    exit 1
fi

for name in omniscient omni; do
    if [[ " $list " != *" $name "* ]]; then
        echo "FAIL: install.sh does not install '$name' (list: $list)" >&2
        exit 1
    fi
done

# Every declared name must also be a real build target, or install will fail on
# a `cp` of something that was never produced.
for name in $list; do
    if ! grep -qE "^name = \"$name\"" "$root/Cargo.toml"; then
        echo "FAIL: install.sh installs '$name' but Cargo.toml builds no such binary" >&2
        exit 1
    fi
done

echo "PASS: install.sh installs both names, and both are build targets"
