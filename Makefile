BIN=$(HOME)/dots/personal/.local/bin
GITNU_RELEASE_BIN=$(PWD)/target/release/git-nu
GITNU_DEBUG_BIN=$(PWD)/target/debug/git-nu

dev:
	cargo build
	cargo test --lib

build:
	cargo build --release
	@echo "Release build complete."

size:
	du -sh target/*/git-nu

bench:
	cargo build --release
	cargo bench --quiet

bench-wrap:
	cargo build --release
	cargo bench-wrap --quiet

test:
	cargo build
	cargo test --quiet

# step 1 of 2 in publishing a new version to crates.io
# this bumps the version in Cargo.toml and creates a new commit and tags it
version:
	@CARGO_MANIFEST_DIR=$(PWD) bash scripts/version.sh

# step 1 of 2 in publishing a new version to crates.io
# after running this step, the latest version will be available on Crates.io
publish:
	@CARGO_MANIFEST_DIR=$(PWD) bash scripts/publish.sh

# copies built binary to a path specified by $BIN
load_bin:
	@rm -f $(BIN)/git-nu
	@cp ./target/release/git-nu $(BIN)

.PHONY: test size quick load_bin bench
