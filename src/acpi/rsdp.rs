#[repr(C, packed)]
pub struct RSDP {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

pub fn parse_rsdp(addr: u64) -> Option<&'static RSDP> {
    unsafe {
        let rsdp = &*(addr as *const RSDP);
        
        if &rsdp.signature != b"RSD PTR " {
            return None;
        }
        
        let mut sum: u8 = 0;
        let bytes = core::slice::from_raw_parts(addr as *const u8, 20);
        for &byte in bytes {
            sum = sum.wrapping_add(byte);
        }
        
        if sum != 0 {
            return None;
        }
        
        Some(rsdp)
    }
}
