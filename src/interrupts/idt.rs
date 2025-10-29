use crate::vga;
use core::arch::asm;

#[repr(C, packed)]
pub struct IDTEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub ist: u8,
    pub type_attributes: u8,
    pub offset_mid: u16,
    pub offset_high: u32,
    pub reserved: u32,
}

#[repr(C, packed)]
pub struct IDTPointer {
    pub limit: u16,
    pub base: u64,
}

static mut IDT: [IDTEntry; 256] = [IDTEntry {
    offset_low: 0,
    selector: 0,
    ist: 0,
    type_attributes: 0,
    offset_mid: 0,
    offset_high: 0,
    reserved: 0,
}; 256];

pub fn init() {
    vga::print!("Setting up IDT...\n");
    
    unsafe {
        setup_exceptions();
        setup_interrupts();
        load_idt();
    }
}

unsafe fn setup_exceptions() {
    set_gate(0, exception_div_by_zero as u64, 0x08, 0x8E);
    set_gate(1, exception_debug as u64, 0x08, 0x8E);
    set_gate(2, exception_nmi as u64, 0x08, 0x8E);
    set_gate(3, exception_breakpoint as u64, 0x08, 0x8E);
    set_gate(4, exception_overflow as u64, 0x08, 0x8E);
    set_gate(5, exception_bound_range as u64, 0x08, 0x8E);
    set_gate(6, exception_invalid_opcode as u64, 0x08, 0x8E);
    set_gate(7, exception_device_not_available as u64, 0x08, 0x8E);
    set_gate(8, exception_double_fault as u64, 0x08, 0x8E);
    set_gate(10, exception_invalid_tss as u64, 0x08, 0x8E);
    set_gate(11, exception_segment_not_present as u64, 0x08, 0x8E);
    set_gate(12, exception_stack_segment_fault as u64, 0x08, 0x8E);
    set_gate(13, exception_general_protection as u64, 0x08, 0x8E);
    set_gate(14, exception_page_fault as u64, 0x08, 0x8E);
    set_gate(16, exception_x87_fpu_error as u64, 0x08, 0x8E);
    set_gate(17, exception_alignment_check as u64, 0x08, 0x8E);
    set_gate(18, exception_machine_check as u64, 0x08, 0x8E);
    set_gate(19, exception_simd_fpu_exception as u64, 0x08, 0x8E);
    set_gate(20, exception_virtualization as u64, 0x08, 0x8E);
}

unsafe fn setup_interrupts() {
    for i in 32..256 {
        set_gate(i, interrupt_stub as u64, 0x08, 0x8E);
    }
}

unsafe fn set_gate(num: usize, handler: u64, selector: u16, type_attributes: u8) {
    IDT[num].offset_low = (handler & 0xFFFF) as u16;
    IDT[num].selector = selector;
    IDT[num].ist = 0;
    IDT[num].type_attributes = type_attributes;
    IDT[num].offset_mid = ((handler >> 16) & 0xFFFF) as u16;
    IDT[num].offset_high = ((handler >> 32) & 0xFFFFFFFF) as u32;
    IDT[num].reserved = 0;
}

unsafe fn load_idt() {
    let idt_ptr = IDTPointer {
        limit: (core::mem::size_of::<[IDTEntry; 256]>() - 1) as u16,
        base: &IDT as *const _ as u64,
    };
    
    asm!("lidt [{}]", in(reg) &idt_ptr);
}

extern "x86-interrupt" fn exception_div_by_zero(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Division by zero at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_debug(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Debug at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_nmi(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: NMI at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_breakpoint(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Breakpoint at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_overflow(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Overflow at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_bound_range(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Bound range exceeded at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_invalid_opcode(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Invalid opcode at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_device_not_available(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Device not available at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_double_fault(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Double fault at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_invalid_tss(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Invalid TSS at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_segment_not_present(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Segment not present at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_stack_segment_fault(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Stack segment fault at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_general_protection(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: General protection fault at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_page_fault(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Page fault at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_x87_fpu_error(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: x87 FPU error at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_alignment_check(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Alignment check at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_machine_check(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Machine check at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_simd_fpu_exception(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: SIMD FPU exception at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn exception_virtualization(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Exception: Virtualization exception at RIP=0x{:x}\n", frame.rip);
    loop {}
}

extern "x86-interrupt" fn interrupt_stub(frame: crate::interrupts::InterruptFrame) -> ! {
    vga::print!("Unhandled interrupt\n");
    loop {}
}
