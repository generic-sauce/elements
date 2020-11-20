#!/bin/bash

(cd ../web/; ./build.sh)
(cd ../web/site; npx webpack)
if [ -d static ]; then
	rm -r static
fi
mkdir static
cp -r ../res static
cp ../web/site/index.html static
cp -r ../web/site/dist/* static
