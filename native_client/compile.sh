#!/bin/bash

tput reset

# RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo check --color=always 2>&1 | less -r
if [ "$1" = "c" ]; then
	cargo clippy --color=always 2>&1 | less -r
else
	cargo check --color=always 2>&1 | less -r
	# cargo check --color=always 2>&1 --package master_server | less -r
	# cargo check --color=always 2>&1 --no-default-features --features=game-server | less -r
	# (cd web; ./build.sh 2>&1 | less -r)
fi
# cargo check --no-default-features --features "server" --color=always 2>&1 | less -r
