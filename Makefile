all: build check test
	
build:
	cargo build

build_rel:
	cargo build --release

check:
	cargo check

check_rel:
	cargo check -r

run_log:
	RUST_LOG=actix_web=info cargo run

run_log_release:
	RUST_LOG=actix_web=info cargo run -r

test:
	cargo test

test_rel:
	cargo test -r
