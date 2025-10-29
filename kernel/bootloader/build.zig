const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = std.zig.CrossTarget{
        .cpu_arch = .x86_64,
        .os_tag = .uefi,
    };
    
    const mode = b.standardReleaseOptions();
    const exe = b.addExecutable("bootloader", "src/main.zig");
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.linkSystemLibrary("efi");
    exe.linkSystemLibrary("eficrt");
    exe.install();
}
