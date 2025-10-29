use core::arch::asm;

#[repr(C, packed)]
pub struct TaskContext {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct Task {
    pub id: usize,
    pub context: TaskContext,
    pub state: TaskState,
    pub stack: *mut u8,
    pub stack_size: usize,
    pub priority: u8,
    pub quantum: u32,
    pub total_runtime: u64,
}

impl Task {
    pub fn new(entry_point: fn(), stack_size: usize) -> Self {
        let stack = unsafe {
            crate::memory::heap::ALLOCATOR.alloc(
                core::alloc::Layout::from_size_align(stack_size, 16).unwrap()
            )
        };
        
        if stack.is_null() {
            panic!("Failed to allocate stack for task");
        }
        
        let stack_top = unsafe { stack.add(stack_size) };
        
        let mut context = TaskContext {
            r15: 0, r14: 0, r13: 0, r12: 0, r11: 0, r10: 0, r9: 0, r8: 0,
            rdi: 0, rsi: 0, rbp: 0, rdx: 0, rcx: 0, rbx: 0, rax: 0,
            rip: entry_point as u64,
            cs: 0x08,
            rflags: 0x202,
            rsp: stack_top as u64,
            ss: 0x10,
        };
        
        unsafe {
            *(stack_top as *mut u64).offset(-1) = 0;
            *(stack_top as *mut u64).offset(-2) = entry_point as u64;
            context.rsp = (stack_top as *mut u64).offset(-2) as u64;
        }
        
        Self {
            id: 0,
            context,
            state: TaskState::Ready,
            stack,
            stack_size,
            priority: 5,
            quantum: 10,
            total_runtime: 0,
        }
    }
    
    pub fn new_idle() -> Self {
        let stack_size = 4096;
        let stack = unsafe {
            crate::memory::heap::ALLOCATOR.alloc(
                core::alloc::Layout::from_size_align(stack_size, 16).unwrap()
            )
        };
        
        if stack.is_null() {
            panic!("Failed to allocate stack for idle task");
        }
        
        let stack_top = unsafe { stack.add(stack_size) };
        
        let context = TaskContext {
            r15: 0, r14: 0, r13: 0, r12: 0, r11: 0, r10: 0, r9: 0, r8: 0,
            rdi: 0, rsi: 0, rbp: 0, rdx: 0, rcx: 0, rbx: 0, rax: 0,
            rip: idle_loop as u64,
            cs: 0x08,
            rflags: 0x202,
            rsp: stack_top as u64,
            ss: 0x10,
        };
        
        Self {
            id: 0,
            context,
            state: TaskState::Running,
            stack,
            stack_size,
            priority: 0,
            quantum: 0,
            total_runtime: 0,
        }
    }
    
    pub fn switch_to(&mut self, other: &mut Task) {
        unsafe {
            asm!(
                "push rbp",
                "mov rbp, rsp",
                "push rax",
                "push rbx",
                "push rcx",
                "push rdx",
                "push rsi",
                "push rdi",
                "push r8",
                "push r9",
                "push r10",
                "push r11",
                "push r12",
                "push r13",
                "push r14",
                "push r15",
                "mov [rdi], rsp",
                "mov rsp, [rsi]",
                "pop r15",
                "pop r14",
                "pop r13",
                "pop r12",
                "pop r11",
                "pop r10",
                "pop r9",
                "pop r8",
                "pop rdi",
                "pop rsi",
                "pop rdx",
                "pop rcx",
                "pop rbx",
                "pop rax",
                "pop rbp",
                in("rdi") &mut self.context.rsp,
                in("rsi") &other.context.rsp,
            );
        }
    }
    
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    
    pub fn is_ready(&self) -> bool {
        self.state == TaskState::Ready
    }
    
    pub fn is_blocked(&self) -> bool {
        self.state == TaskState::Blocked
    }
    
    pub fn is_terminated(&self) -> bool {
        self.state == TaskState::Terminated
    }
}

fn idle_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
