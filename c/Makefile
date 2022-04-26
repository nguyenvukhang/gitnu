# G++ ?= $(shell (command -v /opt/homebrew/bin/g++-11 || echo /opt/homebrew/bin/g++-11 ))
# G++ = clang++
G++ = g++

build:
	$(G++) -std=c++17 main.cpp -o cgit

run:
	make build
	./cgit
