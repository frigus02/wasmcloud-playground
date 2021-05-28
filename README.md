# wasmCloud Playground

## Required tools

- Our modules are compiled to WebAssembly, so we need the wasm target:

  ```
  rustup target add wasm33-unknown-unknown
  ```

- We need `wash` (the WebAssembly shell) to generate keys and sign our modules:

  ```
  brew tap wasmcloud/wasmcloud
  brew install wasmcloud wash
  ```

## Start the app

```sh
# 1. Build the API
cd api && make build

# 2. Start the host
cd host && make run

# 3. Call the API
curl localhost:8081
```
