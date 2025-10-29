use crate::vga;

pub mod ethernet;
pub mod ip;
pub mod tcp;
pub mod udp;

pub struct NetworkStack {
    pub mac_address: [u8; 6],
    pub ip_address: [u8; 4],
    pub subnet_mask: [u8; 4],
    pub gateway: [u8; 4],
}

impl NetworkStack {
    pub fn new() -> Self {
        Self {
            mac_address: [0x02, 0x00, 0x00, 0x00, 0x00, 0x01],
            ip_address: [192, 168, 1, 100],
            subnet_mask: [255, 255, 255, 0],
            gateway: [192, 168, 1, 1],
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing network stack...\n");
        
        ethernet::init();
        ip::init();
        tcp::init();
        udp::init();
        
        vga::print!("Network stack initialized\n");
        vga::print!("  MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\n",
            self.mac_address[0], self.mac_address[1], self.mac_address[2],
            self.mac_address[3], self.mac_address[4], self.mac_address[5]);
        vga::print!("  IP: {}.{}.{}.{}\n",
            self.ip_address[0], self.ip_address[1], self.ip_address[2], self.ip_address[3]);
    }
    
    pub fn send_packet(&mut self, data: &[u8]) -> bool {
        ethernet::send_packet(data)
    }
    
    pub fn receive_packet(&mut self, buffer: &mut [u8]) -> usize {
        ethernet::receive_packet(buffer)
    }
}

pub static mut NETWORK: NetworkStack = NetworkStack::new();

pub fn init() {
    unsafe {
        NETWORK.init();
    }
}
