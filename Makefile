.DEFAULT_GOAL:=help

.PHONY: run
run: ## Run modules in wasmcloud
	$(MAKE) -C api build
	$(MAKE) -C todo build
	$(MAKE) -C web build
	API_ACTOR=$$(wash claims inspect -o json api/target/wasm32-unknown-unknown/debug/api_s.wasm | jq -r .module) TODO_ACTOR=$$(wash claims inspect -o json todo/target/wasm32-unknown-unknown/debug/todo_s.wasm | jq -r .module) WEB_ACTOR=$$(wash claims inspect -o json web/target/wasm32-unknown-unknown/debug/web_s.wasm | jq -r .module) wasmcloud --manifest ./manifest.yaml

.PHONY: help
help: ## Display this help. Thanks to https://suva.sh/posts/well-documented-makefiles/
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
