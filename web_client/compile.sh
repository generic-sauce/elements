#!/bin/bash

tput reset

wasm-pack build -d web/pkg -- --color=always 2>&1 | less -r
