all: build

build:
	cargo build --release

install:
	sudo cp target/release/linux-on-drugs /usr/bin/linux-on-drugs

uninstall:
	sudo rm -f /usr/bin/linux-on-drugs