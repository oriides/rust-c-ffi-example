/// For small projects this file can be created manually, for larger projects I strongly recommend using [bindgen](https://crates.io/crates/bindgen) to generate these bindings automatically.

// A Rust-idiomatic representation of the struct in the C file we want to use
#[repr(C)] // Instruction for the Rust compiler to represent the data of the struct in a C code compliant way
struct CStruct {
    i: i32,
}

#[link(name = "cstruct")] // Instruction for the Rust compiler to link to the `cstruct.c` file here
extern "C" {
    // Declaration of the external C functions = bindings
    fn new_c(i: i32) -> *mut CStruct;
    fn drop_c(s: *mut CStruct);
}
