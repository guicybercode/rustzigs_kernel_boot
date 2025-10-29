use crate::vga;

pub fn init() {
    vga::print!("GDB stub initialized\n");
}

pub fn handle_gdb_packet(packet: &[u8]) {
    vga::print!("GDB packet received: {}\n", core::str::from_utf8(packet).unwrap_or("Invalid UTF-8"));
}
