all: install

install:
	cargo build --release
	sudo cp target/release/linux-on-drugs /usr/bin/linux-on-drugs

uninstall:
	sudo rm -f /usr/bin/linux-on-drugs