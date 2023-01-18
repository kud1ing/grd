fn main() {
    // TODO
    cxx_build::bridge("src/lib.rs")
        //.file("src/grid.cc")
        .flag_if_supported("-std=c++11")
        .compile("grid");

    println!("cargo:rerun-if-changed=src/lib.rs");
    /*
    println!("cargo:rerun-if-changed=src/grid.cc");
    println!("cargo:rerun-if-changed=include/grid.h");
    */
}
