.PHONY: help build test clean deploy-testnet fmt lint check install-deps

# Default target
help:
	@echo "Soroban Smart Contracts - Available Commands:"
	@echo ""
	@echo "  make install-deps    - Install Rust, Soroban CLI, and dependencies"
	@echo "  make build           - Build all contracts for WASM"
	@echo "  make test            - Run all tests"
	@echo "  make fmt             - Format code with rustfmt"
	@echo "  make lint            - Run clippy linter"
	@echo "  make check           - Run fmt and lint checks"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make deploy-testnet  - Deploy all contracts to testnet"
	@echo ""

# Install dependencies
install-deps:
	@echo "Installing dependencies..."
	@chmod +x setup.sh
	@./setup.sh

# Build all contracts
build:
	@echo "Building contracts..."
	@cargo build --target wasm32-unknown-unknown --release

# Run tests
test:
	@echo "Running tests..."
	@cargo test

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt

# Run linter
lint:
	@echo "Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Check formatting and linting
check: fmt lint
	@echo "All checks passed!"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

# Deploy to testnet (requires CONTRACT env var or deploys all)
deploy-testnet: build
	@echo "Deploying to testnet..."
	@if [ -z "$(CONTRACT)" ]; then \
		for wasm in target/wasm32-unknown-unknown/release/*.wasm; do \
			echo "Deploying $$wasm..."; \
			soroban contract deploy \
				--wasm "$$wasm" \
				--source default \
				--network testnet; \
		done \
	else \
		echo "Deploying $(CONTRACT)..."; \
		soroban contract deploy \
			--wasm target/wasm32-unknown-unknown/release/$(CONTRACT).wasm \
			--source default \
			--network testnet; \
	fi

# Build and test in one command
all: build test
	@echo "Build and test complete!"

# Watch mode for development (requires cargo-watch)
watch:
	@echo "Starting watch mode..."
	@cargo watch -x 'test' -x 'build --target wasm32-unknown-unknown --release'

# Optimize WASM files
optimize:
	@echo "Optimizing WASM files..."
	@for wasm in target/wasm32-unknown-unknown/release/*.wasm; do \
		echo "Optimizing $$wasm..."; \
		soroban contract optimize --wasm "$$wasm"; \
	done

# Generate documentation
docs:
	@echo "Generating documentation..."
	@cargo doc --no-deps --open
