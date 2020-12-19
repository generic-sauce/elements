#!/bin/bash

tput reset

RUST_BACKTRACE=1 wasm-pack build -d web/pkg -- --color=always 2>&1 | less -r
