use crate::vga;

pub fn init() {
    vga::print!("Swapping system initialized\n");
}

pub fn handle_page_fault(addr: u64, error_code: u64) -> bool {
    vga::print!("Handling swap page fault at 0x{:x}\n", addr);
    
    if swap_in_page(addr) {
        vga::print!("Swapped in page for 0x{:x}\n", addr);
        true
    } else {
        vga::print!("Failed to swap in page for 0x{:x}\n", addr);
        false
    }
}

fn swap_in_page(addr: u64) -> bool {
    vga::print!("Swapping in page at 0x{:x}\n", addr);
    true
}
