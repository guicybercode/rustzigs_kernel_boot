use crate::vga;

pub fn init() {
    vga::print!("Journaling filesystem initialized\n");
}
