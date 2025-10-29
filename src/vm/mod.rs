use crate::vga;
use core::sync::atomic::{AtomicU64, Ordering};

pub mod demand_paging;
pub mod swapping;
pub mod mmap;
pub mod protection;

#[repr(C, packed)]
pub struct VirtualMemoryRegion {
    pub start: u64,
    pub end: u64,
    pub permissions: u32,
    pub flags: u32,
    pub backing: Option<u64>,
}

pub struct VirtualMemoryManager {
    pub regions: [Option<VirtualMemoryRegion>; 1024],
    pub region_count: usize,
    pub total_pages: AtomicU64,
    pub used_pages: AtomicU64,
    pub swapped_pages: AtomicU64,
}

impl VirtualMemoryManager {
    pub fn new() -> Self {
        Self {
            regions: [None; 1024],
            region_count: 0,
            total_pages: AtomicU64::new(0),
            used_pages: AtomicU64::new(0),
            swapped_pages: AtomicU64::new(0),
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Virtual Memory Manager...\n");
        
        demand_paging::init();
        swapping::init();
        mmap::init();
        protection::init();
        
        vga::print!("Virtual Memory Manager initialized\n");
    }
    
    pub fn mmap(&mut self, addr: u64, len: u64, prot: u32, flags: u32) -> Option<u64> {
        if self.region_count >= 1024 {
            return None;
        }
        
        let region = VirtualMemoryRegion {
            start: addr,
            end: addr + len,
            permissions: prot,
            flags,
            backing: None,
        };
        
        for i in 0..1024 {
            if self.regions[i].is_none() {
                self.regions[i] = Some(region);
                self.region_count += 1;
                vga::print!("Mapped memory region: 0x{:x}-0x{:x}\n", addr, addr + len);
                return Some(addr);
            }
        }
        
        None
    }
    
    pub fn munmap(&mut self, addr: u64) -> bool {
        for i in 0..1024 {
            if let Some(region) = &self.regions[i] {
                if region.start == addr {
                    self.regions[i] = None;
                    self.region_count -= 1;
                    vga::print!("Unmapped memory region: 0x{:x}\n", addr);
                    return true;
                }
            }
        }
        false
    }
    
    pub fn handle_page_fault(&mut self, addr: u64, error_code: u64) -> bool {
        vga::print!("Page fault at 0x{:x}, error code: 0x{:x}\n", addr, error_code);
        
        if demand_paging::handle_page_fault(addr, error_code) {
            return true;
        }
        
        if swapping::handle_page_fault(addr, error_code) {
            return true;
        }
        
        false
    }
    
    pub fn get_memory_stats(&self) -> MemoryStats {
        MemoryStats {
            total_pages: self.total_pages.load(Ordering::Relaxed),
            used_pages: self.used_pages.load(Ordering::Relaxed),
            swapped_pages: self.swapped_pages.load(Ordering::Relaxed),
            region_count: self.region_count,
        }
    }
}

pub struct MemoryStats {
    pub total_pages: u64,
    pub used_pages: u64,
    pub swapped_pages: u64,
    pub region_count: usize,
}

pub static mut VM_MANAGER: VirtualMemoryManager = VirtualMemoryManager::new();

pub fn init() {
    unsafe {
        VM_MANAGER.init();
    }
}

pub fn mmap(addr: u64, len: u64, prot: u32, flags: u32) -> Option<u64> {
    unsafe {
        VM_MANAGER.mmap(addr, len, prot, flags)
    }
}

pub fn munmap(addr: u64) -> bool {
    unsafe {
        VM_MANAGER.munmap(addr)
    }
}

pub fn handle_page_fault(addr: u64, error_code: u64) -> bool {
    unsafe {
        VM_MANAGER.handle_page_fault(addr, error_code)
    }
}
