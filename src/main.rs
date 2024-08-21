// src/main.rs
mod framebuffer;
mod maploader;

use std::env;
use framebuffer::Framebuffer;
use maploader::load_maze;
use minifb::{Key, Window, WindowOptions};

fn draw_cell(framebuffer: &mut Framebuffer, x: usize, y: usize, block_size: usize, cell: char) {
    // Elegir color basándose en el carácter de la celda
    match cell {
        '+' | '-' | '|' => framebuffer.set_current_color(0x8B4513), // Color blanco para paredes
        'p' => framebuffer.set_current_color(0xFF0000),             // Color rojo para inicio
        'g' => framebuffer.set_current_color(0x00FF00),             // Color verde para meta
        _ => framebuffer.set_current_color(0xFFFF0F), // Fondo para espacios
    }

    // Dibujar un rectángulo para representar el bloque
    framebuffer.draw_rectangle(x, y, block_size, block_size);
}

fn render(framebuffer: &mut Framebuffer,  block_size: usize) {
    let maze = load_maze("./maze.txt");

    for (row, maze_row) in maze.iter().enumerate() {
        for (col, &cell) in maze_row.iter().enumerate() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, cell);
        }
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!("Directorio actual: {:?}", current_dir);
    // Cargar el laberinto desde un archivo
    let maze = load_maze("./maze.txt");

    // Determinar las dimensiones del framebuffer basado en el tamaño del laberinto
    let height = maze.len();
    let width = maze.iter().map(|line| line.len()).max().unwrap_or(0);

    // Tamaño del bloque
    let block_size = 50;

    // Crear un framebuffer con dimensiones ajustadas al laberinto
    let mut framebuffer = Framebuffer::new(width * block_size, height * block_size);

    // Renderizar el laberinto en el framebuffer
    render(&mut framebuffer, block_size);

    // Crear una ventana con minifb
    let mut window = Window::new(
        "Laberinto",
        width * block_size,
        height * block_size,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    // Mientras la ventana esté abierta y la tecla Escape no sea presionada
    while window.is_open() && !window.is_key_down(Key::A) {
        // Actualizar el buffer de la ventana
        window
            .update_with_buffer(framebuffer.get_buffer(), width * block_size, height * block_size)
            .unwrap();
    }
}
