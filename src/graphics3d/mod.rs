use crate::vga;

pub mod gpu;
pub mod rendering;
pub mod shaders;
pub mod models;

pub struct Graphics3D {
    pub gpu_initialized: bool,
    pub shader_support: bool,
    pub texture_support: bool,
    pub vertex_count: u32,
    pub triangle_count: u32,
}

impl Graphics3D {
    pub fn new() -> Self {
        Self {
            gpu_initialized: false,
            shader_support: false,
            texture_support: false,
            vertex_count: 0,
            triangle_count: 0,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing 3D Graphics System...\n");
        
        gpu::init();
        rendering::init();
        shaders::init();
        models::init();
        
        self.gpu_initialized = true;
        self.shader_support = true;
        self.texture_support = true;
        
        vga::print!("3D Graphics System initialized\n");
    }
    
    pub fn render_triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.triangle_count += 1;
        vga::print!("Rendered triangle: ({}, {}) ({}, {}) ({}, {})\n", x1, y1, x2, y2, x3, y3);
    }
    
    pub fn render_vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertex_count += 1;
        vga::print!("Rendered vertex: ({}, {}, {})\n", x, y, z);
    }
}

pub static mut GRAPHICS_3D: Graphics3D = Graphics3D::new();

pub fn init() {
    unsafe {
        GRAPHICS_3D.init();
    }
}
