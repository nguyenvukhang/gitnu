# G++ ?= $(shell (command -v /opt/homebrew/bin/g++-11 || echo /opt/homebrew/bin/g++-11 ))
# G++ = clang++
G++ = g++

run:
	make build
	@echo "────────────────────────────────────────────────"
	@./cgit

build:
	@mkdir dist
	$(G++) -std=c++17 main.cpp -o dist/cgit

clean:
	rm -rf dist
