#!/bin/bash

rm -r res src Cargo.toml Cargo.lock

rsync ../Cargo.toml ./Cargo.toml
rsync ../Cargo.lock ./Cargo.lock
rsync ../rust-toolchain ./rust-toolchain
rsync -r ../src .
mkdir -p res
rsync -r ../res/map res/

docker build -t bruno1996/elements2-server .
