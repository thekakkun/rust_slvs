# rust_slvs

[Rust](https://www.rust-lang.org/) binding for [SolveSpace](https://github.com/solvespace/solvespace/)'s geometric constraint solver library.

## Using rust_slvs

### Prerequisites

- [A C compiler](https://github.com/rust-lang/cc-rs#c-support)

  Needed to compile SolveSpace using the [cc library](https://docs.rs/cc/latest/cc/).

### To use as a library

1. Add the crate to your `Cargo.toml`.

   ```toml
   [dependencies]
   slvs = { git = "https://github.com/thekakkun/rust_slvs.git" }
   ```

2. You may need to create a Cargo configuration file (`.cargo/config.toml`) so that all submodules can be downloaded.

   ```toml
   [net]
   git-fetch-with-cli = true  # use the `git` executable for git operations
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
