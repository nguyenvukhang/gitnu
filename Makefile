LOCAL_BIN=$(HOME)/.local/bin
GITNU_RELEASE_BIN=$(PWD)/target/release/git-nu
GITNU_DEBUG_BIN=$(PWD)/target/debug/git-nu

ONE_TEST := 'tests::add_and_status_diff_dirs'

current: test

install:
	make build
	make load-bin

build:
	cargo build --bin git-nu --release
	@echo "Release build complete."

test:
	cargo build
	cargo test

test-one:
	cargo build
	cargo test $(ONE_TEST)

# copies built binary to a path specified by $BIN
load-bin:
	@rm -f $(LOCAL_BIN)/git-nu
	@cp $(GITNU_RELEASE_BIN) $(LOCAL_BIN)

.PHONY: test test-one load-bin
