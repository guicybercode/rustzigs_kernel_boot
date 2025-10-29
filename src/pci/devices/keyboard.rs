use crate::vga;
use core::sync::atomic::{AtomicU8, Ordering};

static mut KEYBOARD_BUFFER: [u8; 256] = [0; 256];
static mut KEYBOARD_HEAD: AtomicU8 = AtomicU8::new(0);
static mut KEYBOARD_TAIL: AtomicU8 = AtomicU8::new(0);

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

pub fn init() {
    vga::print!("Initializing PS/2 keyboard...\n");
    
    unsafe {
        enable_keyboard();
    }
}

unsafe fn enable_keyboard() {
    while (inb(KEYBOARD_STATUS_PORT) & 0x02) != 0 {}
    
    outb(KEYBOARD_STATUS_PORT, 0xAE);
    
    while (inb(KEYBOARD_STATUS_PORT) & 0x02) != 0 {}
    
    outb(KEYBOARD_DATA_PORT, 0xF4);
}

pub fn handle_interrupt() {
    unsafe {
        if (inb(KEYBOARD_STATUS_PORT) & 0x01) != 0 {
            let scancode = inb(KEYBOARD_DATA_PORT);
            process_scancode(scancode);
        }
    }
}

unsafe fn process_scancode(scancode: u8) {
    let head = KEYBOARD_HEAD.load(Ordering::Relaxed);
    let next_head = (head + 1) % 256;
    
    if next_head != KEYBOARD_TAIL.load(Ordering::Relaxed) {
        KEYBOARD_BUFFER[head as usize] = scancode;
        KEYBOARD_HEAD.store(next_head, Ordering::Relaxed);
    }
}

pub fn read_key() -> Option<u8> {
    unsafe {
        let tail = KEYBOARD_TAIL.load(Ordering::Relaxed);
        
        if tail != KEYBOARD_HEAD.load(Ordering::Relaxed) {
            let key = KEYBOARD_BUFFER[tail as usize];
            KEYBOARD_TAIL.store((tail + 1) % 256, Ordering::Relaxed);
            Some(key)
        } else {
            None
        }
    }
}

fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") result);
    }
    result
}

fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}
