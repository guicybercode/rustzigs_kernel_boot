use crate::vga;
use crate::interrupts::InterruptFrame;
use crate::pci::devices;

extern "x86-interrupt" fn timer_handler(frame: InterruptFrame) -> ! {
    static mut TICK_COUNT: u64 = 0;
    
    unsafe {
        TICK_COUNT += 1;
        
        if TICK_COUNT % 1000 == 0 {
            vga::print!("Timer tick: {}\n", TICK_COUNT);
        }
    }
    
    crate::interrupts::pic::send_eoi(0);
    crate::interrupts::halt();
}

extern "x86-interrupt" fn keyboard_handler(frame: InterruptFrame) -> ! {
    devices::keyboard::handle_interrupt();
    crate::interrupts::pic::send_eoi(1);
    crate::interrupts::halt();
}

extern "x86-interrupt" fn mouse_handler(frame: InterruptFrame) -> ! {
    devices::mouse::handle_interrupt();
    crate::interrupts::pic::send_eoi(12);
    crate::interrupts::halt();
}

extern "x86-interrupt" fn storage_handler(frame: InterruptFrame) -> ! {
    devices::storage::handle_interrupt();
    crate::interrupts::pic::send_eoi(14);
    crate::interrupts::halt();
}

extern "x86-interrupt" fn network_handler(frame: InterruptFrame) -> ! {
    devices::network::handle_interrupt();
    crate::interrupts::pic::send_eoi(11);
    crate::interrupts::halt();
}

pub fn register_handlers() {
    crate::interrupts::register_handler(32, timer_handler);
    crate::interrupts::register_handler(33, keyboard_handler);
    crate::interrupts::register_handler(44, mouse_handler);
    crate::interrupts::register_handler(46, storage_handler);
    crate::interrupts::register_handler(43, network_handler);
}
