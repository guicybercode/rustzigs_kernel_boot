#!/bin/bash

# Advanced UEFI Bootloader with Rust Kernel Build Script

set -e

echo "=========================================="
echo "🚀 ULTRA-ADVANCED UEFI BOOTLOADER WITH RUST KERNEL 🚀"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}[BUILD]${NC} $1"
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_header "Checking prerequisites..."
    
    if ! command_exists zig; then
        print_error "Zig is not installed. Please install Zig 0.11.0+"
        exit 1
    fi
    
    if ! command_exists cargo; then
        print_error "Rust/Cargo is not installed. Please install Rust 1.70.0+"
        exit 1
    fi
    
    if ! command_exists qemu-system-x86_64; then
        print_warning "QEMU is not installed. You won't be able to run the system."
    fi
    
    print_status "Prerequisites check completed"
}

# Build bootloader
build_bootloader() {
    print_header "Building UEFI bootloader..."
    cd bootloader
    zig build
    print_status "Bootloader built successfully"
    cd ..
}

# Build kernel
build_kernel() {
    print_header "Building Rust kernel..."
    cd kernel
    cargo build --release
    print_status "Kernel built successfully"
    cd ..
}

# Build everything
build_all() {
    print_header "Building entire system..."
    check_prerequisites
    build_bootloader
    build_kernel
    print_status "Build completed successfully!"
}

# Clean build artifacts
clean() {
    print_header "Cleaning build artifacts..."
    cd bootloader && zig build clean && cd ..
    cd kernel && cargo clean && cd ..
    print_status "Clean completed"
}

# Run with QEMU
run() {
    print_header "Running system with QEMU..."
    if ! command_exists qemu-system-x86_64; then
        print_error "QEMU is not installed. Cannot run the system."
        exit 1
    fi
    
    # Create EFI partition if it doesn't exist
    mkdir -p efi-partition/EFI/BOOT
    
    # Copy bootloader to EFI partition
    if [ -f "bootloader/zig-out/bin/bootloader.efi" ]; then
        cp bootloader/zig-out/bin/bootloader.efi efi-partition/EFI/BOOT/BOOTX64.EFI
        print_status "Bootloader copied to EFI partition"
    else
        print_error "Bootloader not found. Please build first."
        exit 1
    fi
    
    # Run QEMU
    qemu-system-x86_64 \
        -bios /usr/share/ovmf/OVMF.fd \
        -drive format=raw,file=fat:rw:./efi-partition \
        -m 512M \
        -smp 2 \
        -serial stdio \
        -vga std
}

# Debug with GDB
debug() {
    print_header "Starting debug session..."
    if ! command_exists qemu-system-x86_64; then
        print_error "QEMU is not installed. Cannot run debug session."
        exit 1
    fi
    
    if ! command_exists gdb; then
        print_error "GDB is not installed. Cannot start debug session."
        exit 1
    fi
    
    # Start QEMU in background with GDB server
    qemu-system-x86_64 \
        -bios /usr/share/ovmf/OVMF.fd \
        -drive format=raw,file=fat:rw:./efi-partition \
        -m 512M \
        -smp 2 \
        -serial stdio \
        -s -S \
        -gdb tcp::1234 &
    
    QEMU_PID=$!
    print_status "QEMU started with PID $QEMU_PID"
    print_status "Connect GDB to localhost:1234"
    
    # Start GDB
    gdb -ex "target remote localhost:1234" -ex "continue"
    
    # Cleanup
    kill $QEMU_PID 2>/dev/null || true
}

# Create bootable image
create_image() {
    print_header "Creating bootable image..."
    
    # Create EFI partition
    mkdir -p efi-partition/EFI/BOOT
    
    # Copy bootloader
    if [ -f "bootloader/zig-out/bin/bootloader.efi" ]; then
        cp bootloader/zig-out/bin/bootloader.efi efi-partition/EFI/BOOT/BOOTX64.EFI
        print_status "Bootable image created successfully"
    else
        print_error "Bootloader not found. Please build first."
        exit 1
    fi
}

# Test system
test() {
    print_header "Running tests..."
    cd kernel
    cargo test --release
    print_status "Tests completed"
    cd ..
}

# Show help
show_help() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  all        Build everything (default)"
    echo "  bootloader Build only the bootloader"
    echo "  kernel     Build only the kernel"
    echo "  clean      Clean build artifacts"
    echo "  run        Run with QEMU"
    echo "  debug      Debug with GDB"
    echo "  image      Create bootable image"
    echo "  test       Run tests"
    echo "  help       Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 all     # Build everything"
    echo "  $0 run     # Build and run with QEMU"
    echo "  $0 debug   # Build and debug with GDB"
}

# Main script logic
case "${1:-all}" in
    "all")
        build_all
        ;;
    "bootloader")
        check_prerequisites
        build_bootloader
        ;;
    "kernel")
        check_prerequisites
        build_kernel
        ;;
    "clean")
        clean
        ;;
    "run")
        build_all
        run
        ;;
    "debug")
        build_all
        debug
        ;;
    "image")
        build_all
        create_image
        ;;
    "test")
        test
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        show_help
        exit 1
        ;;
esac