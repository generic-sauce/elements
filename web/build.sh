#!/bin/bash

(cd ..; wasm-pack build -d web/pkg -- --no-default-features --features "web-client") && (cd pkg; sudo npm link)
