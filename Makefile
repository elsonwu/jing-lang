# Makefile for Jing Programming Language
.PHONY: help format check lint test build clean install dev-setup ci

# Default target
help:
	@echo "Available commands:"
	@echo "  format     - Format code using rustfmt"
	@echo "  check      - Check code compilation"
	@echo "  lint       - Run clippy lints"
	@echo "  test       - Run all tests"
	@echo "  build      - Build the project"
	@echo "  clean      - Clean build artifacts"
	@echo "  ci         - Run full CI pipeline locally"
	@echo "  dev-setup  - Set up development environment"
	@echo "  install    - Install the binary"

# Format code
format:
	@echo "🎨 Formatting code..."
	cargo fmt

# Check code compilation
check:
	@echo "🔍 Checking code compilation..."
	cargo check --all-targets --all-features

# Run clippy lints
lint:
	@echo "🔬 Running clippy lints..."
	cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test --all-features

# Build project
build:
	@echo "🔨 Building project..."
	cargo build --release

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Full CI pipeline
ci: format check lint test build
	@echo "✅ All CI checks passed!"

# Development environment setup
dev-setup:
	@echo "🛠️  Setting up development environment..."
	@echo "Installing Rust components..."
	rustup component add rustfmt clippy
	@echo "Development environment ready!"

# Install binary
install:
	@echo "📦 Installing jing binary..."
	cargo install --path .

# Quick development cycle
dev: format check test
	@echo "✅ Quick development checks passed!"

# Pre-commit checks (same as git hook)
pre-commit: format check lint test
	@echo "✅ Pre-commit checks passed!"

# Pre-push checks (same as git hook)  
pre-push: format check lint test build
	@echo "✅ Pre-push checks passed!"
