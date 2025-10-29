use crate::vga;

pub mod acpi_power;
pub mod cpu_freq;
pub mod thermal;
pub mod battery;

pub struct PowerManager {
    pub acpi_power_enabled: bool,
    pub cpu_freq_scaling: bool,
    pub thermal_management: bool,
    pub battery_monitoring: bool,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            acpi_power_enabled: false,
            cpu_freq_scaling: false,
            thermal_management: false,
            battery_monitoring: false,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Power Management...\n");
        
        acpi_power::init();
        cpu_freq::init();
        thermal::init();
        battery::init();
        
        self.acpi_power_enabled = true;
        self.cpu_freq_scaling = true;
        self.thermal_management = true;
        self.battery_monitoring = true;
        
        vga::print!("Power Management initialized\n");
    }
}

pub static mut POWER_MANAGER: PowerManager = PowerManager::new();

pub fn init() {
    unsafe {
        POWER_MANAGER.init();
    }
}
