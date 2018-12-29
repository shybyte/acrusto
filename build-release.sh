#!/usr/bin/env bash
# sudo apt-get install pkg-config libssl-dev
cargo build --release
# cargo bloat --release --crates -n 30
ls -l target/release/acrusto
strip target/release/acrusto
ls -l target/release/acrusto
~/opt/upx-3.95/upx target/release/acrusto
ls -l target/release/acrusto

# Disable jemalloc for smaller size
# use std::alloc::System;
# #[global_allocator]
# static GLOBAL: System = System;