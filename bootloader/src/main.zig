const std = @import("std");
const efi = std.os.uefi;
const elf_loader = @import("elf_loader.zig");
const memory = @import("memory.zig");
const handoff = @import("handoff.zig");

pub fn main() !void {
    const bs = efi.systemtable.bootservices;
    const conout = efi.systemtable.con_out;
    _ = try conout.outputstring(conout, "Bootloader iniciado via Zig\n");

    // Placeholder for kernel loading and memory setup
    // This will involve:
    // 1. Locating the kernel (either embedded or from EFI partition)
    // 2. Parsing the ELF header using elf_loader
    // 3. Allocating pages and mapping segments using memory module
    // 4. Exiting boot services
    // 5. Creating and passing the Handoff structure
    // 6. Jumping to the kernel entry point
}
