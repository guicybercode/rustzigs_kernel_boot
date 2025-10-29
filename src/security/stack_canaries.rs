use crate::vga;

pub fn init() {
    vga::print!("Stack canaries initialized\n");
}

pub fn generate_canary() -> u64 {
    0xDEADBEEFCAFEBABE
}

pub fn check_canary(canary: u64) -> bool {
    canary == 0xDEADBEEFCAFEBABE
}
