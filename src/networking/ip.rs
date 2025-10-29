use crate::vga;

#[repr(C, packed)]
pub struct IPHeader {
    pub version_ihl: u8,
    pub tos: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src_ip: [u8; 4],
    pub dest_ip: [u8; 4],
}

const IP_PROTOCOL_TCP: u8 = 6;
const IP_PROTOCOL_UDP: u8 = 17;

pub fn init() {
    vga::print!("IP layer initialized\n");
}

pub fn send_packet(dest_ip: [u8; 4], protocol: u8, data: &[u8]) -> bool {
    vga::print!("Sending IP packet to {}.{}.{}.{}\n",
        dest_ip[0], dest_ip[1], dest_ip[2], dest_ip[3]);
    true
}

pub fn receive_packet(buffer: &mut [u8]) -> usize {
    vga::print!("Receiving IP packet\n");
    0
}
