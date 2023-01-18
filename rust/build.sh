#!/bin/sh
cargo fmt
# cargo build
cargo build --release

# cp target/release/libgrid.dylib ../examples/python/asynchronous/grid.so
# cp target/release/libgrid.dylib ../examples/python/synchronous/grid.so
