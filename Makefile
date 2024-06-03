# This Makefile provides commands for building, testing, and cleaning up a Rust project.

# Phony targets
.PHONY: run full-build build test clean

# Run the application
run:
	cargo run

# Fully build the project: clean, build, and test
full-build:
	cargo clean && cargo build && cargo test

# Build the project
build:
	cargo build

# Test the project
test:
	cargo test

# Clean up the project
clean:
	cargo clean
