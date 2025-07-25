# Makefile for Rust UEFI UART Driver

M ?= .
DRIVER_NAME = uart_hello_driver
TARGET = aarch64-unknown-uefi
OUTPUT_DIR = output
EDK2_OUTPUT_DIR = Platform/RaspberryPi/RPi5/Drivers/UartHelloDriver

ifeq ($(M), .)
	SRC_DIR = .
else
	SRC_DIR = $(M)
	DRIVER_NAME = $(SRC_DIR)
endif

# Default target
.PHONY: all
all: $(OUTPUT_DIR)/$(DRIVER_NAME).efi

# Build the driver
$(OUTPUT_DIR)/$(DRIVER_NAME).efi: $(SRC_DIR)/src/main.rs $(SRC_DIR)/Cargo.toml .cargo/config.toml
	@echo "Building Rust UEFI driver..."
	@rustup target add $(TARGET) 2>/dev/null || true
	cd $(SRC_DIR) && cargo build --target $(TARGET) --release
	@mkdir -p $(EDK2_OUTPUT_DIR)
	@cp $(SRC_DIR)/target/$(TARGET)/debug/$(DRIVER_NAME).efi $(EDK2_OUTPUT_DIR)/ 2>/dev/null || true
	@echo "✅ Build complete"

# Clean build artifacts
.PHONY: clean
clean:
	@if [ "$(MODULE)" = "." ]; then \
		cargo clean; \
	else \
		cd $(MODULE) && cargo clean; \
	fi
	rm -rf $(OUTPUT_DIR)
	rm -f $(EDK2_OUTPUT_DIR)/$(DRIVER_NAME).efi

# Debug build
.PHONY: build
build:
	@echo "Building debug from $(SRC_DIR)..."
	@rustup target add $(TARGET) 2>/dev/null || true
	cd $(SRC_DIR) && cargo build --target $(TARGET)
	mkdir -p $(OUTPUT_DIR)
	cp $(SRC_DIR)/target/$(TARGET)/debug/$(DRIVER_NAME).efi $(OUTPUT_DIR)/$(SRC_DIR).efi
	@echo "✅ Build complete"

# Check if required tools are installed
.PHONY: check
check:
	@echo "Checking build environment..."
	@which rustc >/dev/null || (echo "❌ Rust not installed" && exit 1)
	@which cargo >/dev/null || (echo "❌ Cargo not installed" && exit 1)
	@rustup target list --installed | grep -q $(TARGET) || (echo "⚠️  Target $(TARGET) not installed, will install automatically" && rustup target add $(TARGET))
	@echo "✅ Build environment OK"

# Show build info
.PHONY: info
info:
	@echo "Driver Name: $(DRIVER_NAME)"
	@echo "Target: $(TARGET)"
	@echo "Output Directory: $(OUTPUT_DIR)"
	@echo "EDK2 Output: $(EDK2_OUTPUT_DIR)"
	@echo ""
	@echo "Usage:"
	@echo "  make         - Build release version"
	@echo "  make debug   - Build debug version"
	@echo "  make clean   - Clean build artifacts"
	@echo "  make check   - Check build environment"

# Help target
.PHONY: help
help: info