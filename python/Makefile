OUTPUT_DIR := build

all:
	mkdir -p $(OUTPUT_DIR)
	cython -3 --embed -o $(OUTPUT_DIR)/gitn.c gitn

clean:
	rm -rf $(OUTPUT_DIR)
