NAME = glint
CARGO = cargo

ifeq ($(OS), Windows_NT)
    EXE_EXT = .exe
    RM = del /Q /F
    INSTALL_DIR = $(USERPROFILE)\AppData\Local\Microsoft\WindowsApps
    CP = copy /Y
    MKDIR_CMD = if not exist "$(INSTALL_DIR)" mkdir "$(INSTALL_DIR)"
    FIX_P = $(subst /,\,$(1))
else
    EXE_EXT =
    RM = rm -f
    INSTALL_DIR = /usr/local/bin
    CP = sudo install -Dm755
    MKDIR_CMD = sudo mkdir -p "$(INSTALL_DIR)"
    FIX_P = $(1)
endif

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
	@echo "Installing $(NAME) to $(INSTALL_DIR)..."
	@$(MKDIR_CMD)
	$(CP) $(call FIX_P,target/release/$(NAME)$(EXE_EXT)) $(call FIX_P,$(INSTALL_DIR)/$(NAME)$(EXE_EXT))
	@echo "Installation complete."

uninstall:
	@echo "Removing $(NAME) from $(INSTALL_DIR)..."
	ifeq ($(OS), Windows_NT)
		$(RM) "$(INSTALL_DIR)\$(NAME)$(EXE_EXT)"
	else
		sudo $(RM) $(INSTALL_DIR)/$(NAME)
	endif

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
