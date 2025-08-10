# Saelient

A native Rust library that provides high-level abstractions over key [SAE J1939](https://en.wikipedia.org/wiki/SAE_J1939) concepts.

## Features

- `std` (default) enables the use of slices owned by the library.
- `alloc` enables the use of slices owned by the library.
- `defmt-1` enables [`defmt`](https://crates.io/crates/defmt) formatting on
  relevant types.
