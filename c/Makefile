# G++ ?= $(shell (command -v /opt/homebrew/bin/g++-11 || echo /opt/homebrew/bin/g++-11 ))
# G++ = clang++
G++ = g++

run:
	make build
	@echo "────────────────────────────────────────────────"
	@[ -d repo ] && cd repo/some/thing && ../../../dist/cgit

build:
	@mkdir -p dist
	$(G++) -std=c++17 main.cpp  -c -o dist/main.o
	$(G++) -std=c++17 shell.cpp -c -o dist/shell.o
	$(G++) -std=c++17 entry.cpp -c -o dist/entry.o
	$(G++) -std=c++17 dist/main.o dist/shell.o dist/entry.o -o dist/cgit
	rm dist/*.o

clean:
	rm -rf dist

cache:
	nvim repo/.git/gitnu.txt

all:
	make run
	make cache

bench-all:
	cd repo && python3 ../benchmark.py

bench: FORCE
	./cbench
	git rev-parse --verify HEAD >> time.txt

FORCE:
