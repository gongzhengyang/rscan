export VERSION=$(shell head Cargo.toml -n 3 | tail -n 1| awk '{ print $$3}' | sed 's/"//g')
export RUSTFLAGS=--cfg tokio_unstable -C target-feature=+crt-static

.PHONY: all
all: linux freebsd android macos-intel macos-arm

.PHONY: linux
linux:
	ARCH=x86_64-unknown-linux-musl bash build-release.sh

#todo
#.PHONY: windows
#windows:
#	cross build -r --target x86_64-pc-windows-gnu

.PHONY: freebsd
freebsd:
	ARCH=x86_64-unknown-freebsd bash build-release.sh

.PHONY: android
android:
	ARCH=aarch64-linux-android bash build-release.sh

.PHONY: macos-intel
macos-intel:
	ARCH=x86_64-apple-darwin CARGO_PROFILE_RELEASE_STRIP=false  bash build-release.sh

.PHONY: macos-arm
macos-arm:
	ARCH=aarch64-apple-darwin CARGO_PROFILE_RELEASE_STRIP=false  bash build-release.sh

.PHONY: check
check:
	cargo fmt
	cargo tomlfmt
	cargo install --locked cargo-outdated
	cargo outdated -R
