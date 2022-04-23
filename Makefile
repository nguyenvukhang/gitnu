PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m unittest discover

local:
	# ln -sf $$PWD/run $$HOME/.local/bin/gitnu
	pip uninstall --yes gitnu
	cd dist && pip install gitnu-0.0.4.tar.gz

build:
	$(PYTHON) setup.py sdist bdist_wheel

upload:
	twine upload dist/*

clean:
	rm -rf build
	rm -rf dist
	rm -rf gitnu.egg-info

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
