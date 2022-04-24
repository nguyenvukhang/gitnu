PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m unittest discover

clean:
	rm -rf build
	rm -rf dist
	rm -rf gitnu.egg-info

uninstall:
	make clear-cache
	pip uninstall --yes gitnu
	rm -f ~/.local/bin/gitnu

clear-cache:
	rm -rf ~/Library/Caches/pip/wheels

install-pip:
	make clear-cache
	pip install gitnu

i:
	make install-pip

u:
	make uninstall

symlink:
	ln -sf $$PWD/run $$HOME/.local/bin/gitnu

s:
	make symlink

local:
	cd dist && pip install gitnu*

build:
	make clean
	$(PYTHON) -m build

upload:
	make clean
	make build
	twine upload --username "brew4k" dist/*
	make clean

bin:
	echo "#!/usr/bin/env bash">bin_file
	echo 'python3 -m gitnu "$$@"'>>bin_file
	chmod +x bin_file
	mv bin_file $$HOME/.local/bin/gitnu

dev:
	make clean
	make build
	make local
	make bin
	make clean
