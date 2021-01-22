# wasmCloud TodoMVC

## Required tools

- Our modules are compiled to WebAssembly, so we need the wasm target:

  ```
  rustup target add wasm33-unknown-unknown
  ```

- We need `nk` and `wascap` to generate keys and sign our modules:

  ```
  cargo install wash-cli
  ```

## Initial setup

- Generate keys

  ```
  make keys
  ```
