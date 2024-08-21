// src/framebuffer.rs
#[derive(Clone)]
pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer{ 
            width, 
            height, 
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_color(&mut self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index]
        }
        else {
            0x000000
        }
        
    }
    
    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }


    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for dy in 0..height {
            for dx in 0..width {
                self.point(x + dx, y + dy);
            }
        }
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }
}
