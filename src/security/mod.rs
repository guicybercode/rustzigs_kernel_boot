use crate::vga;

pub mod aslr;
pub mod stack_canaries;
pub mod capabilities;
pub mod sandboxing;

pub struct SecurityManager {
    pub aslr_enabled: bool,
    pub stack_canaries_enabled: bool,
    pub capabilities_enabled: bool,
    pub sandboxing_enabled: bool,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            aslr_enabled: false,
            stack_canaries_enabled: false,
            capabilities_enabled: false,
            sandboxing_enabled: false,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Security System...\n");
        
        aslr::init();
        stack_canaries::init();
        capabilities::init();
        sandboxing::init();
        
        self.aslr_enabled = true;
        self.stack_canaries_enabled = true;
        self.capabilities_enabled = true;
        self.sandboxing_enabled = true;
        
        vga::print!("Security System initialized\n");
    }
}

pub static mut SECURITY_MANAGER: SecurityManager = SecurityManager::new();

pub fn init() {
    unsafe {
        SECURITY_MANAGER.init();
    }
}
