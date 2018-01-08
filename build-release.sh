#!/usr/bin/env bash
# sudo apt-get install pkg-config libssl-dev
cargo build --release
strip target/release/acrusto
ls -l target/release/acr*