#!/bin/sh
cargo fmt
cargo build
# cargo build --release

cp target/debug/libgrid.dylib ../python/grid.so
