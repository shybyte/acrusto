#!/usr/bin/env bash
# sudo apt-get install pkg-config libssl-dev
cargo build --release --target=x86_64-unknown-linux-musl --features vendored

ls -l target/x86_64-unknown-linux-musl/release/acrusto
strip target/x86_64-unknown-linux-musl/release/acrusto
ls -l target/x86_64-unknown-linux-musl/release/acrusto
#~/opt/upx-3.95/upx --best --ultra-brute target/release/acrusto
~/opt/upx-3.95/upx --best --brute target/x86_64-unknown-linux-musl/release/acrusto
ls -l target/x86_64-unknown-linux-musl/release/acrusto

# Disable jemalloc for smaller size
# use std::alloc::System;
# #[global_allocator]
# static GLOBAL: System = System;

#cp target/release/acrusto  ~/bin/

#export PKG_CONFIG_ALLOW_CROSS=1
#export OPENSSL_INCLUDE_DIR=/usr/include/openssl
#cargo build --release --target=x86_64-unknown-linux-musl

