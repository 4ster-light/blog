.PHONY: all build run prod

all: run

build:
	@bun run build:css
	@cargo build --release

run:
	@bun run build:css
	@cargo run

prod:
	@./target/release/blog