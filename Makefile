.PHONY: build build-cli build-plugin release test clean install

# Default: build everything
build: build-cli build-plugin

# CLI only
build-cli:
	cargo build -p maestro

# Zellij plugin (WASM)
build-plugin:
	cargo build -p zellij-plugin --target wasm32-wasip1

# Release builds
release:
	cargo build --release -p maestro
	cargo build --release -p zellij-plugin --target wasm32-wasip1

# Run all tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install CLI and plugin
install:
	./install.sh

# Load plugin into Zellij
load-plugin: build-plugin
	./load-plugin.sh
