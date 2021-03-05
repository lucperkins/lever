.PHONY: check ci clippy dev fmt test

check:
	cargo check

ci: check fmt clippy test

clippy:
	cargo clippy -- --deny warnings

dev:
	cargo watch -x check

fmt:
	cargo fmt --all -- --check

test:
	cargo test
