use crate::vga;

#[repr(C, packed)]
pub struct VESAMode {
    pub mode_number: u16,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub framebuffer_address: u64,
    pub bytes_per_scanline: u32,
}

pub fn find_best_mode() -> Option<VESAMode> {
    vga::print!("Searching for VESA modes...\n");
    
    let modes = [
        VESAMode {
            mode_number: 0x118,
            width: 1024,
            height: 768,
            bpp: 32,
            framebuffer_address: 0xE0000000,
            bytes_per_scanline: 4096,
        },
        VESAMode {
            mode_number: 0x115,
            width: 800,
            height: 600,
            bpp: 32,
            framebuffer_address: 0xE0000000,
            bytes_per_scanline: 3200,
        },
        VESAMode {
            mode_number: 0x114,
            width: 640,
            height: 480,
            bpp: 32,
            framebuffer_address: 0xE0000000,
            bytes_per_scanline: 2560,
        },
    ];
    
    for mode in &modes {
        vga::print!("Found mode: {}x{}x{}bpp (0x{:x})\n", 
            mode.width, mode.height, mode.bpp, mode.mode_number);
    }
    
    modes.first().copied()
}

pub fn set_mode(mode: VESAMode) -> bool {
    vga::print!("Setting VESA mode 0x{:x}...\n", mode.mode_number);
    
    unsafe {
        let result = set_vesa_mode(mode.mode_number);
        if result {
            vga::print!("VESA mode set successfully\n");
        } else {
            vga::print!("Failed to set VESA mode\n");
        }
        result
    }
}

unsafe fn set_vesa_mode(mode: u16) -> bool {
    let mut regs = VESARegisters::new();
    
    regs.ax = 0x4F02;
    regs.bx = mode as u32 | 0x4000;
    
    int86(0x10, &mut regs);
    
    regs.ax == 0x004F
}

#[repr(C, packed)]
struct VESARegisters {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub si: u16,
    pub di: u16,
    pub es: u16,
    pub ds: u16,
    pub flags: u16,
}

impl VESARegisters {
    fn new() -> Self {
        Self {
            ax: 0, bx: 0, cx: 0, dx: 0,
            si: 0, di: 0, es: 0, ds: 0, flags: 0,
        }
    }
}

unsafe fn int86(interrupt: u8, regs: &mut VESARegisters) {
    asm!(
        "int {interrupt}",
        interrupt = const(interrupt),
        inout("ax") regs.ax,
        inout("bx") regs.bx,
        inout("cx") regs.cx,
        inout("dx") regs.dx,
        inout("si") regs.si,
        inout("di") regs.di,
        inout("es") regs.es,
        inout("ds") regs.ds,
        lateout("flags") regs.flags,
    );
}
