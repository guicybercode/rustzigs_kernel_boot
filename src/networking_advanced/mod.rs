use crate::vga;

pub mod ipv6;
pub mod tls;
pub mod firewall;
pub mod routing;

pub struct AdvancedNetworking {
    pub ipv6_enabled: bool,
    pub tls_enabled: bool,
    pub firewall_enabled: bool,
    pub routing_enabled: bool,
}

impl AdvancedNetworking {
    pub fn new() -> Self {
        Self {
            ipv6_enabled: false,
            tls_enabled: false,
            firewall_enabled: false,
            routing_enabled: false,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Advanced Networking...\n");
        
        ipv6::init();
        tls::init();
        firewall::init();
        routing::init();
        
        self.ipv6_enabled = true;
        self.tls_enabled = true;
        self.firewall_enabled = true;
        self.routing_enabled = true;
        
        vga::print!("Advanced Networking initialized\n");
    }
}

pub static mut ADVANCED_NETWORKING: AdvancedNetworking = AdvancedNetworking::new();

pub fn init() {
    unsafe {
        ADVANCED_NETWORKING.init();
    }
}
