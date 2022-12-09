.PHONY: init

configure-rust:
	rustup install 1.62.1
	rustup override set 1.62.1
	rustup toolchain install nightly-2022-08-08
	rustup target add wasm32-unknown-unknown --toolchain nightly-2022-08-08
	rustup component add clippy
init:
	make configure-rust
	git submodule update --init --recursive

generate-specs:
	./scripts/setup-paralink-2001.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --all

.PHONY: run
run:
	./scripts/run-paralink.sh

.PHONY: build
build:
	cargo build --release

.PHONY: watch
watch:
	SKIP_WASM_BUILD=1 cargo watch -c -x build

.PHONY: doc
doc:
	SKIP_WASM_BUILD=1 cargo doc --open
