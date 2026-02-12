.PHONY: fmt lint test check dev

fmt:
	cargo fmt --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --lib

check: fmt lint test

dev:
	cargo run
