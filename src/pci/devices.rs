use crate::vga;
use crate::pci::PCIDevice;

pub mod keyboard;
pub mod mouse;
pub mod storage;
pub mod network;

pub fn init_pci_devices() {
    vga::print!("Initializing PCI devices...\n");
    
    keyboard::init();
    mouse::init();
    storage::init();
    network::init();
}

pub fn handle_pci_interrupt(device: &PCIDevice, interrupt_line: u8) {
    match device.header.class_code {
        0x03 => {
            match device.header.subclass {
                0x00 => vga::print!("VGA interrupt\n"),
                0x01 => vga::print!("VGA 3D controller interrupt\n"),
                _ => vga::print!("Unknown display interrupt\n"),
            }
        }
        0x0C => {
            match device.header.subclass {
                0x03 => keyboard::handle_interrupt(),
                _ => vga::print!("Unknown input device interrupt\n"),
            }
        }
        0x01 => {
            match device.header.subclass {
                0x01 => storage::handle_interrupt(),
                _ => vga::print!("Unknown storage interrupt\n"),
            }
        }
        0x02 => {
            network::handle_interrupt();
        }
        _ => vga::print!("Unknown PCI device interrupt: Class 0x{:02x}\n", device.header.class_code),
    }
}
