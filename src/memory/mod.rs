use crate::HandoffData;

pub mod paging;
pub mod heap;

static mut MEMORY_MAP: [crate::MemoryMapEntry; 256] = [crate::MemoryMapEntry { base: 0, length: 0, type_: 0 }; 256];

pub fn init(handoff: *const HandoffData) {
    unsafe {
        MEMORY_MAP = handoff.memory_map;
        paging::init();
        heap::init();
    }
}

pub fn get_memory_map() -> &'static [crate::MemoryMapEntry] {
    unsafe { &MEMORY_MAP }
}
