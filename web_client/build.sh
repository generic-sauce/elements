#!/bin/bash

mode=""
if [ "$1" == "--release" ]; then
	mode="--release"
elif [ -n "$1" ]; then
	echo "wrong argument!"
	exit
fi

wasm-pack build $mode -d ./pkg
