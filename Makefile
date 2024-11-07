.PHONY: all build run

all: run

build:
	@bun run build:css
	@cargo build --release

run:
	@bun run build:css
	@cargo run
