use crate::vga;

pub mod interface;
pub mod handlers;

pub struct SystemCallManager {
    pub handlers: [Option<fn(u64, u64, u64, u64, u64) -> u64>; 256],
}

impl SystemCallManager {
    pub fn new() -> Self {
        Self {
            handlers: [None; 256],
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing System Call Interface...\n");
        
        interface::init();
        handlers::init();
        
        self.register_handlers();
        
        vga::print!("System Call Interface initialized\n");
    }
    
    fn register_handlers(&mut self) {
        self.handlers[0] = Some(handlers::sys_exit);
        self.handlers[1] = Some(handlers::sys_read);
        self.handlers[2] = Some(handlers::sys_write);
        self.handlers[3] = Some(handlers::sys_open);
        self.handlers[4] = Some(handlers::sys_close);
        self.handlers[5] = Some(handlers::sys_mmap);
        self.handlers[6] = Some(handlers::sys_munmap);
        self.handlers[7] = Some(handlers::sys_fork);
        self.handlers[8] = Some(handlers::sys_exec);
        self.handlers[9] = Some(handlers::sys_wait);
        
        vga::print!("Registered {} system call handlers\n", 10);
    }
    
    pub fn handle_syscall(&self, syscall_num: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
        if syscall_num < 256 {
            if let Some(handler) = self.handlers[syscall_num as usize] {
                handler(arg1, arg2, arg3, arg4, arg5)
            } else {
                vga::print!("Unknown system call: {}\n", syscall_num);
                0xFFFFFFFFFFFFFFFF
            }
        } else {
            vga::print!("Invalid system call number: {}\n", syscall_num);
            0xFFFFFFFFFFFFFFFF
        }
    }
}

pub static mut SYSCALL_MANAGER: SystemCallManager = SystemCallManager::new();

pub fn init() {
    unsafe {
        SYSCALL_MANAGER.init();
    }
}

pub fn handle_syscall(syscall_num: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
    unsafe {
        SYSCALL_MANAGER.handle_syscall(syscall_num, arg1, arg2, arg3, arg4, arg5)
    }
}
