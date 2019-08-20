.PHONY: install help
.DEFAULT_GOAL := help

install: ## Install tmg for local user.
	cp target/release/rust /usr/local/bin/tmg

help: ## This help.
	@echo " ______ __   __  ____  "
	@echo "|______|  \_/  |/ ___| "
	@echo "  |  | | |\_/| | |  _  "
	@echo "  |  | | |   | | |_| | "
	@echo "  |__| |_|   |_|\____| "
	@echo "                       "
	@echo "====== Makefile =======\n"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)