#!/usr/bin/env bash

rm assets/*.rs

cargo build --release

for f in assets/*.json; do
    target/release/jsonsd "$f" > "${f%.json}.rs"
done
