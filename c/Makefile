# G++ ?= $(shell (command -v /opt/homebrew/bin/g++-11 || echo /opt/homebrew/bin/g++-11 ))
# G++ = clang++
G++ = g++

default:
	make build
	make run

all:
	make build
	make run
	make cache

run:
	@[ -d repo ] && cd repo/some/thing && ../../../dist/cgit

build:
	@mkdir -p dist
	$(G++) -std=c++17 main.cpp -c -o dist/main.o
	# $(G++) -std=c++17 shell.cpp -c -o dist/shell.o
	# $(G++) -std=c++17 entry.cpp -c -o dist/entry.o
	# $(G++) -std=c++17 shelf.cpp -c -o dist/shelf.o
	$(G++) -std=c++17 git.cpp -c -o dist/git.o
	$(G++) -std=c++17 dist/main.o \
		dist/git.o -o \
		dist/cgit
	rm dist/*.o

clean:
	rm -rf dist

check:
	git -C repo/some/thing status --porcelain

status:
	git -C repo/some/thing status

cache:
	nvim repo/.git/gitnu.txt

bench-all:
	cd repo && python3 ../benchmark.py

bench: FORCE
	node bench.js
	$$EDITOR time.json

time:
	@make bench

test:
	./harsh
	nvim harsh.log

FORCE:
