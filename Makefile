.PHONY: all build run test clean

all: build

build:
	cargo build --release

run:
	cargo run --release

test:
	cargo test

clean:
	cargo clean
