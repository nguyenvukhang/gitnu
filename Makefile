BIN=$(HOME)/dots/personal/.local/bin
GITNU_RELEASE_BIN=$(PWD)/target/release/gitnu
GITNU_DEBUG_BIN=$(PWD)/target/debug/gitnu

quick:
	cargo build

build:
	cargo test
	cargo build --release
	make load_bin

size:
	du -sh target/*/gitnu

bench:
	cargo build --release
	cargo bench --quiet

test:
	cargo test --no-fail-fast

version:
	@CARGO_MANIFEST_DIR=$(PWD) bash scripts/version.sh

publish:
	@CARGO_MANIFEST_DIR=$(PWD) bash scripts/publish.sh

# copies built binary to a path specified by $BIN
load_bin:
	@rm -f $(BIN)/gitnu
	@cp ./target/release/gitnu $(BIN)

.PHONY: test size quick load_bin bench
