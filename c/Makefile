# G++ ?= $(shell (command -v /opt/homebrew/bin/g++-11 || echo /opt/homebrew/bin/g++-11 ))
# G++ = clang++
G++ = g++

run:
	make build
	@echo "────────────────────────────────────────────────"
	@./dist/cgit

build:
	@mkdir -p dist
	$(G++) -std=c++17 main.cpp -o dist/cgit

clean:
	rm -rf dist
