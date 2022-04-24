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

# derivative stuff

fresh-build:
	make clean
	make build

upload:
	make fresh-build
	twine upload --username "brew4k" dist/*

dev: # use with caution
	@pip show gitnu || pip install .
	@pip show gitnu && ln -sf $$PWD/src/gitnu /opt/homebrew/lib/python3.9/site-packages

# aliases

i:
	make install-pip

u:
	make uninstall-pip
