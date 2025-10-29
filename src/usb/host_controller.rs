use crate::vga;

pub struct USBController {
    pub base_address: u64,
    pub controller_type: ControllerType,
}

pub enum ControllerType {
    UHCI,
    OHCI,
    EHCI,
    XHCI,
}

pub fn init() {
    vga::print!("USB host controller initialized\n");
}
