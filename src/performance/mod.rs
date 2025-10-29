use crate::vga;
use core::sync::atomic::{AtomicU64, Ordering};

pub struct PerformanceMonitor {
    pub cpu_usage: AtomicU64,
    pub memory_usage: AtomicU64,
    pub task_count: AtomicU64,
    pub interrupt_count: AtomicU64,
    pub context_switches: AtomicU64,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            cpu_usage: AtomicU64::new(0),
            memory_usage: AtomicU64::new(0),
            task_count: AtomicU64::new(0),
            interrupt_count: AtomicU64::new(0),
            context_switches: AtomicU64::new(0),
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing performance monitoring...\n");
        
        vga::print!("Performance monitoring initialized\n");
    }
    
    pub fn update_cpu_usage(&self, usage: u64) {
        self.cpu_usage.store(usage, Ordering::Relaxed);
    }
    
    pub fn update_memory_usage(&self, usage: u64) {
        self.memory_usage.store(usage, Ordering::Relaxed);
    }
    
    pub fn increment_interrupts(&self) {
        self.interrupt_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn increment_context_switches(&self) {
        self.context_switches.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_stats(&self) -> PerformanceStats {
        PerformanceStats {
            cpu_usage: self.cpu_usage.load(Ordering::Relaxed),
            memory_usage: self.memory_usage.load(Ordering::Relaxed),
            task_count: self.task_count.load(Ordering::Relaxed),
            interrupt_count: self.interrupt_count.load(Ordering::Relaxed),
            context_switches: self.context_switches.load(Ordering::Relaxed),
        }
    }
}

pub struct PerformanceStats {
    pub cpu_usage: u64,
    pub memory_usage: u64,
    pub task_count: u64,
    pub interrupt_count: u64,
    pub context_switches: u64,
}

pub static mut PERFORMANCE: PerformanceMonitor = PerformanceMonitor::new();

pub fn init() {
    unsafe {
        PERFORMANCE.init();
    }
}

pub fn print_stats() {
    unsafe {
        let stats = PERFORMANCE.get_stats();
        vga::print!("Performance Stats:\n");
        vga::print!("  CPU Usage: {}%\n", stats.cpu_usage);
        vga::print!("  Memory Usage: {} bytes\n", stats.memory_usage);
        vga::print!("  Tasks: {}\n", stats.task_count);
        vga::print!("  Interrupts: {}\n", stats.interrupt_count);
        vga::print!("  Context Switches: {}\n", stats.context_switches);
    }
}
