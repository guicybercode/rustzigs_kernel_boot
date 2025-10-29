use crate::vga;
use crate::filesystem::fat32::FAT32FileSystem;

pub struct File {
    pub id: usize,
    pub name: String,
    pub size: u32,
    pub position: u32,
    pub device_id: usize,
    pub cluster: u32,
}

pub struct MountPoint {
    pub device_id: usize,
    pub path: String,
    pub filesystem: FAT32FileSystem,
}

pub struct VFS {
    files: [Option<File>; 32],
    file_count: usize,
    next_file_id: usize,
    mount_points: [Option<MountPoint>; 8],
    mount_count: usize,
}

impl VFS {
    pub fn new() -> Self {
        Self {
            files: [None; 32],
            file_count: 0,
            next_file_id: 1,
            mount_points: [None; 8],
            mount_count: 0,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("VFS initialized\n");
    }
    
    pub fn mount(&mut self, device_id: usize, mount_point: &str) -> bool {
        if self.mount_count >= 8 {
            vga::print!("Maximum mount points reached\n");
            return false;
        }
        
        let mut filesystem = FAT32FileSystem::new();
        if !filesystem.init(device_id) {
            vga::print!("Failed to initialize filesystem on device {}\n", device_id);
            return false;
        }
        
        let mount = MountPoint {
            device_id,
            path: mount_point.to_string(),
            filesystem,
        };
        
        for i in 0..8 {
            if self.mount_points[i].is_none() {
                self.mount_points[i] = Some(mount);
                self.mount_count += 1;
                vga::print!("Mounted device {} at {}\n", device_id, mount_point);
                return true;
            }
        }
        
        false
    }
    
    pub fn open(&mut self, path: &str) -> Option<usize> {
        if self.file_count >= 32 {
            vga::print!("Maximum files opened\n");
            return None;
        }
        
        let mount_point = self.find_mount_point(path)?;
        let relative_path = &path[mount_point.path.len()..];
        
        let file = File {
            id: self.next_file_id,
            name: relative_path.to_string(),
            size: 0,
            position: 0,
            device_id: mount_point.device_id,
            cluster: 2,
        };
        
        for i in 0..32 {
            if self.files[i].is_none() {
                self.files[i] = Some(file);
                self.file_count += 1;
                self.next_file_id += 1;
                vga::print!("Opened file: {}\n", path);
                return Some(self.next_file_id - 1);
            }
        }
        
        None
    }
    
    pub fn read(&mut self, file_id: usize, buffer: &mut [u8]) -> usize {
        for i in 0..32 {
            if let Some(file) = &mut self.files[i] {
                if file.id == file_id {
                    let mount_point = self.get_mount_point(file.device_id)?;
                    let bytes_read = mount_point.filesystem.read_file(
                        file.device_id,
                        file.cluster,
                        file.size - file.position,
                        buffer
                    );
                    file.position += bytes_read as u32;
                    return bytes_read;
                }
            }
        }
        0
    }
    
    pub fn write(&mut self, file_id: usize, data: &[u8]) -> usize {
        vga::print!("File write not implemented\n");
        0
    }
    
    pub fn close(&mut self, file_id: usize) {
        for i in 0..32 {
            if let Some(file) = &self.files[i] {
                if file.id == file_id {
                    self.files[i] = None;
                    self.file_count -= 1;
                    vga::print!("Closed file: {}\n", file_id);
                    break;
                }
            }
        }
    }
    
    pub fn list_directory(&mut self, path: &str) -> Vec<String> {
        let mut entries = Vec::new();
        entries.push(".".to_string());
        entries.push("..".to_string());
        entries
    }
    
    fn find_mount_point(&self, path: &str) -> Option<&MountPoint> {
        for mount in &self.mount_points {
            if let Some(mount) = mount {
                if path.starts_with(&mount.path) {
                    return Some(mount);
                }
            }
        }
        None
    }
    
    fn get_mount_point(&mut self, device_id: usize) -> Option<&mut MountPoint> {
        for mount in &mut self.mount_points {
            if let Some(mount) = mount {
                if mount.device_id == device_id {
                    return Some(mount);
                }
            }
        }
        None
    }
}
