PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)
SITE_PKG=/opt/homebrew/lib/python3.9/site-packages

# MODULAR CORE STUFF

FORCE:

clean:
	rm -rf build dist src/gitnu.egg-info .pytest_cache src/gitnu/__pycache__ test/__pycache__

clear-cache:
	rm -rf ~/Library/Caches/pip

build-full:
	make clean
	$(PYTHON) -m black src test
	$(PYTHON) -m build

build: FORCE
	make build-full

clear-site-packages:
	# if installed by pip, this will be a directory
	# if installed by make dev, this will be a symlink to src/gitnu
	rm -rf $(SITE_PKG)/gitnu

# PIP STUFF

pip-uninstall:
	make clear-site-packages
	pip uninstall --yes gitnu

pip-install:
	pip install gitnu

local-install:
	make build
	cd dist && pip install *.tar.gz

fresh-pip-install:
	make pip-uninstall
	make pip-install

unpack:
	cd dist && tar -xvf *.tar.gz

lint:
	pylint src/gitnu --disable=missing-class-docstring --disable=missing-function-docstring

# derivative stuff

send-to-pypi:
	twine upload --username "brew4k" dist/*

upload:
	make clean
	make build-full
	make send-to-pypi

dev: # use with caution
	@pip show gitnu || pip install .
	make clear-site-packages
	ln -sf $$PWD/src/gitnu $(SITE_PKG)

# aliases

i:
	make pip-install

fi:
	make fresh-pip-install

u:
	make pip-uninstall

test: FORCE
	@echo "Running all tests..."
	$(PYTHON) -m unittest discover

pytest:
	pytest

pt:
	pytest

test-shell:
	pytest test/test_shell.py

ts:
	make test-shell
