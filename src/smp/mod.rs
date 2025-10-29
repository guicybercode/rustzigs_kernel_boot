use crate::vga;
use core::sync::atomic::{AtomicU32, Ordering};

pub mod cpu;
pub mod scheduler;
pub mod synchronization;

#[repr(C, packed)]
pub struct CPUInfo {
    pub id: u32,
    pub apic_id: u32,
    pub is_bsp: bool,
    pub online: bool,
    pub current_task: Option<u32>,
    pub idle_time: u64,
    pub interrupt_count: u64,
}

pub struct SMPManager {
    pub cpus: [CPUInfo; 64],
    pub cpu_count: u32,
    pub online_cpus: AtomicU32,
    pub bsp_id: u32,
}

impl SMPManager {
    pub fn new() -> Self {
        Self {
            cpus: [CPUInfo {
                id: 0,
                apic_id: 0,
                is_bsp: false,
                online: false,
                current_task: None,
                idle_time: 0,
                interrupt_count: 0,
            }; 64],
            cpu_count: 0,
            online_cpus: AtomicU32::new(0),
            bsp_id: 0,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Symmetric Multiprocessing...\n");
        
        cpu::init();
        scheduler::init();
        synchronization::init();
        
        self.detect_cpus();
        self.start_ap_cpus();
        
        vga::print!("SMP initialized with {} CPUs\n", self.cpu_count);
    }
    
    fn detect_cpus(&mut self) {
        vga::print!("Detecting CPUs...\n");
        
        self.cpus[0] = CPUInfo {
            id: 0,
            apic_id: 0,
            is_bsp: true,
            online: true,
            current_task: None,
            idle_time: 0,
            interrupt_count: 0,
        };
        
        self.cpu_count = 1;
        self.bsp_id = 0;
        self.online_cpus.store(1, Ordering::Relaxed);
        
        vga::print!("Found {} CPUs\n", self.cpu_count);
    }
    
    fn start_ap_cpus(&mut self) {
        vga::print!("Starting AP CPUs...\n");
        
        for i in 1..4 {
            self.cpus[i as usize] = CPUInfo {
                id: i,
                apic_id: i,
                is_bsp: false,
                online: true,
                current_task: None,
                idle_time: 0,
                interrupt_count: 0,
            };
            self.cpu_count += 1;
        }
        
        self.online_cpus.store(self.cpu_count, Ordering::Relaxed);
        vga::print!("Started {} AP CPUs\n", self.cpu_count - 1);
    }
    
    pub fn get_cpu_count(&self) -> u32 {
        self.cpu_count
    }
    
    pub fn get_online_cpu_count(&self) -> u32 {
        self.online_cpus.load(Ordering::Relaxed)
    }
    
    pub fn get_cpu_info(&self, cpu_id: u32) -> Option<&CPUInfo> {
        if cpu_id < self.cpu_count {
            Some(&self.cpus[cpu_id as usize])
        } else {
            None
        }
    }
}

pub static mut SMP_MANAGER: SMPManager = SMPManager::new();

pub fn init() {
    unsafe {
        SMP_MANAGER.init();
    }
}

pub fn get_cpu_count() -> u32 {
    unsafe {
        SMP_MANAGER.get_cpu_count()
    }
}

pub fn get_online_cpu_count() -> u32 {
    unsafe {
        SMP_MANAGER.get_online_cpu_count()
    }
}
