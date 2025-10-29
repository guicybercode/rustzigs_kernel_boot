use crate::vga;

pub struct NetworkDevice {
    pub mac_address: [u8; 6],
    pub ip_address: [u8; 4],
    pub subnet_mask: [u8; 4],
    pub gateway: [u8; 4],
}

static mut NETWORK_DEVICES: [Option<NetworkDevice>; 4] = [None; 4];
static mut NETWORK_COUNT: usize = 0;

pub fn init() {
    vga::print!("Initializing network devices...\n");
    
    unsafe {
        init_e1000_devices();
    }
}

unsafe fn init_e1000_devices() {
    vga::print!("E1000 network devices not yet implemented\n");
}

pub fn handle_interrupt() {
    vga::print!("Network device interrupt\n");
}

pub fn send_packet(device_id: usize, data: &[u8]) -> bool {
    unsafe {
        if device_id >= NETWORK_COUNT {
            return false;
        }
        
        vga::print!("Sending {} bytes on network device {}\n", data.len(), device_id);
        true
    }
}

pub fn receive_packet(device_id: usize, buffer: &mut [u8]) -> usize {
    unsafe {
        if device_id >= NETWORK_COUNT {
            return 0;
        }
        
        0
    }
}
