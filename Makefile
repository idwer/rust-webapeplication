all: build check test
	
build:
	cargo build --release

check:
	cargo check

run_log:
	RUST_LOG=actix_web=info cargo run

run_log_release:
	RUST_LOG=actix_web=info cargo run -r

test:
	cargo test
