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

# 2. Start inside wash (ACTOR_MODULE_KEY is visible after the make build command above)
wash up
ctl start actor api/target/wasm32-unknown-unknown/debug/api_s.wasm
ctl start provider wasmcloud.azurecr.io/httpserver:0.12.1
ctl link (ACTOR_MODULE_KEY) VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver PORT=8080

# 3. Call the API
curl localhost:8080
```
