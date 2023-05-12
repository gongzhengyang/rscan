.PHONY: fmt
fmt:
	cargo fmt
	cargo clippy --fix --allow-dirty --all-features

.PHONY: linux
linux:
	cross build -r --target x86_64-unknown-linux-musl

.PHONY: windows
windows:
	cross build -r --target x86_64-pc-windows-gnu
