LOCAL_BIN=$(HOME)/dots/personal/.local/bin
GITNU_RELEASE_BIN=$(PWD)/target/release/git-nu
GITNU_DEBUG_BIN=$(PWD)/target/debug/git-nu

PY_UTILS := python3 scripts/utils.py

current:
	make ci-test

install:
	make build
	make load-bin

dev:
	cargo build
	cargo test --lib

build:
	cargo build --release
	@echo "Release build complete."

test:
	cargo build
	cargo test --lib

# copies built binary to a path specified by $BIN
load-bin:
	@rm -f $(LOCAL_BIN)/git-nu
	@cp $(GITNU_RELEASE_BIN) $(LOCAL_BIN)


# ────────────────────────────────────────────────────────────────────
# MARK: - CI 

ci-git-user:
	git config --global user.name gitnu-ci
	git config --global user.email ci@gitnu.com

ci-increment-prerelease:
	$(PY_UTILS) increment-prerelease

current-version:
	@$(PY_UTILS) current-version

next-prerelease-version:
	@$(PY_UTILS) next-prerelease-version

ci-build:
	cargo build --bin git-nu --release

ci-test:
	cargo build
	cargo test --lib

.PHONY: test load-bin
