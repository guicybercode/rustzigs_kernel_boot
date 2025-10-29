# Advanced UEFI Bootloader with Rust Kernel Makefile

.PHONY: all clean bootloader kernel test run help

# Default target
all: bootloader kernel

# Build bootloader
bootloader:
	@echo "Building UEFI bootloader..."
	cd bootloader && zig build
	@echo "Bootloader built successfully"

# Build kernel
kernel:
	@echo "Building Rust kernel..."
	cd kernel && cargo build --release
	@echo "Kernel built successfully"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cd bootloader && zig build clean
	cd kernel && cargo clean
	@echo "Clean completed"

# Test the build
test: all
	@echo "Testing build..."
	@if [ -f "bootloader/zig-out/bin/bootloader.efi" ]; then \
		echo "✓ Bootloader EFI binary found"; \
	else \
		echo "✗ Bootloader EFI binary not found"; \
		exit 1; \
	fi
	@if [ -f "kernel/target/x86_64-unknown-none/release/libkernel.a" ]; then \
		echo "✓ Kernel library found"; \
	else \
		echo "✗ Kernel library not found"; \
		exit 1; \
	fi
	@echo "✓ All tests passed"

# Create bootable image
image: all
	@echo "Creating bootable image..."
	@mkdir -p efi-partition/EFI/BOOT
	cp bootloader/zig-out/bin/bootloader.efi efi-partition/EFI/BOOT/BOOTX64.EFI
	@echo "Bootable image created in efi-partition/"

# Run with QEMU
run: image
	@echo "Running with QEMU..."
	qemu-system-x86_64 \
		-bios /usr/share/ovmf/OVMF.fd \
		-drive format=raw,file=fat:rw:./efi-partition \
		-m 512M \
		-serial stdio \
		-vga std

# Run with QEMU (no graphics)
run-headless: image
	@echo "Running with QEMU (headless)..."
	qemu-system-x86_64 \
		-bios /usr/share/ovmf/OVMF.fd \
		-drive format=raw,file=fat:rw:./efi-partition \
		-m 512M \
		-serial stdio \
		-nographic

# Debug with GDB
debug: image
	@echo "Starting debug session..."
	qemu-system-x86_64 \
		-bios /usr/share/ovmf/OVMF.fd \
		-drive format=raw,file=fat:rw:./efi-partition \
		-m 512M \
		-serial stdio \
		-s -S \
		-gdb tcp::1234

# Show help
help:
	@echo "Advanced UEFI Bootloader with Rust Kernel"
	@echo ""
	@echo "Available targets:"
	@echo "  all          - Build both bootloader and kernel"
	@echo "  bootloader   - Build UEFI bootloader only"
	@echo "  kernel       - Build Rust kernel only"
	@echo "  clean        - Clean all build artifacts"
	@echo "  test         - Test the build"
	@echo "  image        - Create bootable EFI image"
	@echo "  run          - Run with QEMU (graphics)"
	@echo "  run-headless - Run with QEMU (no graphics)"
	@echo "  debug        - Run with GDB debugging"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "Prerequisites:"
	@echo "  - Zig 0.11.0+"
	@echo "  - Rust 1.70.0+"
	@echo "  - QEMU with OVMF"
	@echo "  - UEFI development tools"
