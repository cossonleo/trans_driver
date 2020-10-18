
all: trans_driver

.PHONY: trans_driver

trans_driver:
	mkdir -p bin 
	cargo build --release 
	cp -f target/release/trans_driver bin/
