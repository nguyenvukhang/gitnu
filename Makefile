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
	$(PYTHON) -m build --sdist

clear-site:
	# if installed by pip, this will be a directory
	# if installed by make dev, this will be a symlink to src/gitnu
	rm -rf $(SITE_PKG)/gitnu

pip-uninstall:
	make clear-site
	pip uninstall --yes gitnu

pip-install:
	pip install gitnu

fresh-pip-install:
	make pip-uninstall
	make pip-install

# derivative stuff

fresh-build:
	make clean
	make build

upload:
	make fresh-build
	twine upload --username "brew4k" dist/*

dev: # use with caution
	@pip show gitnu || pip install .
	@make clear-site
	ln -sf $$PWD/src/gitnu $(SITE_PKG)

# aliases

i:
	make pip-install

fi:
	make fresh-pip-install

u:
	make pip-uninstall

fb: 
	make fresh-build
