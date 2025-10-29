use crate::vga;

pub mod rsdp;
pub mod tables;

static mut ACPI_RSDP: u64 = 0;

pub fn init(rsdp_addr: u64) {
    unsafe {
        ACPI_RSDP = rsdp_addr;
        vga::print!("ACPI RSDP at: 0x{:x}\n", rsdp_addr);
        
        if let Some(rsdp) = rsdp::parse_rsdp(rsdp_addr) {
            vga::print!("ACPI RSDP signature: {}\n", rsdp.signature);
            vga::print!("ACPI revision: {}\n", rsdp.revision);
            
            if let Some(rsdt) = tables::parse_rsdt(rsdp.rsdt_address) {
                vga::print!("ACPI RSDT parsed successfully\n");
                parse_acpi_tables(rsdt);
            }
        }
    }
}

fn parse_acpi_tables(rsdt: &tables::RSDT) {
    for i in 0..rsdt.entry_count {
        let entry_addr = rsdt.entries[i as usize];
        if let Some(header) = tables::parse_table_header(entry_addr) {
            match &header.signature {
                b"MADT" => {
                    vga::print!("Found MADT table\n");
                }
                b"FADT" => {
                    vga::print!("Found FADT table\n");
                }
                b"MCFG" => {
                    vga::print!("Found MCFG table\n");
                }
                _ => {}
            }
        }
    }
}
