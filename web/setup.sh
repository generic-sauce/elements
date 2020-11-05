#!/bin/bash

(cd ..; wasm-pack build -d web/pkg -- --no-default-features --features "web-client") && (cd pkg; sudo npm link) && (cd site; npm link elements && npm install) && ln -s "$(readlink -f ../res)" site/res
