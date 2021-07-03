#!make
.PHONY: start setup clean lint

start:
	@PKGLS_LOG=debug cargo watch -x build

setup:
	@cargo install cargo-watch

clean:
	@cargo clean

test:
	@cargo clippy
	@cargo test

build:
	@cargo build
