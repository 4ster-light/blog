.PHONY: all build run

all: run

build:
	@bun i
	@bun run build:css
	@cargo build --release

run:
	@bun run build:css
	@cargo run
