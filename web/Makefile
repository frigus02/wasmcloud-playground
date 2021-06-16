.DEFAULT_GOAL:=help
DEBUG = target/wasm32-unknown-unknown/debug
RELEASE = target/wasm32-unknown-unknown/release
VERSION = $$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
REVISION = $$(test -f $(DEBUG)/web_s.wasm && wash claims inspect $(DEBUG)/web_s.wasm -o json | jq -r '.revision' || echo -1)

.PHONY: build
build: ## Debug build
	@cargo build
	@wash claims sign \
		$(DEBUG)/web.wasm \
		--name web \
		--ver $(VERSION) \
		--rev $$(( $(REVISION) + 1)) \
		--call-alias web

.PHONY: release
release: ## Release build
	@cargo build --release
	@wash claims sign \
		$(RELEASE)/web.wasm \
		--name web \
		--ver $(VERSION) \
		--rev 0 \
		--call-alias web
	@wash reg push \
		localhost:5000/web:$(VERSION) \
		$(RELEASE)/web_s.wasm \
		--insecure

.PHONY: help
help: ## Display this help. Thanks to https://suva.sh/posts/well-documented-makefiles/
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
