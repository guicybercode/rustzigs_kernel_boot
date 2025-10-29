use crate::vga;

pub fn init() {
    vga::print!("Demand paging initialized\n");
}

pub fn handle_page_fault(addr: u64, error_code: u64) -> bool {
    vga::print!("Handling demand page fault at 0x{:x}\n", addr);
    
    let page_addr = addr & !0xFFF;
    
    if allocate_page(page_addr) {
        vga::print!("Allocated page for 0x{:x}\n", page_addr);
        true
    } else {
        vga::print!("Failed to allocate page for 0x{:x}\n", page_addr);
        false
    }
}

fn allocate_page(addr: u64) -> bool {
    vga::print!("Allocating page at 0x{:x}\n", addr);
    true
}
