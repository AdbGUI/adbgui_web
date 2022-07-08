DISTPATH := ./dist
HOOKSPATH := ./githooks

install:
	rustup target add wasm32-unknown-unknown
	cargo install trunk wasm-bindgen-cli

setup:
	git config --local core.hooksPath
	cargo test

run:
	trunk serve

build:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check
	trunk build --release

clean:
	rm -fr $(DISTPATH)

hard-clean:
	rm -fr $(DISTPATH)
	rm -fr ./target
