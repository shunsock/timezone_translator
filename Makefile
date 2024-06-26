.PHONY: install uninstall

install:
	cargo build --release
	sudo cp target/release/timezone_translator /usr/local/bin/tzt

uninstall:
	sudo rm /usr/local/bin/tzt