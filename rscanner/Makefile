.PHONY: check
check:
	cargo fmt
	cargo tomlfmt
	cargo clippy
	cargo install --locked cargo-outdated
	cargo outdated -R
	cargo install cargo-udeps --locked
	cargo +nightly udeps
