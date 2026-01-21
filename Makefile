NAME = glint
CARGO = cargo

.PHONY: all build run clean check fmt help

all: run

build:
	@echo "Building $(NAME)..."
	$(CARGO) build

run:
	@echo "Starting $(NAME)..."
	$(CARGO) run

release:
	@echo "Building release version..."
	$(CARGO) build --release

clean:
	@echo "Cleaning target directory..."
	$(CARGO) clean

check:
	@echo "Checking syntax..."
	$(CARGO) check

fmt:
	@echo "Formatting code..."
	$(CARGO) fmt

help:
	@echo "Glint Management Commands:"
	@echo "  make build   - Build the project"
	@echo "  make run     - Build and run (default)"
	@echo "  make release - Build optimized release binary"
	@echo "  make clean   - Remove build artifacts"
	@echo "  make check   - Run fast syntax check"
	@echo "  make fmt     - Format Rust source code"
