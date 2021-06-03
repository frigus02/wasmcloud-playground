.DEFAULT_GOAL:=help
DEBUG = target/wasm32-unknown-unknown/debug
RELEASE = target/wasm32-unknown-unknown/release

.PHONY: build
build: ## Debug build
	@cargo build
	@wash claims sign \
		$(DEBUG)/front.wasm \
		--name front

.PHONY: release
release: ## Release build
	@cargo build --release
	@wash claims sign \
		$(RELEASE)/front.wasm \
		--name front

.PHONY: help
help: ## Display this help. Thanks to https://suva.sh/posts/well-documented-makefiles/
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)