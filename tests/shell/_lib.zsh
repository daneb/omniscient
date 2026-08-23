# Shared harness for omniscient's shell-hook tests.
#
# These test the generated hook itself, not the Rust behind it. The bug they
# exist for lives entirely in the interaction between the hook and the user's
# history options, so a Rust test would have to mock the very thing that breaks.
#
# Each test sources the real hook from `omniscient init`, puts a stub
# `omniscient` earlier on PATH to record what capture was asked to store, then
# calls the hook functions directly — a non-interactive zsh never fires
# precmd/preexec on its own.

# Deliberately no `set -u`: an interactive shell does not have it, and the hook
# legitimately tests unset variables. Running the hook under stricter settings
# than it will ever meet tests the harness, not the hook.

_omni_repo_root() {
    # tests/shell/foo.zsh -> repo root
    print -r -- "${${(%):-%x}:A:h:h:h}"
}

omni_setup() {
    # The generated hook depends on two zsh modules it never loads itself:
    # zsh/datetime for $EPOCHREALTIME and zsh/mathfunc for int(). An interactive
    # shell usually has both via a framework; `zsh -f` has neither. Load them so
    # these tests exercise the hook as a real shell meets it.
    zmodload zsh/datetime 2>/dev/null
    zmodload zsh/mathfunc 2>/dev/null

    OMNI_ROOT=$(_omni_repo_root)
    OMNI_TMP=$(mktemp -d)
    OMNI_CAPTURED="$OMNI_TMP/captured"
    : > "$OMNI_CAPTURED"

    OMNI_BIN="$OMNI_ROOT/target/debug/omniscient"
    if [[ ! -x $OMNI_BIN ]]; then
        cargo build --quiet --manifest-path "$OMNI_ROOT/Cargo.toml" >/dev/null 2>&1 \
            || { print -u2 "FAIL: could not build omniscient"; exit 1; }
    fi

    # A stub that records the command text the hook passed to `capture`. The
    # command is the last argument; everything before it is flags.
    mkdir -p "$OMNI_TMP/bin"
    cat > "$OMNI_TMP/bin/omniscient" <<STUB
#!/bin/sh
[ "\$1" = "capture" ] || exit 0
eval "cmd=\\\${\$#}"
printf '%s\n' "\$cmd" >> "$OMNI_CAPTURED"
STUB
    chmod +x "$OMNI_TMP/bin/omniscient"
    PATH="$OMNI_TMP/bin:$PATH"

    # History confined to the sandbox so a test never touches the real one.
    HISTFILE="$OMNI_TMP/history"
    HISTSIZE=200
    SAVEHIST=200
    : > "$HISTFILE"

    local hook
    hook=$("$OMNI_BIN" init --shell zsh 2>/dev/null) \
        || { print -u2 "FAIL: omniscient init did not emit a hook"; exit 1; }
    eval "$hook"
}

omni_teardown() { [[ -n ${OMNI_TMP:-} ]] && rm -rf "$OMNI_TMP"; }

# Run one command through the hook exactly as an interactive shell would.
omni_run() {
    _omniscient_preexec "$1"
    _omniscient_precmd
}

# Captures are backgrounded and disowned, so `wait` cannot see them.
omni_await() {
    local want=$1 waited=0
    while (( waited < 200 )); do
        (( $(wc -l < "$OMNI_CAPTURED") >= want )) && return 0
        sleep 0.01
        (( waited++ ))
    done
    return 1
}

omni_captured() { cat "$OMNI_CAPTURED" }
omni_count() { wc -l < "$OMNI_CAPTURED" | tr -d ' ' }

omni_pass() { print "PASS: $1"; omni_teardown; exit 0 }
omni_fail() {
    print -u2 "FAIL: $1"
    print -u2 "  captured was:"
    omni_captured | sed 's/^/    /' >&2
    omni_teardown
    exit 1
}
