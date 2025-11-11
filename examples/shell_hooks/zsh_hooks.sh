#!/usr/bin/env zsh
# Omniscient - Command History Tracker
# Zsh Integration Hook Example
# 
# This file shows the hooks that omniscient uses to capture commands.
# To install, run: omniscient init >> ~/.zshrc

# Start timer before command execution
_omniscient_preexec() {
    export _OMNISCIENT_START=$EPOCHREALTIME
}

# Capture command after execution
_omniscient_precmd() {
    local exit_code=$?
    local cmd=$(fc -ln -1 | sed 's/^[[:space:]]*//')
    
    if [[ -n "$_OMNISCIENT_START" ]]; then
        local end=$EPOCHREALTIME
        local duration=$(( (end - _OMNISCIENT_START) * 1000 ))
        
        # Run capture in background to avoid blocking shell
        omniscient capture --exit-code "$exit_code" --duration "$duration" "$cmd" &
        
        unset _OMNISCIENT_START
    fi
}

# Register hooks with Zsh
precmd_functions+=(_omniscient_precmd)
preexec_functions+=(_omniscient_preexec)