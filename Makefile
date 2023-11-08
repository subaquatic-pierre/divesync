# Makefile

.PHONY: all clean core cli ffi api

all: core cli ffi api

core:
	@echo "Building core..."
	@cd core && cargo build --release

cli:
	@echo "Building cli..."
	@cd cli && cargo build --release

ffi:
	@echo "Building ffi..."
	@cd ffi && cargo build --release

api:
	@echo "Building api..."
	@cd api && cargo build --release

clean:
	@echo "Cleaning up..."
	@cd core && cargo clean
	@cd cli && cargo clean
	@cd ffi && cargo clean
	@cd api && cargo clean

install: all
	@echo "Moving binaries to bin directory..."
	@mkdir -p bin
	@cp target/release/divesync-api bin/
	@cp target/release/divesync-cli bin/
	@echo "Binaries moved to bin directory."

test_all:
	@for crate in $(CRATES); do \
		echo "Running tests for $$crate..."; \
		cd $$crate && cargo test; \
	done

test_api:
	@echo "Running tests for api..."
	@cd api && cargo test

test_core:
	@echo "Running tests for core..."
	@cd core && cargo test -- --nocapture

test_ffi:
	@echo "Running tests for ffi..."
	@cd ffi && cargo test

test_cli:
	@echo "Running tests for cli..."
	@cd cli && cargo test
