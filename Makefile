.PHONY: install build run test check update help

install:
	cargo install --path .

build:
	cargo build --release

# Run with default config
run:
	cargo run -- .

# Run with custom config
run-config:
	cargo run -- -c config.toml

# Run recursive with verbose
run-recursive:
	cargo run -- -c config.toml -v -r

# Run preview mode (don't actually rename)
run-preview:
	cargo run -- -c config.toml -f -v -r

test:
	cargo test

check:
	cargo check

update:
	cargo update

clean:
	cargo clean

help:
	@echo "Available targets:"
	@echo "  make install      - Install the application"
	@echo "  make build        - Build the application"
	@echo "  make run          - Run with default config"
	@echo "  make run-config   - Run with config.toml"
	@echo "  make run-recursive - Run recursive with verbose"
	@echo "  make run-preview  - Preview mode (dry-run)"
	@echo "  make test         - Run tests"
	@echo "  make check        - Check code without building"
	@echo "  make update       - Update dependencies"
	@echo "  make clean        - Clean build artifacts"
