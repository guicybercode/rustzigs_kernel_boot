use crate::vga;

pub mod host_controller;
pub mod devices;

pub struct USBManager {
    pub controllers: [Option<host_controller::USBController>; 4],
    pub device_count: usize,
}

impl USBManager {
    pub fn new() -> Self {
        Self {
            controllers: [None; 4],
            device_count: 0,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing USB system...\n");
        
        host_controller::init();
        devices::init();
        
        vga::print!("USB system initialized\n");
    }
}

pub static mut USB: USBManager = USBManager::new();

pub fn init() {
    unsafe {
        USB.init();
    }
}
