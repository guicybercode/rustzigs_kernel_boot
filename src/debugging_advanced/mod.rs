use crate::vga;

pub mod profiling;
pub mod tracing;
pub mod analysis;
pub mod hotpatch;

pub struct AdvancedDebugging {
    pub profiling_enabled: bool,
    pub tracing_enabled: bool,
    pub analysis_enabled: bool,
    pub hotpatch_enabled: bool,
}

impl AdvancedDebugging {
    pub fn new() -> Self {
        Self {
            profiling_enabled: false,
            tracing_enabled: false,
            analysis_enabled: false,
            hotpatch_enabled: false,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing Advanced Debugging...\n");
        
        profiling::init();
        tracing::init();
        analysis::init();
        hotpatch::init();
        
        self.profiling_enabled = true;
        self.tracing_enabled = true;
        self.analysis_enabled = true;
        self.hotpatch_enabled = true;
        
        vga::print!("Advanced Debugging initialized\n");
    }
}

pub static mut ADVANCED_DEBUGGING: AdvancedDebugging = AdvancedDebugging::new();

pub fn init() {
    unsafe {
        ADVANCED_DEBUGGING.init();
    }
}
