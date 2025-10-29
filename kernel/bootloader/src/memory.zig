const std = @import("std");
const efi = std.os.uefi;
const handoff = @import("handoff.zig");

pub const PageTable = extern struct {
    entries: [512]u64,
};

const PAGE_PRESENT = 1 << 0;
const PAGE_WRITABLE = 1 << 1;
const PAGE_USER = 1 << 2;
const PAGE_SIZE = 1 << 7;
const PAGE_NX = 1 << 63;

pub fn setup_paging() ?*PageTable {
    const bs = efi.systemtable.boot_services;
    
    const pml4 = bs.allocate_pages(efi.AllocateType.AllocateAnyPages, efi.MemoryType.LoaderData, 1) orelse {
        return null;
    };
    
    const pml4_ptr = @ptrCast(*PageTable, @as(*anyopaque, @ptrFromInt(pml4)));
    @memset(@ptrCast([*]u8, pml4_ptr)[0..@sizeOf(PageTable)], 0);

    const pdpt = bs.allocate_pages(efi.AllocateType.AllocateAnyPages, efi.MemoryType.LoaderData, 1) orelse {
        return null;
    };
    
    const pdpt_ptr = @ptrCast(*PageTable, @as(*anyopaque, @ptrFromInt(pdpt)));
    @memset(@ptrCast([*]u8, pdpt_ptr)[0..@sizeOf(PageTable)], 0);

    pml4_ptr.entries[0] = pdpt | PAGE_PRESENT | PAGE_WRITABLE;

    for (0..4) |i| {
        const pd = bs.allocate_pages(efi.AllocateType.AllocateAnyPages, efi.MemoryType.LoaderData, 1) orelse {
            return null;
        };
        
        const pd_ptr = @ptrCast(*PageTable, @as(*anyopaque, @ptrFromInt(pd)));
        @memset(@ptrCast([*]u8, pd_ptr)[0..@sizeOf(PageTable)], 0);

        pdpt_ptr.entries[i] = pd | PAGE_PRESENT | PAGE_WRITABLE;

        for (0..512) |j| {
            const paddr = (i * 512 + j) * 0x200000;
            pd_ptr.entries[j] = paddr | PAGE_PRESENT | PAGE_WRITABLE | PAGE_SIZE;
        }
    }

    enable_paging(pml4);
    return pml4_ptr;
}

fn enable_paging(pml4: u64) void {
    asm volatile (
        \\mov %[pml4], %%cr3
        \\mov %%cr4, %%rax
        \\or $0x20, %%rax
        \\mov %%rax, %%cr4
        \\mov $0xC0000080, %%ecx
        \\rdmsr
        \\or $0x100, %%eax
        \\wrmsr
        \\mov %%cr0, %%rax
        \\or $0x80000000, %%rax
        \\mov %%rax, %%cr0
        :
        : [pml4] "r" (pml4)
        : "rax", "rcx"
    );
}

pub fn convert_memory_map(efi_map: []const efi.MemoryDescriptor, count: usize) [256]handoff.MemoryMapEntry {
    var map: [256]handoff.MemoryMapEntry = undefined;
    @memset(@ptrCast([*]u8, &map)[0..@sizeOf(@TypeOf(map))], 0);

    const max_entries = @min(count, map.len);
    for (0..max_entries) |i| {
        const desc = efi_map[i];
        map[i] = handoff.MemoryMapEntry{
            .base = desc.physical_start,
            .length = desc.number_of_pages * 0x1000,
            .type = @intCast(desc.type),
        };
    }

    return map;
}
