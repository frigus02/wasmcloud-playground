.DEFAULT_GOAL:=help

keys: .keys/account.nk
keys: .keys/module-api.nk
keys: ## Generate keys

.keys/account.json:
	@mkdir -p .keys
	wash keys gen account -o json >$@

.keys/account.nk: .keys/account.json
	jq -r '.seed' <$< >$@

.keys/module-api.json:
	@mkdir -p .keys
	wash keys gen module -o json >$@

.keys/module-api.nk: .keys/module-api.json
	jq -r '.seed' <$< >$@

.PHONY: help
help: ## Display this help. Thanks to https://suva.sh/posts/well-documented-makefiles/
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
