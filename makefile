all: build run test fmt

build:
	cargo build

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt

coverage:
	# cargo llvm-coverage
	cargo tarpaulin
