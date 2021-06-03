.DEFAULT_GOAL:=help
DEBUG = target/wasm32-unknown-unknown/debug
RELEASE = target/wasm32-unknown-unknown/release

.PHONY: run
run: ## Run modules in wasmcloud
	$(MAKE) -C api release
	$(MAKE) -C front release
	$(MAKE) -C todo release
	$(MAKE) -C web release
	FRONT_ACTOR=$$(wash claims inspect -o json front/$(RELEASE)/front_s.wasm | jq -r .module) TODO_ACTOR=$$(wash claims inspect -o json todo/$(RELEASE)/todo_s.wasm | jq -r .module) wasmcloud --manifest ./manifest.yaml

.PHONY: watch
watch: ## Run modules in wash cli watching them for changes
	$(MAKE) -C api build
	$(MAKE) -C front build
	$(MAKE) -C todo build
	$(MAKE) -C web build
	FRONT_ACTOR=$$(wash claims inspect -o json front/$(DEBUG)/front_s.wasm | jq -r .module) TODO_ACTOR=$$(wash claims inspect -o json todo/$(DEBUG)/todo_s.wasm | jq -r .module) wash up --manifest ./manifest-debug.yaml --watch ./api/$(DEBUG)/api_s.wasm --watch ./front/$(DEBUG)/front_s.wasm --watch ./todo/$(DEBUG)/todo_s.wasm --watch ./web/$(DEBUG)/web_s.wasm

.PHONY: help
help: ## Display this help. Thanks to https://suva.sh/posts/well-documented-makefiles/
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
