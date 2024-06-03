.PHONY: run full-build build test clean

run:
	cargo run

full-build: 
	cargo clean && cargo build && cargo test

build:
	cargo build

test:
	cargo test

clean:
	cargo clean