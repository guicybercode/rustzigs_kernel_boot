const std = @import("std");
const efi = std.os.uefi;
const elf_loader = @import("elf_loader.zig");
const memory = @import("memory.zig");
const handoff = @import("handoff.zig");

pub fn main() !void {
    const bs = efi.systemtable.boot_services;
    const conout = efi.systemtable.con_out;
    _ = conout.output_string(conout, "UEFI Bootloader started\n");

    var memory_map: [256]efi.MemoryDescriptor = undefined;
    var memory_map_size: usize = memory_map.len * @sizeOf(efi.MemoryDescriptor);
    var map_key: usize = undefined;
    var descriptor_size: usize = undefined;
    var descriptor_version: u32 = undefined;

    const status = bs.get_memory_map(
        &memory_map_size,
        &memory_map,
        &map_key,
        &descriptor_size,
        &descriptor_version,
    );

    if (status != efi.Status.Success) {
        _ = conout.output_string(conout, "Failed to get memory map\n");
        return;
    }

    const acpi_rsdp = find_acpi_rsdp() orelse {
        _ = conout.output_string(conout, "ACPI RSDP not found\n");
        return;
    };

    const kernel_path = "kernel.elf";
    const kernel_data = load_kernel_file(kernel_path) orelse {
        _ = conout.output_string(conout, "Failed to load kernel\n");
        return;
    };

    const kernel_entry = elf_loader.load_elf(kernel_data) orelse {
        _ = conout.output_string(conout, "Failed to parse ELF\n");
        return;
    };

    const page_tables = memory.setup_paging() orelse {
        _ = conout.output_string(conout, "Failed to setup paging\n");
        return;
    };

    var handoff_data = handoff.HandoffData{
        .memory_map = memory.convert_memory_map(&memory_map, memory_map_size / descriptor_size),
        .acpi_rsdp = acpi_rsdp,
        .framebuffer = get_framebuffer_info(),
        .kernel_physical = kernel_entry.physical,
        .kernel_virtual = kernel_entry.virtual,
    };

    _ = conout.output_string(conout, "Exiting boot services...\n");
    _ = bs.exit_boot_services(efi.systemtable.image_handle, map_key);

    const kernel_start: *const fn (*handoff.HandoffData) callconv(.C) noreturn = @ptrCast(kernel_entry.virtual);
    kernel_start(&handoff_data);
}

fn find_acpi_rsdp() ?*efi.Acpi20TableHeader {
    const system_table = efi.systemtable;
    const config_table = system_table.configuration_table;
    
    for (0..system_table.number_of_table_entries) |i| {
        const entry = config_table[i];
        if (std.mem.eql(u8, &entry.vendor_guid.data, &efi.Acpi20TableGuid.data)) {
            return @ptrCast(entry.vendor_table);
        }
    }
    return null;
}

fn load_kernel_file(path: []const u8) ?[]const u8 {
    const bs = efi.systemtable.boot_services;
    const conout = efi.systemtable.con_out;
    
    var root: ?*efi.SimpleFileSystemProtocol = null;
    var status = bs.locate_protocol(&efi.SimpleFileSystemProtocol.guid, null, @ptrCast(&root));
    if (status != efi.Status.Success) {
        _ = conout.output_string(conout, "Failed to locate SimpleFileSystem\n");
        return null;
    }

    var volume: ?*efi.FileProtocol = null;
    status = root.?.open_volume(root.?, &volume);
    if (status != efi.Status.Success) {
        _ = conout.output_string(conout, "Failed to open volume\n");
        return null;
    }

    var file: ?*efi.FileProtocol = null;
    var file_path = efi.unicode_string_from_ascii(path);
    status = volume.?.open(volume.?, &file, &file_path, efi.FileMode.Read, efi.FileAttribute.ReadOnly);
    if (status != efi.Status.Success) {
        _ = conout.output_string(conout, "Failed to open kernel file\n");
        return null;
    }

    var file_info: efi.FileInfo = undefined;
    var file_info_size: usize = @sizeOf(efi.FileInfo);
    status = file.?.get_info(file.?, &efi.FileInfo.guid, &file_info_size, &file_info);
    if (status != efi.Status.Success) {
        _ = conout.output_string(conout, "Failed to get file info\n");
        return null;
    }

    const kernel_size = file_info.file_size;
    const kernel_buffer = bs.allocate_pool(efi.MemoryType.LoaderData, kernel_size) orelse {
        _ = conout.output_string(conout, "Failed to allocate kernel buffer\n");
        return null;
    };

    var bytes_read: usize = kernel_size;
    status = file.?.read(file.?, &bytes_read, kernel_buffer);
    if (status != efi.Status.Success) {
        _ = conout.output_string(conout, "Failed to read kernel file\n");
        return null;
    }

    return kernel_buffer[0..kernel_size];
}

fn get_framebuffer_info() handoff.FramebufferInfo {
    const gop = efi.systemtable.boot_services.locate_protocol(&efi.GraphicsOutputProtocol.guid, null, null);
    if (gop != efi.Status.Success) {
        return handoff.FramebufferInfo{
            .base = 0,
            .width = 0,
            .height = 0,
            .pitch = 0,
        };
    }
    
    return handoff.FramebufferInfo{
        .base = 0,
        .width = 0,
        .height = 0,
        .pitch = 0,
    };
}
