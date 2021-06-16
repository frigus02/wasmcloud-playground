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

- For data storage we need a local Redis instance and we use a local Docker registry to store our signed modules:

  ```
  docker compose up -d
  ```

## Start the app

1. `make run` compiles the modules in release mode and starts them in wasmcloud
1. Open your browser at http://localhost:8080

## Develop on the app

1. `make watch` compiles the modules in debug mode, starts then in wash up and watches for changes
1. Open your browser at http://localhost:8080
1. Make changes in any of the modules and compile it with `make build`. The actor should be updated automatically. Reload the page in your browser to see the changes
