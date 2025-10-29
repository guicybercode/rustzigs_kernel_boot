use crate::vga;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

pub fn init() {
    vga::print!("Initializing PIC...\n");
    
    unsafe {
        outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        outb(PIC1_DATA, 32);
        outb(PIC1_DATA, 4);
        outb(PIC1_DATA, ICW4_8086);
        
        outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        outb(PIC2_DATA, 40);
        outb(PIC2_DATA, 2);
        outb(PIC2_DATA, ICW4_8086);
        
        outb(PIC1_DATA, 0xFE);
        outb(PIC2_DATA, 0xFF);
    }
    
    vga::print!("PIC initialized\n");
}

pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        unsafe {
            outb(PIC2_COMMAND, 0x20);
        }
    }
    
    unsafe {
        outb(PIC1_COMMAND, 0x20);
    }
}

fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}
