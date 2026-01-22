NAME = glint
CARGO = cargo
PREFIX = /usr/local
BIN_DIR = $(PREFIX)/bin

.PHONY: all build run clean check fmt help install uninstall

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

install: release
	@echo "Installing $(NAME) to $(BIN_DIR)..."
	@sudo install -Dm755 target/release/$(NAME) $(BIN_DIR)/$(NAME)
	@echo "Installation complete."

uninstall:
	@echo "Removing $(NAME) from $(INSTALL_DIR)..."
	sudo $(RM) $(BIN_DIR)/$(NAME)

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
	@echo "  make build     - Build the project (debug)"
	@echo "  make run       - Build and run (default)"
	@echo "  make release   - Build optimized release binary"
	@echo "  make install   - Build release and install to $(PREFIX)/bin"
	@echo "  make uninstall - Remove the binary from $(PREFIX)/bin"
	@echo "  make clean     - Remove build artifacts"
	@echo "  make check     - Run fast syntax check"
	@echo "  make fmt       - Format Rust source code"
