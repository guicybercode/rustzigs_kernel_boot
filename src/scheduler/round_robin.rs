use crate::scheduler::task::{Task, TaskState};
use crate::vga;

pub struct RoundRobinScheduler {
    tasks: [Option<Task>; 64],
    task_count: usize,
    current_index: usize,
    next_id: usize,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self {
            tasks: [None; 64],
            task_count: 0,
            current_index: 0,
            next_id: 1,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Round-robin scheduler initialized\n");
    }
    
    pub fn add_task(&mut self, mut task: Task) -> usize {
        if self.task_count >= 64 {
            vga::print!("Maximum number of tasks reached\n");
            return 0;
        }
        
        let task_id = self.next_id;
        task.set_id(task_id);
        self.next_id += 1;
        
        for i in 0..64 {
            if self.tasks[i].is_none() {
                self.tasks[i] = Some(task);
                self.task_count += 1;
                vga::print!("Added task {} to scheduler\n", task_id);
                return task_id;
            }
        }
        
        0
    }
    
    pub fn get_next_task(&mut self) -> Option<usize> {
        if self.task_count == 0 {
            return None;
        }
        
        let start_index = self.current_index;
        
        loop {
            if let Some(task) = &self.tasks[self.current_index] {
                if task.is_ready() {
                    let task_id = task.id;
                    self.current_index = (self.current_index + 1) % 64;
                    return Some(task_id);
                }
            }
            
            self.current_index = (self.current_index + 1) % 64;
            
            if self.current_index == start_index {
                break;
            }
        }
        
        None
    }
    
    pub fn switch_to_task(&mut self, task_id: usize) {
        for i in 0..64 {
            if let Some(task) = &mut self.tasks[i] {
                if task.id == task_id {
                    task.state = TaskState::Running;
                    return;
                }
            }
        }
    }
    
    pub fn yield_current_task(&mut self) {
        for i in 0..64 {
            if let Some(task) = &mut self.tasks[i] {
                if task.state == TaskState::Running {
                    task.state = TaskState::Ready;
                    break;
                }
            }
        }
    }
    
    pub fn block_task(&mut self, task_id: usize) {
        for i in 0..64 {
            if let Some(task) = &mut self.tasks[i] {
                if task.id == task_id {
                    task.state = TaskState::Blocked;
                    vga::print!("Task {} blocked\n", task_id);
                    break;
                }
            }
        }
    }
    
    pub fn unblock_task(&mut self, task_id: usize) {
        for i in 0..64 {
            if let Some(task) = &mut self.tasks[i] {
                if task.id == task_id {
                    task.state = TaskState::Ready;
                    vga::print!("Task {} unblocked\n", task_id);
                    break;
                }
            }
        }
    }
    
    pub fn terminate_task(&mut self, task_id: usize) {
        for i in 0..64 {
            if let Some(task) = &mut self.tasks[i] {
                if task.id == task_id {
                    task.state = TaskState::Terminated;
                    self.task_count -= 1;
                    vga::print!("Task {} terminated\n", task_id);
                    break;
                }
            }
        }
    }
    
    pub fn get_task_count(&self) -> usize {
        self.task_count
    }
}
