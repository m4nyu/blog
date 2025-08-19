.PHONY: help install dev build test lint format check audit clean

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install all dependencies
	cargo install cargo-leptos cargo-audit
	bun install

dev: ## Start development server
	cargo leptos watch

build: ## Build for production
	cargo leptos build --release

test: ## Run tests
	cargo test

lint: ## Run linters and fix issues
	bun run lint

format: ## Format code
	bun run format

check: ## Run all checks (format, lint, test)
	bun run check

check-ci: ## Run CI checks (no auto-fix)
	bun run check:ci

audit: ## Security audit
	cargo audit

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/
	rm -rf node_modules/