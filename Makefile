.PHONY: help install dev build test lint format check audit clean

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install all dependencies
	cargo install cargo-leptos cargo-audit cargo-watch
	bun install

dev: ## Start development server
	bun run dev

build: ## Build for production
	bun run build

test: ## Run tests
	bun run test

lint: ## Run linting and fix issues
	bun run lint

format: ## Format all code
	bun run format

check: ## Run all checks (CI mode)
	bun run check

audit: ## Security audit
	cargo audit

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/
	rm -rf node_modules/