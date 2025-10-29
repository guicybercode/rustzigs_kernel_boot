use crate::vga;
use core::arch::asm;

pub mod idt;
pub mod handlers;
pub mod pic;
pub mod apic;

#[repr(C, packed)]
pub struct InterruptFrame {
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

pub type InterruptHandler = fn(InterruptFrame) -> !;

static mut INTERRUPT_HANDLERS: [Option<InterruptHandler>; 256] = [None; 256];

pub fn init() {
    vga::print!("Initializing interrupt system...\n");
    
    unsafe {
        idt::init();
        pic::init();
        apic::init();
        
        vga::print!("Interrupt system initialized\n");
    }
}

pub fn register_handler(interrupt: u8, handler: InterruptHandler) {
    unsafe {
        INTERRUPT_HANDLERS[interrupt as usize] = Some(handler);
    }
}

pub fn handle_interrupt(frame: InterruptFrame, interrupt: u8) {
    unsafe {
        if let Some(handler) = INTERRUPT_HANDLERS[interrupt as usize] {
            handler(frame);
        } else {
            vga::print!("Unhandled interrupt: {}\n", interrupt);
        }
    }
}

pub fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}

pub fn disable_interrupts() {
    unsafe {
        asm!("cli");
    }
}

pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}
