use crate::vga;

pub mod hypervisor;
pub mod vms;
pub mod emulation;

pub struct VirtualizationManager {
    pub hypervisor_enabled: bool,
    pub vm_count: u32,
    pub emulation_enabled: bool,
}

impl VirtualizationManager {
    pub fn new() -> Self {
        Self {
            hypervisor_enabled: false,
            vm_count: 0,
            emulation_enabled: false,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Virtualization...\n");
        
        hypervisor::init();
        vms::init();
        emulation::init();
        
        self.hypervisor_enabled = true;
        self.emulation_enabled = true;
        
        vga::print!("Virtualization initialized\n");
    }
}

pub static mut VIRTUALIZATION: VirtualizationManager = VirtualizationManager::new();

pub fn init() {
    unsafe {
        VIRTUALIZATION.init();
    }
}
