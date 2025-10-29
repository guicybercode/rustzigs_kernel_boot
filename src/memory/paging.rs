use core::arch::asm;

const PAGE_SIZE: u64 = 4096;
const PAGE_PRESENT: u64 = 1 << 0;
const PAGE_WRITABLE: u64 = 1 << 1;
const PAGE_USER: u64 = 1 << 2;
const PAGE_SIZE_2MB: u64 = 1 << 7;
const PAGE_NX: u64 = 1 << 63;

pub struct PageTable {
    entries: [u64; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        Self { entries: [0; 512] }
    }
}

static mut PML4: PageTable = PageTable::new();
static mut PDPT: PageTable = PageTable::new();
static mut PD: [PageTable; 4] = [PageTable::new(); 4];

pub fn init() {
    unsafe {
        PML4.entries[0] = (&PDPT as *const PageTable as u64) | PAGE_PRESENT | PAGE_WRITABLE;
        
        for i in 0..4 {
            PD[i].entries[0] = 0x00000000 | PAGE_PRESENT | PAGE_WRITABLE | PAGE_SIZE_2MB;
            PD[i].entries[1] = 0x20000000 | PAGE_PRESENT | PAGE_WRITABLE | PAGE_SIZE_2MB;
            PD[i].entries[2] = 0x40000000 | PAGE_PRESENT | PAGE_WRITABLE | PAGE_SIZE_2MB;
            PD[i].entries[3] = 0x60000000 | PAGE_PRESENT | PAGE_WRITABLE | PAGE_SIZE_2MB;
            
            PDPT.entries[i] = (&PD[i] as *const PageTable as u64) | PAGE_PRESENT | PAGE_WRITABLE;
        }
        
        load_cr3(&PML4 as *const PageTable as u64);
    }
}

fn load_cr3(addr: u64) {
    unsafe {
        asm!("mov {}, %cr3", in(reg) addr);
    }
}

pub fn map_page(virtual_addr: u64, physical_addr: u64, flags: u64) {
    let pml4_index = (virtual_addr >> 39) & 0x1FF;
    let pdpt_index = (virtual_addr >> 30) & 0x1FF;
    let pd_index = (virtual_addr >> 21) & 0x1FF;
    let pt_index = (virtual_addr >> 12) & 0x1FF;
    
    unsafe {
        if PML4.entries[pml4_index as usize] == 0 {
            PML4.entries[pml4_index as usize] = allocate_page() | PAGE_PRESENT | PAGE_WRITABLE;
        }
        
        let pdpt = &mut *((PML4.entries[pml4_index as usize] & !0xFFF) as *mut PageTable);
        if pdpt.entries[pdpt_index as usize] == 0 {
            pdpt.entries[pdpt_index as usize] = allocate_page() | PAGE_PRESENT | PAGE_WRITABLE;
        }
        
        let pd = &mut *((pdpt.entries[pdpt_index as usize] & !0xFFF) as *mut PageTable);
        if pd.entries[pd_index as usize] == 0 {
            pd.entries[pd_index as usize] = allocate_page() | PAGE_PRESENT | PAGE_WRITABLE;
        }
        
        let pt = &mut *((pd.entries[pd_index as usize] & !0xFFF) as *mut PageTable);
        pt.entries[pt_index as usize] = physical_addr | flags | PAGE_PRESENT;
    }
}

fn allocate_page() -> u64 {
    0x1000000
}
