#!/bin/bash

TARGET_RELEASE="target/release"
TARGET_DEBUG="target/debug"
REL=release
DEBUG=debug

set -e
clear
cargo clean
rm -rf bin/*

if [ "$1" = "release" ]; then
    cargo build --release
    mkdir -p "bin/$REL"
    ls -l "$TARGET_RELEASE"
    mv "$TARGET_RELEASE/crystal-caverns" "bin/$REL/crystal-caverns"
    cp -r "assets" "bin/$REL/assets"
fi

if [ "$1" = "debug" ]; then
    cargo build
    mkdir -p "bin/$DEBUG"
    ls -l "$TARGET_DEBUG"
    mv "$TARGET_DEBUG/crystal-caverns" "bin/$DEBUG/crystal-caverns"
    cp -r "assets" "bin/$DEBUG/assets"
fi
