PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m unittest discover

install:
	# ln -sf $$PWD/run $$HOME/.local/bin/gitnu
	sudo ln -s /opt/homebrew/lib/python3.9/site-packages/gitnu

local:
	# ln -sf $$PWD/run $$HOME/.local/bin/gitnu
	pip uninstall gitnu
	cd dist && pip install gitnu-0.0.4.tar.gz

build:
	$(PYTHON) setup.py sdist bdist_wheel

upload:
	twine upload dist/*

clean:
	rm -rf build
	rm -rf dist
	rm -rf gitnu.egg-info
