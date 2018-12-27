#!/usr/bin/env bash
# sudo apt-get install pkg-config libssl-dev
cargo build --release
strip target/release/acrusto
ls -l target/release/acr*

# Disable jemalloc for smaller size
# use std::alloc::System;
# #[global_allocator]
# static GLOBAL: System = System;