#!/bin/bash

mode=""
if [ "$1" == "--release" ]; then
	mode="--release"
elif [ -n "$1" ]; then
	echo "wrong argument!"
	exit
fi

(cd ..; wasm-pack build $mode -d web/pkg -- --no-default-features --features "web-client")
