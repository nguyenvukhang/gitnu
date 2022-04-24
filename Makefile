PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m unittest discover

# modular core stuff

clean:
	rm -rf build dist src/gitnu.egg-info

build:
	$(PYTHON) -m build

uninstall-pip:
	pip uninstall --yes gitnu

install-pip:
	make clear-cache
	pip install gitnu

install-local:
	pip install .

# derivative stuff

fresh-build:
	make clean
	make build

upload:
	make fresh-build
	twine upload --username "brew4k" dist/*

dev:
	make uninstall-pip
	make install-local

# aliases

i:
	make install-pip

u:
	make uninstall-pip
