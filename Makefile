LOCAL_BIN=$(HOME)/dots/personal/.local/bin
GITNU_RELEASE_BIN=$(PWD)/target/release/git-nu
GITNU_DEBUG_BIN=$(PWD)/target/debug/git-nu

PY_UTILS := python3 scripts/utils.py
ONE_TEST := parser::tests::test_date_filename

current:
	make test

install:
	make build
	make load-bin

dev:
	cargo build
	cargo test --lib

build:
	cargo build --bin git-nu --release
	@echo "Release build complete."

test:
	cargo build
	GITNU_DEBUG=0 cargo test --lib

test-one:
	cargo build
	GITNU_DEBUG=0 cargo test $(ONE_TEST)

# copies built binary to a path specified by $BIN
load-bin:
	@rm -f $(LOCAL_BIN)/git-nu
	@cp $(GITNU_RELEASE_BIN) $(LOCAL_BIN)


# ────────────────────────────────────────────────────────────────────
# MARK: - CI 

ci-git-user:
	git config --global user.name gitnu-ci
	git config --global user.email ci@gitnu.com

py:
	@$(PY_UTILS) $(ARG)

.PHONY: test load-bin
