generate-protocols:
	scripts/gen-proto

build:
	cargo build

test:
	RUST_BACKTRACE=full cargo watch -s 'cargo test -- --nocapture'

integration:
	cargo build
	export BISQ_CLI_BIN_DIR="$(if $(BISQ_CLI_BIN_DIR),$(BISQ_CLI_BIN_DIR),./target/debug)" && bats -t -r test/integration
