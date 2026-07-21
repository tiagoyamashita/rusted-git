
.PHONY: debug build-release release-linux-musl test clippy clippy-pedantic install install-debug sort

ARGS=-l
# ARGS=-l -d ~/code/extern/kubernetes
# ARGS=-l -d ~/code/extern/linux
# ARGS=-l -d ~/code/git-bare-test.git -w ~/code/git-bare-test

profile:
	CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --features timing -- ${ARGS}

run-timing:
	cargo run --features=timing --release -- ${ARGS}

debug:
	RUST_BACKTRACE=true cargo run --features=timing -- ${ARGS}

build-release:
	cargo build --release --locked

release-mac: build-release
	strip target/release/rusted-git
	otool -L target/release/rusted-git
	ls -lisah target/release/rusted-git
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/rusted-git-mac.tar.gz ./rusted-git
	ls -lisah ./release/rusted-git-mac.tar.gz

release-mac-x86: build-apple-x86-release
	strip target/x86_64-apple-darwin/release/rusted-git
	otool -L target/x86_64-apple-darwin/release/rusted-git
	ls -lisah target/x86_64-apple-darwin/release/rusted-git
	mkdir -p release
	tar -C ./target/x86_64-apple-darwin/release/ -czvf ./release/rusted-git-mac-x86.tar.gz ./rusted-git
	ls -lisah ./release/rusted-git-mac-x86.tar.gz

release-win: build-release
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/rusted-git-win.tar.gz ./rusted-git.exe
	cargo install cargo-wix --version 0.3.3 --locked
	cargo wix -p rusted-git --no-build --nocapture --output ./release/rusted-git-win.msi
	ls -l ./release/rusted-git-win.msi

release-linux-musl: build-linux-musl-release
	strip target/x86_64-unknown-linux-musl/release/rusted-git
	mkdir -p release
	tar -C ./target/x86_64-unknown-linux-musl/release/ -czvf ./release/rusted-git-linux-x86_64.tar.gz ./rusted-git

build-apple-x86-debug:
	cargo build --target=x86_64-apple-darwin

build-apple-x86-release:
	cargo build --release --target=x86_64-apple-darwin --locked

build-linux-musl-debug:
	cargo build --target=x86_64-unknown-linux-musl

build-linux-musl-release:
	cargo build --release --target=x86_64-unknown-linux-musl --locked

test-linux-musl:
	cargo nextest run --workspace --target=x86_64-unknown-linux-musl

# aarch64 test binaries are cross-compiled, so CI runs them under qemu via a
# CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER (see .github/workflows/ci.yml).
# Exclude test_hook_with_missing_shebang: it needs the kernel's ENOEXEC (so
# rusted-git retries the hook via `sh`), which qemu-user doesn't emulate — the exec
# just exits 127. It passes on real aarch64 hardware.
test-linux-arm:
	cargo nextest run --workspace --target=aarch64-unknown-linux-gnu -E 'not test(test_hook_with_missing_shebang)'

release-linux-arm: build-linux-arm-release
	mkdir -p release

	aarch64-linux-gnu-strip target/aarch64-unknown-linux-gnu/release/rusted-git
	arm-linux-gnueabihf-strip target/armv7-unknown-linux-gnueabihf/release/rusted-git
	arm-linux-gnueabihf-strip target/arm-unknown-linux-gnueabihf/release/rusted-git

	tar -C ./target/aarch64-unknown-linux-gnu/release/ -czvf ./release/rusted-git-linux-aarch64.tar.gz ./rusted-git
	tar -C ./target/armv7-unknown-linux-gnueabihf/release/ -czvf ./release/rusted-git-linux-armv7.tar.gz ./rusted-git
	tar -C ./target/arm-unknown-linux-gnueabihf/release/ -czvf ./release/rusted-git-linux-arm.tar.gz ./rusted-git

build-linux-arm-debug:
	cargo build --target=aarch64-unknown-linux-gnu
	cargo build --target=armv7-unknown-linux-gnueabihf
	cargo build --target=arm-unknown-linux-gnueabihf

build-linux-arm-release:
	cargo build --release --target=aarch64-unknown-linux-gnu --locked
	cargo build --release --target=armv7-unknown-linux-gnueabihf --locked
	cargo build --release --target=arm-unknown-linux-gnueabihf --locked

test:
	cargo nextest run --workspace

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy --workspace --all-features

clippy-nightly:
	cargo +nightly clippy --workspace --all-features

check: fmt clippy test sort deny

check-nightly:
	cargo +nightly c
	cargo +nightly clippy --workspace --all-features
	cargo +nightly t

deny:
	cargo deny check

sort:
	tombi format --check

install:
	cargo install --path "." --offline --locked

install-timing:
	cargo install --features=timing --path "." --offline --locked

licenses:
	cargo bundle-licenses --format toml --output THIRDPARTY.toml

clean:
	cargo clean
