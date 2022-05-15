fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/include/cstruct.h");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=static=cstruct");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/include/cstruct.c")
        .compile("cstruct");
}
