.PHONY: check
check:
	cargo fmt
	cargo tomlfmt
	cargo clippy --all-targets
	cargo install --locked cargo-outdated
	cargo outdated -R
	cargo install cargo-udeps --locked
	cargo +nightly udeps
