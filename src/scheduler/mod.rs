use crate::vga;
use core::sync::atomic::{AtomicUsize, Ordering};

pub mod task;
pub mod round_robin;

use task::Task;
use round_robin::RoundRobinScheduler;

pub struct Scheduler {
    current_task: AtomicUsize,
    scheduler: RoundRobinScheduler,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            current_task: AtomicUsize::new(0),
            scheduler: RoundRobinScheduler::new(),
        }
    }

    pub fn init(&mut self) {
        vga::print!("Initializing task scheduler...\n");
        
        self.scheduler.init();
        self.create_idle_task();
        
        vga::print!("Task scheduler initialized\n");
    }

    fn create_idle_task(&mut self) {
        let idle_task = Task::new_idle();
        self.scheduler.add_task(idle_task);
    }

    pub fn create_task(&mut self, entry_point: fn(), stack_size: usize) -> usize {
        let task = Task::new(entry_point, stack_size);
        let task_id = self.scheduler.add_task(task);
        vga::print!("Created task with ID: {}\n", task_id);
        task_id
    }

    pub fn schedule(&mut self) {
        let next_task = self.scheduler.get_next_task();
        if let Some(task_id) = next_task {
            self.current_task.store(task_id, Ordering::Relaxed);
            self.scheduler.switch_to_task(task_id);
        }
    }

    pub fn yield_cpu(&mut self) {
        self.scheduler.yield_current_task();
        self.schedule();
    }

    pub fn block_task(&mut self, task_id: usize) {
        self.scheduler.block_task(task_id);
    }

    pub fn unblock_task(&mut self, task_id: usize) {
        self.scheduler.unblock_task(task_id);
    }

    pub fn get_current_task_id(&self) -> usize {
        self.current_task.load(Ordering::Relaxed)
    }
}

pub static mut SCHEDULER: Scheduler = Scheduler::new();

pub fn init() {
    unsafe {
        SCHEDULER.init();
    }
}

pub fn create_task(entry_point: fn(), stack_size: usize) -> usize {
    unsafe {
        SCHEDULER.create_task(entry_point, stack_size)
    }
}

pub fn yield_cpu() {
    unsafe {
        SCHEDULER.yield_cpu();
    }
}

pub fn block_task(task_id: usize) {
    unsafe {
        SCHEDULER.block_task(task_id);
    }
}

pub fn unblock_task(task_id: usize) {
    unsafe {
        SCHEDULER.unblock_task(task_id);
    }
}
