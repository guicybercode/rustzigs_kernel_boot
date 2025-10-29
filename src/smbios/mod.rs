use crate::vga;

#[repr(C, packed)]
pub struct SMBIOSEntryPoint {
    pub anchor_string: [u8; 4],
    pub checksum: u8,
    pub length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub max_structure_size: u16,
    pub entry_point_revision: u8,
    pub formatted_area: [u8; 5],
    pub intermediate_anchor_string: [u8; 5],
    pub intermediate_checksum: u8,
    pub structure_table_length: u16,
    pub structure_table_address: u32,
    pub number_of_structures: u16,
    pub bcd_revision: u8,
}

static mut SMBIOS_ENTRY: u64 = 0;

pub fn init(entry_addr: u64) {
    unsafe {
        SMBIOS_ENTRY = entry_addr;
        
        if entry_addr == 0 {
            vga::print!("SMBIOS entry point not found\n");
            return;
        }
        
        vga::print!("SMBIOS entry point at: 0x{:x}\n", entry_addr);
        
        if let Some(entry) = parse_smbios_entry(entry_addr) {
            vga::print!("SMBIOS version: {}.{}\n", entry.major_version, entry.minor_version);
            vga::print!("SMBIOS structures: {}\n", entry.number_of_structures);
            
            parse_smbios_tables(entry);
        }
    }
}

fn parse_smbios_entry(addr: u64) -> Option<&'static SMBIOSEntryPoint> {
    unsafe {
        let entry = &*(addr as *const SMBIOSEntryPoint);
        
        if &entry.anchor_string != b"_SM_" {
            return None;
        }
        
        let mut sum: u8 = 0;
        let bytes = core::slice::from_raw_parts(addr as *const u8, entry.length as usize);
        for &byte in bytes {
            sum = sum.wrapping_add(byte);
        }
        
        if sum != 0 {
            return None;
        }
        
        Some(entry)
    }
}

fn parse_smbios_tables(entry: &SMBIOSEntryPoint) {
    let table_addr = entry.structure_table_address as u64;
    let mut current_addr = table_addr;
    
    for _ in 0..entry.number_of_structures {
        unsafe {
            let header = &*(current_addr as *const SMBIOSStructureHeader);
            
            if header.length < 4 {
                break;
            }
            
            match header.type_ {
                0 => {
                    vga::print!("Found BIOS Information structure\n");
                }
                1 => {
                    vga::print!("Found System Information structure\n");
                }
                4 => {
                    vga::print!("Found Processor Information structure\n");
                }
                16 => {
                    vga::print!("Found Physical Memory Array structure\n");
                }
                17 => {
                    vga::print!("Found Memory Device structure\n");
                }
                _ => {}
            }
            
            current_addr += header.length as u64;
            while current_addr < table_addr + entry.structure_table_length as u64 {
                let byte = *(current_addr as *const u8);
                if byte == 0 && *(current_addr as *const u16) == 0 {
                    current_addr += 2;
                    break;
                }
                current_addr += 1;
            }
        }
    }
}

#[repr(C, packed)]
struct SMBIOSStructureHeader {
    type_: u8,
    length: u8,
    handle: u16,
}
