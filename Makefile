.PHONY: build test clean deploy

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test

test-verbose:
	cargo test -- --nocapture --test-threads=1

clean:
	cargo clean

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

all: fmt lint test build

help:
	@echo "Available targets:"
	@echo "  make build       - Build WASM contracts"
	@echo "  make test        - Run tests"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make fmt         - Format code"
	@echo "  make lint        - Run clippy linter"
	@echo "  make all         - Format, lint, test, build"
	@echo "  make help        - Show this help message"