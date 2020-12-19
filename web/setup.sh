#!/bin/bash

(cd ../web_client; wasm-pack build -d ../web/pkg) && (cd pkg; sudo npm link) && (cd site; npm link elements && npm install) && ln -s "$(readlink -f ../res)" site/res
