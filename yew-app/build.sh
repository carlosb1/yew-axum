#!/bin/bash

# build for wasm
wasm-pack build --target web --release

# try to run via trunk serve
# trunk serve