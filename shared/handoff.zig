pub const MemoryMapEntry = extern struct {
    base: u64,
    length: u64,
    type: u32,
};

pub const FramebufferInfo = extern struct {
    base: u64,
    width: u32,
    height: u32,
    pitch: u32,
};

pub const HandoffData = extern struct {
    memory_map: [256]MemoryMapEntry,
    acpi_rsdp: u64,
    smbios_entry: u64,
    framebuffer: FramebufferInfo,
    kernel_physical: u64,
    kernel_virtual: u64,
};
