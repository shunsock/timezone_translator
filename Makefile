.PHONY: install uninstall test

install:
	cargo build --release
	sudo cp target/release/timezone_translator /usr/local/bin/tzt

uninstall:
	sudo rm /usr/local/bin/tzt

test:
	cargo build
	cargo fmt
	cargo clippy
	cargo test
