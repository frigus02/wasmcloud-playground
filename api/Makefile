.DEFAULT_GOAL:=help
DEBUG = target/wasm32-unknown-unknown/debug
RELEASE = target/wasm32-unknown-unknown/release
KEYDIR = ../.keys

keys:
	@$(MAKE) -C .. keys

.PHONY: build
build: keys ## Debug build
	cargo build
	wash claims sign \
		--issuer $(KEYDIR)/account.nk \
		--subject $(KEYDIR)/module-api.nk \
		--http_server \
		$(DEBUG)/api.wasm \
		--name api
	wash claims inspect $(DEBUG)/api_s.wasm

.PHONY: release
release: keys ## Release build
	cargo build --release
	wash claims sign \
		--issuer $(KEYDIR)/account.nk \
		--subject $(KEYDIR)/module-api.nk \
		--http_server \
		$(RELEASE)/api.wasm \
		--name api
	wash claims inspect $(RELEASE)/api_s.wasm

.PHONY: help
help: ## Display this help. Thanks to https://suva.sh/posts/well-documented-makefiles/
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
