use crate::vga;

pub struct Framebuffer {
    pub address: u64,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub bytes_per_pixel: u32,
    pub bytes_per_scanline: u32,
}

impl Framebuffer {
    pub fn new(address: u64, width: u32, height: u32, bpp: u8) -> Self {
        let bytes_per_pixel = (bpp / 8) as u32;
        let bytes_per_scanline = width * bytes_per_pixel;
        
        Self {
            address,
            width,
            height,
            bpp,
            bytes_per_pixel,
            bytes_per_scanline,
        }
    }
    
    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let offset = (y * self.bytes_per_scanline + x * self.bytes_per_pixel) as u64;
        let pixel_address = self.address + offset;
        
        unsafe {
            match self.bpp {
                32 => {
                    *(pixel_address as *mut u32) = color;
                }
                24 => {
                    *(pixel_address as *mut u8) = (color & 0xFF) as u8;
                    *((pixel_address + 1) as *mut u8) = ((color >> 8) & 0xFF) as u8;
                    *((pixel_address + 2) as *mut u8) = ((color >> 16) & 0xFF) as u8;
                }
                16 => {
                    *(pixel_address as *mut u16) = (color & 0xFFFF) as u16;
                }
                8 => {
                    *(pixel_address as *mut u8) = (color & 0xFF) as u8;
                }
                _ => {}
            }
        }
    }
    
    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        
        let offset = (y * self.bytes_per_scanline + x * self.bytes_per_pixel) as u64;
        let pixel_address = self.address + offset;
        
        unsafe {
            match self.bpp {
                32 => *(pixel_address as *const u32),
                24 => {
                    let b = *(pixel_address as *const u8);
                    let g = *((pixel_address + 1) as *const u8);
                    let r = *((pixel_address + 2) as *const u8);
                    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
                }
                16 => *(pixel_address as *const u16) as u32,
                8 => *(pixel_address as *const u8) as u32,
                _ => 0,
            }
        }
    }
    
    pub fn clear(&mut self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }
    
    pub fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: u32) {
        let dx = (x2 as i32 - x1 as i32).abs() as u32;
        let dy = (y2 as i32 - y1 as i32).abs() as u32;
        
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        
        let mut err = dx as i32 - dy as i32;
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        
        loop {
            self.draw_pixel(x as u32, y as u32, color);
            
            if x == x2 as i32 && y == y2 as i32 {
                break;
            }
            
            let e2 = 2 * err;
            
            if e2 > -(dy as i32) {
                err -= dy as i32;
                x += sx;
            }
            
            if e2 < dx as i32 {
                err += dx as i32;
                y += sy;
            }
        }
    }
    
    pub fn draw_circle(&mut self, center_x: u32, center_y: u32, radius: u32, color: u32) {
        let mut x = 0i32;
        let mut y = radius as i32;
        let mut d = 1 - radius as i32;
        
        while x <= y {
            self.draw_pixel((center_x as i32 + x) as u32, (center_y as i32 + y) as u32, color);
            self.draw_pixel((center_x as i32 + y) as u32, (center_y as i32 + x) as u32, color);
            self.draw_pixel((center_x as i32 - x) as u32, (center_y as i32 + y) as u32, color);
            self.draw_pixel((center_x as i32 - y) as u32, (center_y as i32 + x) as u32, color);
            self.draw_pixel((center_x as i32 + x) as u32, (center_y as i32 - y) as u32, color);
            self.draw_pixel((center_y as i32 + y) as u32, (center_x as i32 - x) as u32, color);
            self.draw_pixel((center_x as i32 - x) as u32, (center_y as i32 - y) as u32, color);
            self.draw_pixel((center_x as i32 - y) as u32, (center_y as i32 - x) as u32, color);
            
            if d <= 0 {
                d += 2 * x + 3;
            } else {
                d += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }
}
