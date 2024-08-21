// src/framebuffer.rs
#[derive(Clone)]
pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0x333355; width * height]; // Default background color
        let current_color = 0xFFDDDD;
        Framebuffer { width, height, buffer, current_color }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        self.set_color(x, y, self.current_color);
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
