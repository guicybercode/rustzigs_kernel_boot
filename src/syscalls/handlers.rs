use crate::vga;

pub fn init() {
    vga::print!("System call handlers initialized\n");
}

pub fn sys_exit(status: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Process exit with status: {}\n", status);
    0
}

pub fn sys_read(fd: u64, buf: u64, count: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Read from fd {}: {} bytes to 0x{:x}\n", fd, count, buf);
    count
}

pub fn sys_write(fd: u64, buf: u64, count: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Write to fd {}: {} bytes from 0x{:x}\n", fd, count, buf);
    count
}

pub fn sys_open(pathname: u64, flags: u64, mode: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Open file at 0x{:x}, flags: 0x{:x}, mode: 0x{:x}\n", pathname, flags, mode);
    1
}

pub fn sys_close(fd: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Close fd {}\n", fd);
    0
}

pub fn sys_mmap(addr: u64, length: u64, prot: u64, flags: u64, fd: u64) -> u64 {
    vga::print!("Mmap: addr=0x{:x}, len={}, prot=0x{:x}, flags=0x{:x}, fd={}\n", 
        addr, length, prot, flags, fd);
    addr
}

pub fn sys_munmap(addr: u64, length: u64, _arg3: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Munmap: addr=0x{:x}, len={}\n", addr, length);
    0
}

pub fn sys_fork(_arg1: u64, _arg2: u64, _arg3: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Fork system call\n");
    0
}

pub fn sys_exec(pathname: u64, argv: u64, envp: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Exec: path=0x{:x}, argv=0x{:x}, envp=0x{:x}\n", pathname, argv, envp);
    0
}

pub fn sys_wait(pid: u64, status: u64, options: u64, _arg4: u64, _arg5: u64) -> u64 {
    vga::print!("Wait: pid={}, status=0x{:x}, options=0x{:x}\n", pid, status, options);
    pid
}
