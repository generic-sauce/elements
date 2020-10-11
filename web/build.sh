#!/bin/bash

(cd ..;  wasm-pack build -d web/pkg) && (cd pkg; sudo npm link)
