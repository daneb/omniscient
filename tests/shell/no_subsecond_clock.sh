#!/usr/bin/env bash
# AC-5 — a bash shell with no sub-second clock still records commands.
#
# BSD date has no %N, so `date +%s%N` yields "1787479259N"; bash arithmetic then
# fails with "value too great for base", duration is empty, and the capture is
# rejected. macOS bash users therefore record nothing at all.
set -uo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$here/../.." && pwd)"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
captured="$tmp/captured"; : > "$captured"

bin="$root/target/debug/omniscient"
[[ -x $bin ]] || cargo build --quiet --manifest-path "$root/Cargo.toml" >/dev/null 2>&1

mkdir -p "$tmp/bin"
cat > "$tmp/bin/omniscient" <<STUB
#!/bin/sh
[ "\$1" = "capture" ] || exit 0
eval "cmd=\\\${\$#}"
printf '%s\n' "\$cmd" >> "$captured"
STUB
chmod +x "$tmp/bin/omniscient"
PATH="$tmp/bin:$PATH"

# A shell with no sub-second clock: BSD date, and EPOCHREALTIME unset (bash < 5).
unset EPOCHREALTIME

eval "$("$bin" init --shell bash 2>/dev/null)"
_omniscient_preexec "echo from-bash"
_omniscient_precmd

for _ in $(seq 1 200); do
    [[ -s $captured ]] && break
    sleep 0.01
done

if [[ ! -s $captured ]]; then
    echo "FAIL: bash recorded nothing without a sub-second clock" >&2
    exit 1
fi
got="$(cat "$captured")"
if [[ "$got" != "echo from-bash" ]]; then
    echo "FAIL: expected 'echo from-bash', got '$got'" >&2
    exit 1
fi
echo "PASS: bash records commands without a sub-second clock"
