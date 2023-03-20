# rust_slvs

[Rust](https://www.rust-lang.org/) binding for [Solvespace](https://github.com/solvespace/solvespace/).

## Prerequisites

### SolveSpace library

1. Check out any necessary submodules

   ```shell
   git submodule update --init --recursive
   ```

2. Follow build instructions for [SolveSpace](https://github.com/solvespace/solvespace/#building-on-linux)

   On the make step, Specify the target as `slvs` so that only the static library is built.

   ```shell
   make slvs
   ```
