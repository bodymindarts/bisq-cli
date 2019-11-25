generate-protocols:
	scripts/gen-proto

build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test -- --nocapture'

integration:
	cargo build
	export BISQ_CLI_BIN_DIR="$(if $(BISQ_CLI_BIN_DIR),$(BISQ_CLI_BIN_DIR),./target/debug)" && bats -t -r test/integration

test-in-ci:
	cargo clippy --all-features
	cargo test --all-features --verbose --locked

build-arm-unknown-linux-gnueabihf-release:
	cargo build --locked --release --target arm-unknown-linux-gnueabihf

build-x86_64-unknown-linux-gnu-release:
	cargo build --locked --release --target x86_64-unknown-linux-gnu
