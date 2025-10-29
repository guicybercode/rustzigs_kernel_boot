use crate::vga;

pub mod fat32;
pub mod vfs;

use vfs::VFS;

pub struct FileSystem {
    vfs: VFS,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            vfs: VFS::new(),
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing filesystem...\n");
        
        self.vfs.init();
        
        vga::print!("Filesystem initialized\n");
    }
    
    pub fn mount(&mut self, device_id: usize, mount_point: &str) -> bool {
        self.vfs.mount(device_id, mount_point)
    }
    
    pub fn open(&mut self, path: &str) -> Option<usize> {
        self.vfs.open(path)
    }
    
    pub fn read(&mut self, file_id: usize, buffer: &mut [u8]) -> usize {
        self.vfs.read(file_id, buffer)
    }
    
    pub fn write(&mut self, file_id: usize, data: &[u8]) -> usize {
        self.vfs.write(file_id, data)
    }
    
    pub fn close(&mut self, file_id: usize) {
        self.vfs.close(file_id);
    }
    
    pub fn list_directory(&mut self, path: &str) -> Vec<String> {
        self.vfs.list_directory(path)
    }
}

pub static mut FILESYSTEM: FileSystem = FileSystem::new();

pub fn init() {
    unsafe {
        FILESYSTEM.init();
    }
}

pub fn mount(device_id: usize, mount_point: &str) -> bool {
    unsafe {
        FILESYSTEM.mount(device_id, mount_point)
    }
}

pub fn open_file(path: &str) -> Option<usize> {
    unsafe {
        FILESYSTEM.open(path)
    }
}

pub fn read_file(file_id: usize, buffer: &mut [u8]) -> usize {
    unsafe {
        FILESYSTEM.read(file_id, buffer)
    }
}

pub fn write_file(file_id: usize, data: &[u8]) -> usize {
    unsafe {
        FILESYSTEM.write(file_id, data)
    }
}

pub fn close_file(file_id: usize) {
    unsafe {
        FILESYSTEM.close(file_id);
    }
}
