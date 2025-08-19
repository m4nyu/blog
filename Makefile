.PHONY: help install dev build test lint lint-rust lint-frontend format format-rust format-frontend check audit clean

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install all dependencies
	cargo install cargo-leptos cargo-audit cargo-watch
	bun install

dev: ## Start development server
	cargo leptos watch

build: ## Build for production
	cargo leptos build --release

test: ## Run Rust tests
	cargo test

test-watch: ## Run Rust tests in watch mode
	cargo watch -x test

lint: ## Run all linters and fix issues
	@echo "🔍 Linting frontend files with Biome..."
	biome lint --write .
	@echo "🦀 Linting Rust code with clippy..."
	cargo clippy --all-targets --all-features

lint-frontend: ## Run frontend linting (Biome)
	biome lint --write .

lint-rust: ## Run Rust linting (clippy)
	cargo clippy --all-targets --all-features

format: ## Format all code
	@echo "🎨 Formatting frontend files with Biome..."
	biome format --write .
	@echo "🦀 Formatting Rust code with rustfmt..."
	cargo fmt

format-frontend: ## Format frontend code (Biome)
	biome format --write .

format-rust: ## Format Rust code (rustfmt)
	cargo fmt

check: ## Run all checks (format, lint, test)
	@echo "🔍 Running all quality checks..."
	biome check --write .
	cargo fmt
	cargo clippy --all-targets --all-features
	cargo test

check-ci: ## Run CI checks (no auto-fix)
	@echo "🔍 Running CI quality checks..."
	biome ci .
	cargo fmt --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

audit: ## Security audit
	cargo audit

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/
	rm -rf node_modules/