all: build check test
	
build:
	cargo build --release

check:
	cargo check

test:
	cargo test
