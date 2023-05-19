.PHONY: fmt
fmt:
	cargo fmt
	cargo clippy --fix --allow-dirty --all-features

.PHONY: linux
linux:
	cross build -r --target x86_64-unknown-linux-musl

#todo
#.PHONY: windows
#windows:
#	cross build -r --target x86_64-pc-windows-gnu

.PHONY: freebsd
freebsd:
	cross build -r --target x86_64-unknown-freebsd

.PHONY: android
android:
	cross build -r --target aarch64-linux-android

.PHONY: all
all: linux freebsd android
