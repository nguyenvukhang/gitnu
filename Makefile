PYTHON = $(shell (command -v python3))
CWD = $(shell pwd)

all_tests:
	@echo "running tests..."
	@$(PYTHON) -m unittest discover

install:
	# ln -sf $$PWD/run $$HOME/.local/bin/gitnu
	sudo ln -s /opt/homebrew/lib/python3.9/site-packages/gitnu

build:
	$(PYTHON) setup.py sdist bdist_wheel

upload:
	twine upload dist/*

clean:
	rm -rf build
	rm -rf dist
	rm -rf gitnu.egg-info
