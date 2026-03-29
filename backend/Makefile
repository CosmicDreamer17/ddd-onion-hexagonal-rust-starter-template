check-arch:
	@! grep -r "infra" crates/domain

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

test-all:
	cargo test --workspace

check-deps:
	cargo machete
	cargo deny check
