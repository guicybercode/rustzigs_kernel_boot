use crate::vga;

pub mod gdb;
pub mod symbols;
pub mod breakpoints;

pub struct Debugger {
    pub gdb_enabled: bool,
    pub breakpoints: [Option<u64>; 32],
    pub breakpoint_count: usize,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            gdb_enabled: false,
            breakpoints: [None; 32],
            breakpoint_count: 0,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing debugging system...\n");
        
        gdb::init();
        symbols::init();
        breakpoints::init();
        
        vga::print!("Debugging system initialized\n");
    }
    
    pub fn set_breakpoint(&mut self, address: u64) -> bool {
        if self.breakpoint_count >= 32 {
            return false;
        }
        
        for i in 0..32 {
            if self.breakpoints[i].is_none() {
                self.breakpoints[i] = Some(address);
                self.breakpoint_count += 1;
                vga::print!("Breakpoint set at 0x{:x}\n", address);
                return true;
            }
        }
        
        false
    }
    
    pub fn remove_breakpoint(&mut self, address: u64) -> bool {
        for i in 0..32 {
            if self.breakpoints[i] == Some(address) {
                self.breakpoints[i] = None;
                self.breakpoint_count -= 1;
                vga::print!("Breakpoint removed at 0x{:x}\n", address);
                return true;
            }
        }
        
        false
    }
}

pub static mut DEBUGGER: Debugger = Debugger::new();

pub fn init() {
    unsafe {
        DEBUGGER.init();
    }
}
