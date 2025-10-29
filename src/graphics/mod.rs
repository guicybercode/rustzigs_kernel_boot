use crate::vga;

pub mod vesa;
pub mod framebuffer;
pub mod font;

use framebuffer::Framebuffer;
use vesa::VESAMode;

pub struct GraphicsManager {
    framebuffer: Option<Framebuffer>,
    current_mode: Option<VESAMode>,
    width: u32,
    height: u32,
    bpp: u8,
}

impl GraphicsManager {
    pub fn new() -> Self {
        Self {
            framebuffer: None,
            current_mode: None,
            width: 0,
            height: 0,
            bpp: 0,
        }
    }
    
    pub fn init(&mut self) {
        vga::print!("Initializing graphics manager...\n");
        
        if let Some(mode) = vesa::find_best_mode() {
            if vesa::set_mode(mode) {
                self.framebuffer = Some(Framebuffer::new(
                    mode.framebuffer_address,
                    mode.width,
                    mode.height,
                    mode.bpp
                ));
                self.current_mode = Some(mode);
                self.width = mode.width;
                self.height = mode.height;
                self.bpp = mode.bpp;
                
                vga::print!("Graphics initialized: {}x{}x{}bpp\n", 
                    mode.width, mode.height, mode.bpp);
            } else {
                vga::print!("Failed to set graphics mode\n");
            }
        } else {
            vga::print!("No suitable graphics mode found\n");
        }
    }
    
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        if let Some(fb) = &mut self.framebuffer {
            fb.draw_pixel(x, y, color);
        }
    }
    
    pub fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        for py in y..y + height {
            for px in x..x + width {
                self.draw_pixel(px, py, color);
            }
        }
    }
    
    pub fn draw_char(&mut self, x: u32, y: u32, ch: char, color: u32, bg_color: u32) {
        if let Some(fb) = &mut self.framebuffer {
            font::draw_char(fb, x, y, ch, color, bg_color);
        }
    }
    
    pub fn draw_string(&mut self, x: u32, y: u32, text: &str, color: u32, bg_color: u32) {
        let mut cx = x;
        for ch in text.chars() {
            if ch == '\n' {
                cx = x;
                // Note: y increment would need to be handled by caller
            } else {
                self.draw_char(cx, y, ch, color, bg_color);
                cx += 8;
            }
        }
    }
    
    pub fn clear_screen(&mut self, color: u32) {
        if let Some(fb) = &mut self.framebuffer {
            fb.clear(color);
        }
    }
    
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

pub static mut GRAPHICS: GraphicsManager = GraphicsManager::new();

pub fn init() {
    unsafe {
        GRAPHICS.init();
    }
}

pub fn draw_pixel(x: u32, y: u32, color: u32) {
    unsafe {
        GRAPHICS.draw_pixel(x, y, color);
    }
}

pub fn draw_rectangle(x: u32, y: u32, width: u32, height: u32, color: u32) {
    unsafe {
        GRAPHICS.draw_rectangle(x, y, width, height, color);
    }
}

pub fn draw_string(x: u32, y: u32, text: &str, color: u32, bg_color: u32) {
    unsafe {
        GRAPHICS.draw_string(x, y, text, color, bg_color);
    }
}

pub fn clear_screen(color: u32) {
    unsafe {
        GRAPHICS.clear_screen(color);
    }
}
