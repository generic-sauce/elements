#!/bin/bash


ELEMENTS_ROOT_DIR="${PWD%/*/*}"

(cd $ELEMENTS_ROOT_DIR/web_client/; ./build.sh)
(cd $ELEMENTS_ROOT_DIR/web_client/site; npx webpack)
if [ -d static ]; then
	rm -r static
fi
mkdir static
cp -r $ELEMENTS_ROOT_DIR/res static
cp $ELEMENTS_ROOT_DIR/web_client/site/index.html static
cp -r $ELEMENTS_ROOT_DIR/web_client/site/dist/* static
