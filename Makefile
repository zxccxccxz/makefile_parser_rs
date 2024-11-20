build:
	cargo build
	cargo build --release

run:
	cargo run --release $(args)

test:
	cargo test --test parser_test

clippy:
	cargo clippy
	cargo clippy --release
	cargo clippy --tests

fix:
	cargo clippy --fix --lib -p makefile_parser_rs --allow-dirty

fmt:
	cargo fmt --all

clean:
	cargo clean

install:
	cargo install --path .