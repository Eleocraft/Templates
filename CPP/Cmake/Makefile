BUILD_DIR := build
NPROC := $(shell nproc)

.PHONY: all configure build clean

all: build

configure:
	mkdir -p $(BUILD_DIR)
	cmake -S . -B $(BUILD_DIR) -G Ninja

build: configure
	cmake --build $(BUILD_DIR) -j $(NPROC)

clean:
	rm -rf $(BUILD_DIR)
