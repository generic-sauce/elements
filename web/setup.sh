#!/bin/bash

set -e

(cd ../web_client; wasm-pack build -d ../web/pkg)
(cd site; npm install)

ln -s "$(readlink -f ./pkg)" ./site/node_modules/elements
ln -s "$(readlink -f ../res)" ./site/
