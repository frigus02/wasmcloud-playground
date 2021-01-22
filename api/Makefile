.DEFAULT_GOAL:=help
DEBUG = target/wasm32-unknown-unknown/debug
RELEASE = target/wasm32-unknown-unknown/release
KEYDIR = ../.keys

keys:
	@$(MAKE) -C .. keys

.PHONY: build
build: keys ## Debug build
	cargo build
	wascap sign $(DEBUG)/api.wasm $(DEBUG)/api_signed.wasm -i $(KEYDIR)/account.nk -u $(KEYDIR)/module-api.nk -s -n api

.PHONY: release
release: keys ## Release build
	cargo build --release
	wascap sign $(RELEASE)/api.wasm $(RELEASE)/api_signed.wasm -i $(KEYDIR)/account.nk -u $(KEYDIR)/module-api.nk -s -n api

.PHONY: help
help: ## Display this help. Thanks to https://suva.sh/posts/well-documented-makefiles/
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
