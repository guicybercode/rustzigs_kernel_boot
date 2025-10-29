use crate::vga;

pub fn init() {
    vga::print!("ASLR (Address Space Layout Randomization) initialized\n");
}

pub fn randomize_address(base: u64) -> u64 {
    let offset = (base % 0x1000) as u64;
    base + offset
}
