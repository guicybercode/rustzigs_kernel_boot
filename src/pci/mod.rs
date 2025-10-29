use crate::vga;
use core::ptr;

pub mod mcfg;
pub mod devices;

#[repr(C, packed)]
pub struct PCIConfigHeader {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub prog_if: u8,
    pub subclass: u8,
    pub class_code: u8,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
}

#[repr(C, packed)]
pub struct PCIDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub header: PCIConfigHeader,
    pub bars: [u32; 6],
}

pub struct PCIManager {
    devices: [Option<PCIDevice>; 256],
    device_count: usize,
    mcfg_base: u64,
}

impl PCIManager {
    pub fn new() -> Self {
        Self {
            devices: [None; 256],
            device_count: 0,
            mcfg_base: 0,
        }
    }

    pub fn init(&mut self, mcfg_base: u64) {
        self.mcfg_base = mcfg_base;
        vga::print!("PCI Manager initialized with MCFG at 0x{:x}\n", mcfg_base);
        self.enumerate_devices();
    }

    fn enumerate_devices(&mut self) {
        vga::print!("Enumerating PCI devices...\n");
        
        for bus in 0..256 {
            for device in 0..32 {
                for function in 0..8 {
                    if let Some(pci_device) = self.read_device(bus, device, function) {
                        if pci_device.header.vendor_id != 0xFFFF {
                            self.devices[self.device_count] = Some(pci_device);
                            self.device_count += 1;
                            
                            vga::print!("PCI Device: {:02x}:{:02x}.{} - {:04x}:{:04x} - Class: {:02x}\n",
                                bus, device, function,
                                pci_device.header.vendor_id,
                                pci_device.header.device_id,
                                pci_device.header.class_code
                            );
                        }
                    }
                }
            }
        }
        
        vga::print!("Found {} PCI devices\n", self.device_count);
    }

    fn read_device(&self, bus: u8, device: u8, function: u8) -> Option<PCIDevice> {
        let config_address = self.calculate_config_address(bus, device, function);
        
        unsafe {
            let header = ptr::read_volatile(config_address as *const PCIConfigHeader);
            
            if header.vendor_id == 0xFFFF {
                return None;
            }
            
            let mut bars = [0u32; 6];
            for i in 0..6 {
                let bar_offset = 0x10 + (i * 4) as u32;
                bars[i] = self.read_config_register(config_address, bar_offset);
            }
            
            Some(PCIDevice {
                bus,
                device,
                function,
                header,
                bars,
            })
        }
    }

    fn calculate_config_address(&self, bus: u8, device: u8, function: u8) -> u64 {
        if self.mcfg_base == 0 {
            self.legacy_config_address(bus, device, function)
        } else {
            self.mcfg_base + ((bus as u64) << 20) + ((device as u64) << 15) + ((function as u64) << 12)
        }
    }

    fn legacy_config_address(&self, bus: u8, device: u8, function: u8) -> u64 {
        0x80000000 | ((bus as u32) << 16) | ((device as u32) << 11) | ((function as u32) << 8)
    }

    fn read_config_register(&self, config_address: u64, offset: u32) -> u32 {
        unsafe {
            if self.mcfg_base == 0 {
                let address = config_address | (offset & 0xFC);
                ptr::write_volatile(0xCF8 as *mut u32, address);
                ptr::read_volatile(0xCFC as *const u32)
            } else {
                ptr::read_volatile((config_address + offset as u64) as *const u32)
            }
        }
    }

    pub fn find_device(&self, vendor_id: u16, device_id: u16) -> Option<&PCIDevice> {
        for device in &self.devices {
            if let Some(dev) = device {
                if dev.header.vendor_id == vendor_id && dev.header.device_id == device_id {
                    return Some(dev);
                }
            }
        }
        None
    }

    pub fn find_device_by_class(&self, class_code: u8, subclass: u8) -> Option<&PCIDevice> {
        for device in &self.devices {
            if let Some(dev) = device {
                if dev.header.class_code == class_code && dev.header.subclass == subclass {
                    return Some(dev);
                }
            }
        }
        None
    }
}

pub static mut PCI_MANAGER: PCIManager = PCIManager::new();

pub fn init(mcfg_base: u64) {
    unsafe {
        PCI_MANAGER.init(mcfg_base);
    }
}
