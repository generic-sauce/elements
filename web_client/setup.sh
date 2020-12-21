#!/bin/bash

set -e

wasm-pack build -d ./pkg
(cd site; npm install)

ln -s "$(readlink -f ./pkg)" ./site/node_modules/elements
ln -s "$(readlink -f ../res)" ./site/
