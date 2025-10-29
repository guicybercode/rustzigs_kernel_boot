use crate::vga;

pub mod raid;
pub mod journaling;
pub mod compression;
pub mod encryption;

pub struct AdvancedStorage {
    pub raid_enabled: bool,
    pub journaling_enabled: bool,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

impl AdvancedStorage {
    pub fn new() -> Self {
        Self {
            raid_enabled: false,
            journaling_enabled: false,
            compression_enabled: false,
            encryption_enabled: false,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Advanced Storage...\n");
        
        raid::init();
        journaling::init();
        compression::init();
        encryption::init();
        
        self.raid_enabled = true;
        self.journaling_enabled = true;
        self.compression_enabled = true;
        self.encryption_enabled = true;
        
        vga::print!("Advanced Storage initialized\n");
    }
}

pub static mut ADVANCED_STORAGE: AdvancedStorage = AdvancedStorage::new();

pub fn init() {
    unsafe {
        ADVANCED_STORAGE.init();
    }
}
