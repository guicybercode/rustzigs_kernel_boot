use crate::vga;

#[repr(C, packed)]
pub struct MCFGEntry {
    pub base_address: u64,
    pub segment_group: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub reserved: u32,
}

#[repr(C, packed)]
pub struct MCFGTable {
    pub header: crate::acpi::tables::ACPITableHeader,
    pub reserved: u64,
    pub entries: [MCFGEntry; 0],
}

pub fn parse_mcfg(mcfg_addr: u64) -> Option<u64> {
    unsafe {
        let mcfg = &*(mcfg_addr as *const MCFGTable);
        
        if &mcfg.header.signature != b"MCFG" {
            vga::print!("Invalid MCFG signature\n");
            return None;
        }
        
        vga::print!("MCFG table found with {} entries\n", 
            (mcfg.header.length - 44) / 16);
        
        if mcfg.header.length >= 44 {
            let entry = &mcfg.entries[0];
            vga::print!("MCFG Entry: Base=0x{:x}, Segment={}, Bus={}-{}\n",
                entry.base_address,
                entry.segment_group,
                entry.start_bus,
                entry.end_bus
            );
            
            return Some(entry.base_address);
        }
        
        None
    }
}
