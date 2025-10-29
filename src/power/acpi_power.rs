use crate::vga;

pub fn init() {
    vga::print!("ACPI power management initialized\n");
}
