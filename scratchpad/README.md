# scratchpad

Practice crate for *Rust Atomics and Locks* by Mara Bos.

## Layout

| Path | Contents |
|---|---|
| `examples/` | Demo chapters (1, 2, 3, 7, 8). One runnable file per topic. |
| `src/` | Build-your-own chapters (4, 5, 6, 9) with inline tests. |

## Commands

Run from this directory.

```sh
# List examples
cargo run --example

# Run an example
cargo run --example ch01_basics
cargo run --release --example ch02_atomic_load_and_store

# Build all examples
cargo build --examples

# Test the build-your-own chapters
cargo test
cargo test ch04_spin_lock

# Lint
cargo clippy --all-targets
```

Examples can use library code with `use scratchpad::...;`.
