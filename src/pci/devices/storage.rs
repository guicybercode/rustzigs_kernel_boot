use crate::vga;
use crate::pci::PCIDevice;

pub struct StorageDevice {
    pub device_type: StorageType,
    pub capacity: u64,
    pub sector_size: u32,
    pub lba_count: u64,
}

pub enum StorageType {
    ATA,
    SATA,
    NVMe,
    SCSI,
}

static mut STORAGE_DEVICES: [Option<StorageDevice>; 8] = [None; 8];
static mut STORAGE_COUNT: usize = 0;

pub fn init() {
    vga::print!("Initializing storage devices...\n");
    
    unsafe {
        init_ata_devices();
        init_nvme_devices();
    }
}

unsafe fn init_ata_devices() {
    for i in 0..4 {
        let device = StorageDevice {
            device_type: StorageType::ATA,
            capacity: 0,
            sector_size: 512,
            lba_count: 0,
        };
        
        STORAGE_DEVICES[STORAGE_COUNT] = Some(device);
        STORAGE_COUNT += 1;
    }
    
    vga::print!("Found {} ATA devices\n", 4);
}

unsafe fn init_nvme_devices() {
    vga::print!("NVMe devices not yet implemented\n");
}

pub fn handle_interrupt() {
    vga::print!("Storage device interrupt\n");
}

pub fn read_sector(device_id: usize, lba: u64, buffer: &mut [u8]) -> bool {
    unsafe {
        if device_id >= STORAGE_COUNT {
            return false;
        }
        
        if let Some(device) = &STORAGE_DEVICES[device_id] {
            match device.device_type {
                StorageType::ATA => read_ata_sector(device_id, lba, buffer),
                StorageType::NVMe => read_nvme_sector(device_id, lba, buffer),
                _ => false,
            }
        } else {
            false
        }
    }
}

unsafe fn read_ata_sector(device_id: usize, lba: u64, buffer: &mut [u8]) -> bool {
    let base_port = 0x1F0 + (device_id as u16 * 0x10);
    
    outb(base_port + 6, 0x40 | ((device_id as u8) << 4));
    outb(base_port + 2, 1);
    outb(base_port + 3, (lba & 0xFF) as u8);
    outb(base_port + 4, ((lba >> 8) & 0xFF) as u8);
    outb(base_port + 5, ((lba >> 16) & 0xFF) as u8);
    outb(base_port + 7, 0x20);
    
    while (inb(base_port + 7) & 0x80) != 0 {}
    
    for i in 0..256 {
        let data = inw(base_port);
        buffer[i * 2] = (data & 0xFF) as u8;
        buffer[i * 2 + 1] = ((data >> 8) & 0xFF) as u8;
    }
    
    true
}

unsafe fn read_nvme_sector(device_id: usize, lba: u64, buffer: &mut [u8]) -> bool {
    vga::print!("NVMe read not implemented\n");
    false
}

fn inb(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!("in al, dx", in("dx") port, out("al") result);
    }
    result
}

fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}

fn inw(port: u16) -> u16 {
    let result: u16;
    unsafe {
        asm!("in ax, dx", in("dx") port, out("ax") result);
    }
    result
}
