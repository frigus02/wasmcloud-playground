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

## Start the app

```sh
# 1. Build the API
cd api && make build

# 2. Build the Todo actor
cd todo && make build

# 3. Start host (API_ACTOR is the module key visible after the above make build command)
docker run -d -p 6379:6379 redis
API_ACTOR=??? TODO_ACTOR=??? wasmcloud --manifest ./manifest.yaml

# 4. Call the API
curl localhost:8080
```
