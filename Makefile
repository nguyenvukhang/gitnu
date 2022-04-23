PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m test.test_gitn

install:
	ln -sf $$PWD/bin/gitn $$HOME/.local/bin
