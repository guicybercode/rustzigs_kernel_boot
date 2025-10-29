use crate::vga;

const MOUSE_DATA_PORT: u16 = 0x60;
const MOUSE_STATUS_PORT: u16 = 0x64;
const MOUSE_COMMAND_PORT: u16 = 0x64;

pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub buttons: u8,
}

static mut MOUSE_STATE: MouseState = MouseState {
    x: 0,
    y: 0,
    buttons: 0,
};

static mut MOUSE_PACKET: [u8; 3] = [0; 3];
static mut MOUSE_PACKET_INDEX: usize = 0;

pub fn init() {
    vga::print!("Initializing PS/2 mouse...\n");
    
    unsafe {
        enable_mouse();
    }
}

unsafe fn enable_mouse() {
    while (inb(MOUSE_STATUS_PORT) & 0x02) != 0 {}
    
    outb(MOUSE_COMMAND_PORT, 0xA8);
    
    while (inb(MOUSE_STATUS_PORT) & 0x02) != 0 {}
    
    outb(MOUSE_DATA_PORT, 0xF4);
}

pub fn handle_interrupt() {
    unsafe {
        if (inb(MOUSE_STATUS_PORT) & 0x01) != 0 {
            let data = inb(MOUSE_DATA_PORT);
            process_mouse_data(data);
        }
    }
}

unsafe fn process_mouse_data(data: u8) {
    MOUSE_PACKET[MOUSE_PACKET_INDEX] = data;
    MOUSE_PACKET_INDEX += 1;
    
    if MOUSE_PACKET_INDEX == 3 {
        MOUSE_PACKET_INDEX = 0;
        
        let buttons = MOUSE_PACKET[0] & 0x07;
        let x_delta = ((MOUSE_PACKET[0] & 0x10) as i8 as i32) << 4 | (MOUSE_PACKET[1] as i32);
        let y_delta = ((MOUSE_PACKET[0] & 0x20) as i8 as i32) << 3 | (MOUSE_PACKET[2] as i32);
        
        MOUSE_STATE.buttons = buttons;
        MOUSE_STATE.x += x_delta;
        MOUSE_STATE.y -= y_delta;
        
        if MOUSE_STATE.x < 0 { MOUSE_STATE.x = 0; }
        if MOUSE_STATE.y < 0 { MOUSE_STATE.y = 0; }
        if MOUSE_STATE.x >= 80 { MOUSE_STATE.x = 79; }
        if MOUSE_STATE.y >= 25 { MOUSE_STATE.y = 24; }
    }
}

pub fn get_mouse_state() -> MouseState {
    unsafe { MOUSE_STATE }
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
