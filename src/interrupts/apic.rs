use crate::vga;
use core::arch::asm;

const APIC_BASE_MSR: u32 = 0x1B;
const APIC_ENABLE: u64 = 0x800;

const APIC_ID: u32 = 0x20;
const APIC_VERSION: u32 = 0x30;
const APIC_TPR: u32 = 0x80;
const APIC_EOI: u32 = 0xB0;
const APIC_SIVR: u32 = 0xF0;
const APIC_ICR_LOW: u32 = 0x300;
const APIC_ICR_HIGH: u32 = 0x310;

static mut LOCAL_APIC_BASE: u64 = 0;

pub fn init() {
    vga::print!("Initializing Local APIC...\n");
    
    unsafe {
        let apic_base = read_msr(APIC_BASE_MSR);
        LOCAL_APIC_BASE = apic_base & 0xFFFFF000;
        
        vga::print!("Local APIC base: 0x{:x}\n", LOCAL_APIC_BASE);
        
        enable_apic();
        setup_spurious_vector();
        
        vga::print!("Local APIC initialized\n");
    }
}

unsafe fn enable_apic() {
    let apic_base = read_msr(APIC_BASE_MSR);
    write_msr(APIC_BASE_MSR, apic_base | APIC_ENABLE);
}

unsafe fn setup_spurious_vector() {
    let sivr = read_apic_register(APIC_SIVR);
    write_apic_register(APIC_SIVR, sivr | 0x100 | 0xFF);
}

pub fn send_eoi() {
    unsafe {
        write_apic_register(APIC_EOI, 0);
    }
}

pub fn send_ipi(target_cpu: u8, vector: u8) {
    unsafe {
        let icr_high = (target_cpu as u32) << 24;
        let icr_low = (vector as u32) | 0x00004000;
        
        write_apic_register(APIC_ICR_HIGH, icr_high);
        write_apic_register(APIC_ICR_LOW, icr_low);
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
