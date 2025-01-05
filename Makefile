.PHONY: all build run test lint clean help

# Colors for terminal output
GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
RESET  := $(shell tput -Txterm sgr0)

# Default target
.DEFAULT_GOAL := help

help: ## Show this help
	@echo 'Usage:'
	@echo '  ${YELLOW}make${RESET} ${GREEN}<target>${RESET}'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  ${YELLOW}%-15s${GREEN}%s${RESET}\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project
	@echo "Building project..."
	cargo build

run: build ## Run the server
	@echo "Starting server..."
	cargo run

test: ## Run tests
	@echo "Running tests..."
	cargo test

lint: ## Run linter and formatter
	@echo "Running clippy..."
	cargo clippy -- -D warnings
	@echo "Running formatter..."
	cargo fmt --all -- --check

clean: ## Clean build artifacts
	@echo "Cleaning build artifacts..."
	cargo clean

check: lint test ## Run all checks (lint and test)
	@echo "All checks passed!"
