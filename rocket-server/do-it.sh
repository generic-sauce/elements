#!/bin/bash

../web/build.sh
(cd ../web/site; npx webpack)
rm -r static
mkdir static
cp -r ../res static
cp ../web/site/index.html static
cp -r ../web/site/dist/* static
