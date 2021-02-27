check:
	cargo check

clippy:
	cargo clippy -- --deny warnings

dev:
	cargo watch -x check

fmt:
	cargo fmt --all -- --check

test:
	cargo test

ci: check fmt clippy test