PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)
SITE_PKG=/opt/homebrew/lib/python3.9/site-packages

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m unittest discover

# modular core stuff

clean:
	rm -rf build dist src/gitnu.egg-info

build:
	$(PYTHON) -m build

uninstall-pip:
	rm -f $(SITE_PKG)/gitnu
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
	@pip show gitnu && ln -sf $$PWD/src/gitnu $(SITE_PKG)

# aliases

i:
	make install-pip

u:
	make uninstall-pip
