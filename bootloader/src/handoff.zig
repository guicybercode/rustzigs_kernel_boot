const std = @import("std");
const efi = std.os.uefi;

pub const Handoff = extern struct {
    memory_map_addr: u64,
    memory_map_size: u64,
    memory_map_desc_size: u64,
    rsdp_addr: u64,
    smbios_addr: u64,
    framebuffer_addr: u64,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_stride: u32,
    // Add other necessary fields for ACPI, SMBIOS, etc.
};
