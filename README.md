# Rust C Foreign Function Interface (FFI) Example

Rust allows the inclusion of program-code written in other languages via foreign function interfaces (FFIs).

Projects like [Rust-for-Linux](https://github.com/Rust-for-Linux) make heavy use of this in order to build Rust APIs for the Linux-Kernel that enable the development of kernel code using the Rust programming language.

This repo acts as an example on how to include a C file into a Rust project.

At the center of this C FFI stands the [cc-rs crate](https://crates.io/crates/cc) which is a compile-time library to compile C/C++/assembly into a Rust library/application.

See the [README.md for the cc-rs crate](https://github.com/rust-lang/cc-rs/blob/main/README.md) for more usage instructions (e.g platform support).

**⚠ BEWARE:** Rust FFIs are not beginner friendly and require pretty extensive knowledge of both programming languages. If you want to use FFIs you have to deal with `unsafe`, which means **you are responsible** for a correct/safe/sound program.

## Trying out the example

Run the following command to build and run the example program

```bash
cargo run
```

Output should look something like this:

```stdout
Compiling libc v0.2.124
Compiling jobserver v0.1.24
Compiling cc v1.0.73
Compiling libcstruct-sys v0.1.0 (/home/nikolai/Development/Rust/rust-c-ffi-example)
 Finished dev [unoptimized + debuginfo] target(s) in 0.74s
  Running `target/debug/libcstruct-sys`
my_struct->i = 1
```

if the output `my_struct->i = 1` appears the FFI works.

## Repo Structure

```text
.
├── src/
│   ├── include/
│   │   └── cstruct.c
│   ├── bindings.rs
│   └── main.rs
├── build.rs
└── Cargo.toml
```

Let me point out the most important bits:

- `Cargo.toml` is where project information and dependencies are tracked. Notice the `cc` build-dependency as well as the `links = "cstruct"` and `build = "build.rs"` declarations.
- `build.rs` contains the custom build instructions for the rust compiler. Notice the `cc::Build::new().file("src/include/cstruct.c").compile("cstruct");` command which tells the rust compiler to build the `src/include/cstruct.c` file.
- `main.rs` is where the rust main function lives. This file also contains an wrapper struct in idiomatic Rust around the C struct we are linking to. We are also including the bindings contained in `bindings.rs`
- `bindings.rs` contains the bindings to the C struct and its functions.
- `cstruct.c` contains an implementation of a struct in C code. We want to use this implementation in our Rust code.