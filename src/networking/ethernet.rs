use crate::vga;

#[repr(C, packed)]
pub struct EthernetHeader {
    pub dest_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16,
}

const ETHERTYPE_IP: u16 = 0x0800;
const ETHERTYPE_ARP: u16 = 0x0806;

pub fn init() {
    vga::print!("Ethernet layer initialized\n");
}

pub fn send_packet(data: &[u8]) -> bool {
    vga::print!("Sending {} bytes via Ethernet\n", data.len());
    true
}

pub fn receive_packet(buffer: &mut [u8]) -> usize {
    vga::print!("Receiving packet via Ethernet\n");
    0
}
