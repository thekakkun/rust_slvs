# rust_slvs

[Rust](https://www.rust-lang.org/) binding for [SolveSpace](https://github.com/solvespace/solvespace/)'s geometric constraint solver library.

[**Documentation**](https://docs.rs/slvs/)

## Using slvs

### Prerequisites

- A C compiler

  The SolveSpace library is compiled using [cc](https://docs.rs/cc/latest/cc/), which requires a C compiler.

  [See here for more information](https://github.com/rust-lang/cc-rs#compile-time-requirements).

- libclang

  Bindings from C to Rust are generated using [bindgen](https://github.com/rust-lang/rust-bindgen), which requires `libclang`. On Windows, the environment variable `LIBCLANG_PATH` will need to be set, pointing to the location of the clang library.

  [See here for more information](https://rust-lang.github.io/rust-bindgen/requirements.html).

### To add to your project

Run the following Cargo command in your project directory:

```shell
cargo add slvs
```

### To Build

1. Check out any necessary submodules

   ```shell
   git submodule update --init --recursive
   ```

2. Run the build command

   ```shell
   cargo build
   ```
