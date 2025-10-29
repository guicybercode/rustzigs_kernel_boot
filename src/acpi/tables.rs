#[repr(C, packed)]
pub struct ACPITableHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: [u8; 4],
    pub creator_revision: u32,
}

#[repr(C, packed)]
pub struct RSDT {
    pub header: ACPITableHeader,
    pub entries: [u32; 0],
}

pub fn parse_table_header(addr: u64) -> Option<&'static ACPITableHeader> {
    unsafe {
        let header = &*(addr as *const ACPITableHeader);
        
        let mut sum: u8 = 0;
        let bytes = core::slice::from_raw_parts(addr as *const u8, header.length as usize);
        for &byte in bytes {
            sum = sum.wrapping_add(byte);
        }
        
        if sum != 0 {
            return None;
        }
        
        Some(header)
    }
}

pub fn parse_rsdt(rsdt_addr: u32) -> Option<&'static RSDT> {
    unsafe {
        let rsdt = &*(rsdt_addr as *const RSDT);
        
        if &rsdt.header.signature != b"RSDT" {
            return None;
        }
        
        Some(rsdt)
    }
}
