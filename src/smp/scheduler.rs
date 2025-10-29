use crate::vga;

pub fn init() {
    vga::print!("SMP scheduler initialized\n");
}
