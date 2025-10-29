use crate::vga;
use core::arch::asm;

const APIC_BASE_MSR: u32 = 0x1B;
const APIC_ENABLE: u64 = 0x800;

static mut LOCAL_APIC_BASE: u64 = 0;

pub fn init() {
    unsafe {
        let apic_base = read_msr(APIC_BASE_MSR);
        LOCAL_APIC_BASE = apic_base & 0xFFFFF000;
        
        vga::print!("Local APIC base: 0x{:x}\n", LOCAL_APIC_BASE);
        
        enable_apic();
        vga::print!("Local APIC enabled\n");
        
        init_ap_cores();
    }
}

fn read_msr(msr: u32) -> u64 {
    let (low, high): (u32, u32);
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
        );
    }
    (high as u64) << 32 | (low as u64)
}

fn write_msr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high,
        );
    }
}

fn enable_apic() {
    unsafe {
        let apic_base = read_msr(APIC_BASE_MSR);
        write_msr(APIC_BASE_MSR, apic_base | APIC_ENABLE);
    }
}

fn init_ap_cores() {
    vga::print!("Initializing AP cores...\n");
    
    let apic_id = read_apic_register(0x20);
    vga::print!("BSP APIC ID: {}\n", apic_id);
    
    send_init_ipi();
    send_startup_ipi();
}

fn read_apic_register(offset: u32) -> u32 {
    unsafe {
        let addr = LOCAL_APIC_BASE + offset as u64;
        *(addr as *const u32)
    }
}

fn write_apic_register(offset: u32, value: u32) {
    unsafe {
        let addr = LOCAL_APIC_BASE + offset as u64;
        *(addr as *mut u32) = value;
    }
}

fn send_init_ipi() {
    write_apic_register(0x310, 0x000C4500);
    write_apic_register(0x300, 0x000C4500);
}

fn send_startup_ipi() {
    write_apic_register(0x310, 0x000C4600);
    write_apic_register(0x300, 0x000C4600);
}
