.PHONY: install uninstall

install:
	cargo build --release
	sudo cp target/release/timezone_converter /usr/local/bin/tzconv

uninstall:
	sudo rm /usr/local/bin/tzconv