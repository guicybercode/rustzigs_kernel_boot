use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
        }
    }
    
    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.next, layout.align());
        let alloc_end = alloc_start + layout.size();
        
        if alloc_end > self.heap_end {
            return core::ptr::null_mut();
        }
        
        (alloc_start as *mut u8)
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub static mut ALLOCATOR: BumpAllocator = BumpAllocator::new();

pub fn init() {
    unsafe {
        ALLOCATOR.init(0x2000000, 0x1000000);
    }
}

#[global_allocator]
static ALLOCATOR_REF: BumpAllocator = unsafe { core::mem::transmute(ALLOCATOR) };
