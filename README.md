# wasmcloud Playground

## Required tools

- Our modules are compiled to WebAssembly, so we need the wasm target:

  ```
  rustup target add wasm32-unknown-unknown
  ```

- We need `wash` (the WebAssembly shell) to generate keys and sign our modules:

  ```
  brew tap wasmcloud/wasmcloud
  brew install wasmcloud wash
  ```

- For code generation we also need the waPC CLI:

  ```
  brew install wapc/tap/wap
  ```

- For data storage we need a local Redis instance:

  ```
  docker run -d -p 6379:6379 --name todo-redis redis
  ```

## Start the app

```sh
# 1. Compile modules and start them in wasmcloud
make run

# 2. Call the API
curl localhost:8080
```
