PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m unittest discover

install:
	ln -sf $$PWD/run $$HOME/.local/bin/gitnu

build:
	$(PYTHON) setup.py sdist bdist_wheel

upload:
	twine upload dist/*
