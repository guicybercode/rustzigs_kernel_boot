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
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check Zig
    if ! command -v zig &> /dev/null; then
        print_error "Zig is not installed. Please install Zig 0.11.0 or later."
        exit 1
    fi
    
    # Check Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust is not installed. Please install Rust 1.70.0 or later."
        exit 1
    fi
    
    # Check QEMU
    if ! command -v qemu-system-x86_64 &> /dev/null; then
        print_warning "QEMU not found. You won't be able to run the system."
    fi
    
    print_success "Prerequisites check completed"
}

# Build bootloader
build_bootloader() {
    print_status "Building UEFI bootloader..."
    
    cd bootloader
    if zig build; then
        print_success "Bootloader built successfully"
    else
        print_error "Failed to build bootloader"
        exit 1
    fi
    cd ..
}

# Build kernel
build_kernel() {
    print_status "Building Rust kernel..."
    
    cd kernel
    if cargo build --release; then
        print_success "Kernel built successfully"
    else
        print_error "Failed to build kernel"
        exit 1
    fi
    cd ..
}

# Create bootable image
create_image() {
    print_status "Creating bootable EFI image..."
    
    # Create EFI partition directory
    mkdir -p efi-partition/EFI/BOOT
    
    # Copy bootloader
    if [ -f "bootloader/zig-out/bin/bootloader.efi" ]; then
        cp bootloader/zig-out/bin/bootloader.efi efi-partition/EFI/BOOT/BOOTX64.EFI
        print_success "Bootloader copied to EFI partition"
    else
        print_error "Bootloader EFI binary not found"
        exit 1
    fi
    
    # Copy kernel (if available)
    if [ -f "kernel/target/x86_64-unknown-none/release/libkernel.a" ]; then
        cp kernel/target/x86_64-unknown-none/release/libkernel.a efi-partition/kernel.a
        print_success "Kernel library copied to EFI partition"
    else
        print_warning "Kernel library not found"
    fi
    
    print_success "Bootable image created in efi-partition/"
}

# Run with QEMU
run_qemu() {
    print_status "Starting QEMU..."
    
    if command -v qemu-system-x86_64 &> /dev/null; then
        qemu-system-x86_64 \
            -bios /usr/share/ovmf/OVMF.fd \
            -drive format=raw,file=fat:rw:./efi-partition \
            -m 512M \
            -serial stdio \
            -vga std
    else
        print_error "QEMU not found. Please install QEMU to run the system."
        exit 1
    fi
}

# Main execution
main() {
    case "${1:-all}" in
        "bootloader")
            check_prerequisites
            build_bootloader
            ;;
        "kernel")
            check_prerequisites
            build_kernel
            ;;
        "image")
            check_prerequisites
            build_bootloader
            build_kernel
            create_image
            ;;
        "run")
            check_prerequisites
            build_bootloader
            build_kernel
            create_image
            run_qemu
            ;;
        "clean")
            print_status "Cleaning build artifacts..."
            cd bootloader && zig build clean && cd ..
            cd kernel && cargo clean && cd ..
            rm -rf efi-partition
            print_success "Clean completed"
            ;;
        "all"|"")
            check_prerequisites
            build_bootloader
            build_kernel
            create_image
            print_success "Build completed successfully!"
            print_status "To run the system: ./build.sh run"
            ;;
        "help")
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  all         - Build everything (default)"
            echo "  bootloader  - Build bootloader only"
            echo "  kernel      - Build kernel only"
            echo "  image       - Build and create bootable image"
            echo "  run         - Build and run with QEMU"
            echo "  clean       - Clean build artifacts"
            echo "  help        - Show this help"
            ;;
        *)
            print_error "Unknown command: $1"
            echo "Use '$0 help' for usage information"
            exit 1
            ;;
    esac
}

main "$@"
