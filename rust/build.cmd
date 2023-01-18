cargo fmt
cargo build
REM cargo build --release

copy target\debug\libgrid.dll ..\python\grid.pyd
