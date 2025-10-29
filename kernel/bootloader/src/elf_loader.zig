const std = @import("std");
const efi = std.os.uefi;

pub const KernelEntry = struct {
    physical: u64,
    virtual: u64,
};

const Elf64Header = extern struct {
    magic: u32,
    class: u8,
    data: u8,
    version: u8,
    os_abi: u8,
    abi_version: u8,
    padding: [7]u8,
    type: u16,
    machine: u16,
    version: u32,
    entry: u64,
    phoff: u64,
    shoff: u64,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
};

const Elf64ProgramHeader = extern struct {
    type: u32,
    flags: u32,
    offset: u64,
    vaddr: u64,
    paddr: u64,
    filesz: u64,
    memsz: u64,
    align: u64,
};

const ELF_MAGIC = 0x464C457F;
const PT_LOAD = 1;
const PF_X = 0x1;
const PF_W = 0x2;
const PF_R = 0x4;

pub fn load_elf(data: []const u8) ?KernelEntry {
    if (data.len < @sizeOf(Elf64Header)) {
        return null;
    }

    const header = @ptrCast(*const Elf64Header, data.ptr);
    
    if (header.magic != ELF_MAGIC) {
        return null;
    }

    if (header.class != 2 or header.data != 1) {
        return null;
    }

    if (header.machine != 0x3E) {
        return null;
    }

    const bs = efi.systemtable.boot_services;
    var kernel_physical: u64 = 0;
    var kernel_virtual: u64 = 0;

    for (0..header.phnum) |i| {
        const ph_offset = header.phoff + i * header.phentsize;
        if (ph_offset + @sizeOf(Elf64ProgramHeader) > data.len) {
            return null;
        }

        const ph = @ptrCast(*const Elf64ProgramHeader, data.ptr + ph_offset);
        
        if (ph.type != PT_LOAD) {
            continue;
        }

        const pages = (ph.memsz + 0xFFF) / 0x1000;
        const physical_addr = bs.allocate_pages(efi.AllocateType.AllocateAnyPages, efi.MemoryType.LoaderCode, pages) orelse {
            return null;
        };

        if (i == 0) {
            kernel_physical = physical_addr;
            kernel_virtual = ph.vaddr;
        }

        const segment_data = data[ph.offset..ph.offset + ph.filesz];
        @memcpy(@ptrCast([*]u8, @as(*anyopaque, @ptrFromInt(physical_addr)))[0..segment_data.len], segment_data);

        if (ph.memsz > ph.filesz) {
            const zero_start = physical_addr + ph.filesz;
            const zero_size = ph.memsz - ph.filesz;
            @memset(@ptrCast([*]u8, @as(*anyopaque, @ptrFromInt(zero_start)))[0..zero_size], 0);
        }
    }

    return KernelEntry{
        .physical = kernel_physical,
        .virtual = kernel_virtual,
    };
}
