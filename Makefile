generate-protocols:
	scripts/gen-proto

build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test -- --nocapture'
