use crate::vga;

#[repr(C, packed)]
pub struct TCPHeader {
    pub src_port: u16,
    pub dest_port: u16,
    pub sequence: u32,
    pub ack_sequence: u32,
    pub flags: u16,
    pub window: u16,
    pub checksum: u16,
    pub urgent: u16,
}

pub fn init() {
    vga::print!("TCP layer initialized\n");
}

pub fn send_packet(dest_ip: [u8; 4], dest_port: u16, data: &[u8]) -> bool {
    vga::print!("Sending TCP packet to {}.{}.{}.{}:{}\n",
        dest_ip[0], dest_ip[1], dest_ip[2], dest_ip[3], dest_port);
    true
}

pub fn receive_packet(buffer: &mut [u8]) -> usize {
    vga::print!("Receiving TCP packet\n");
    0
}
