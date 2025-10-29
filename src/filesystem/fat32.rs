use crate::vga;

#[repr(C, packed)]
pub struct FAT32BootSector {
    pub jump_instruction: [u8; 3],
    pub oem_name: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub number_of_fats: u8,
    pub root_entries: u16,
    pub total_sectors_16: u16,
    pub media_type: u8,
    pub sectors_per_fat_16: u16,
    pub sectors_per_track: u16,
    pub heads: u16,
    pub hidden_sectors: u32,
    pub total_sectors_32: u32,
    pub sectors_per_fat_32: u32,
    pub flags: u16,
    pub version: u16,
    pub root_cluster: u32,
    pub fs_info_sector: u16,
    pub backup_boot_sector: u16,
    pub reserved: [u8; 12],
    pub drive_number: u8,
    pub reserved1: u8,
    pub boot_signature: u8,
    pub volume_id: u32,
    pub volume_label: [u8; 11],
    pub file_system_type: [u8; 8],
}

#[repr(C, packed)]
pub struct FAT32DirectoryEntry {
    pub name: [u8; 11],
    pub attributes: u8,
    pub reserved: u8,
    pub creation_time_tenths: u8,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_access_date: u16,
    pub first_cluster_high: u16,
    pub last_write_time: u16,
    pub last_write_date: u16,
    pub first_cluster_low: u16,
    pub file_size: u32,
}

pub struct FAT32FileSystem {
    boot_sector: FAT32BootSector,
    fat_start: u64,
    root_start: u64,
    data_start: u64,
    sectors_per_cluster: u32,
    bytes_per_sector: u32,
}

impl FAT32FileSystem {
    pub fn new() -> Self {
        Self {
            boot_sector: unsafe { core::mem::zeroed() },
            fat_start: 0,
            root_start: 0,
            data_start: 0,
            sectors_per_cluster: 0,
            bytes_per_sector: 0,
        }
    }
    
    pub fn init(&mut self, device_id: usize) -> bool {
        vga::print!("Initializing FAT32 filesystem on device {}\n", device_id);
        
        let mut boot_sector_data = [0u8; 512];
        if !self.read_sector(device_id, 0, &mut boot_sector_data) {
            vga::print!("Failed to read boot sector\n");
            return false;
        }
        
        self.boot_sector = unsafe { core::ptr::read(boot_sector_data.as_ptr() as *const FAT32BootSector) };
        
        if &self.boot_sector.file_system_type[0..8] != b"FAT32   " {
            vga::print!("Not a FAT32 filesystem\n");
            return false;
        }
        
        self.bytes_per_sector = self.boot_sector.bytes_per_sector as u32;
        self.sectors_per_cluster = self.boot_sector.sectors_per_cluster as u32;
        
        self.fat_start = (self.boot_sector.reserved_sectors as u64) * (self.bytes_per_sector as u64);
        self.root_start = self.fat_start + (self.boot_sector.number_of_fats as u64) * (self.boot_sector.sectors_per_fat_32 as u64) * (self.bytes_per_sector as u64);
        self.data_start = self.root_start;
        
        vga::print!("FAT32 filesystem initialized\n");
        vga::print!("  Bytes per sector: {}\n", self.bytes_per_sector);
        vga::print!("  Sectors per cluster: {}\n", self.sectors_per_cluster);
        vga::print!("  Root cluster: {}\n", self.boot_sector.root_cluster);
        
        true
    }
    
    fn read_sector(&self, device_id: usize, sector: u64, buffer: &mut [u8]) -> bool {
        crate::pci::devices::storage::read_sector(device_id, sector, buffer)
    }
    
    fn cluster_to_sector(&self, cluster: u32) -> u64 {
        (cluster - 2) as u64 * self.sectors_per_cluster as u64 + self.data_start / self.bytes_per_sector as u64
    }
    
    pub fn read_file(&self, device_id: usize, cluster: u32, size: u32, buffer: &mut [u8]) -> usize {
        let mut current_cluster = cluster;
        let mut bytes_read = 0;
        let mut buffer_offset = 0;
        
        while current_cluster < 0xFFFFFFF8 && bytes_read < size as usize {
            let sector = self.cluster_to_sector(current_cluster);
            let mut sector_data = [0u8; 512];
            
            if self.read_sector(device_id, sector, &mut sector_data) {
                let bytes_to_copy = core::cmp::min(512, size as usize - bytes_read);
                let bytes_to_copy = core::cmp::min(bytes_to_copy, buffer.len() - buffer_offset);
                
                buffer[buffer_offset..buffer_offset + bytes_to_copy]
                    .copy_from_slice(&sector_data[0..bytes_to_copy]);
                
                buffer_offset += bytes_to_copy;
                bytes_read += bytes_to_copy;
            }
            
            current_cluster = self.get_next_cluster(device_id, current_cluster);
        }
        
        bytes_read
    }
    
    fn get_next_cluster(&self, device_id: usize, cluster: u32) -> u32 {
        let fat_offset = cluster as u64 * 4;
        let fat_sector = self.fat_start / self.bytes_per_sector as u64 + fat_offset / self.bytes_per_sector as u64;
        let fat_entry_offset = (fat_offset % self.bytes_per_sector as u64) as usize;
        
        let mut fat_sector_data = [0u8; 512];
        if self.read_sector(device_id, fat_sector, &mut fat_sector_data) {
            let fat_entry = u32::from_le_bytes([
                fat_sector_data[fat_entry_offset],
                fat_sector_data[fat_entry_offset + 1],
                fat_sector_data[fat_entry_offset + 2],
                fat_sector_data[fat_entry_offset + 3],
            ]);
            fat_entry & 0x0FFFFFFF
        } else {
            0xFFFFFFF8
        }
    }
}
