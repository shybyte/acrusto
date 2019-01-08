#!/usr/bin/env bash
# sudo apt-get install pkg-config libssl-dev
cargo build --release
# cargo bloat --release --crates -n 30
ls -l target/release/acrusto
strip target/release/acrusto
ls -l target/release/acrusto
#~/opt/upx-3.95/upx --best --ultra-brute target/release/acrusto
~/opt/upx-3.95/upx --best --brute target/release/acrusto
ls -l target/release/acrusto

# Disable jemalloc for smaller size
# use std::alloc::System;
# #[global_allocator]
# static GLOBAL: System = System;

cp target/release/acrusto  ~/bin/
#
#export PKG_CONFIG_ALLOW_CROSS=1
#export OPENSSL_INCLUDE_DIR=/usr/include/openssl
#cargo build --release --target=x86_64-unknown-linux-musl

