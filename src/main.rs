mod framebuffer;
mod maploader;
mod cast_ray;
mod jugador;

use std::env;
use framebuffer::Framebuffer;
use maploader::load_maze;
use cast_ray::cast_ray;
use jugador::{Player, process_events};
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::vec2;

fn draw_cell(framebuffer: &mut Framebuffer, x: usize, y: usize, block_size: usize, cell: char) {
    match cell {
        '+' | '-' | '|' => framebuffer.set_current_color(0x8B4513), // Paredes
        'p' => framebuffer.set_current_color(0xFF0000),             // Inicio
        'g' => framebuffer.set_current_color(0x00FF00),             // Meta
        _ => framebuffer.set_current_color(0xFFFF0F),               // Espacio
    }
    framebuffer.draw_rectangle(x, y, block_size, block_size);
}

fn render(framebuffer: &mut Framebuffer, block_size: usize, player: &Player) {
    let maze = load_maze("./maze.txt");

    for (row, maze_row) in maze.iter().enumerate() {
        for (col, &cell) in maze_row.iter().enumerate() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, cell);
        }
    }

    // Dibujar la posición del jugador como un punto rojo
    framebuffer.set_current_color(0xFF0000);
    framebuffer.draw_rectangle(
        player.pos.x as usize,
        player.pos.y as usize,
        5,
        5,
    );

    // Dibujar los rayos
    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!("Directorio actual: {:?}", current_dir);
    
    let maze = load_maze("./maze.txt");
    let height = maze.len();
    let width = maze.iter().map(|line| line.len()).max().unwrap_or(0);
    let block_size = 50;
    
    let mut framebuffer = Framebuffer::new(width * block_size, height * block_size);
    
    // Inicializa la posición del jugador, ángulo y campo de visión
    let mut player = Player::new(vec2(100.0, 100.0), 0.0, 60.0);
    
    // Crear una ventana con minifb
    let mut window = Window::new(
        "Laberinto",
        width * block_size,
        height * block_size,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    while window.is_open() && !window.is_key_down(Key::O) {
        // Procesar eventos (movimiento del jugador)
        process_events(&window, &mut player, &maze);

        // Renderizar el laberinto, la posición del jugador y los rayos
        render(&mut framebuffer, block_size, &player);

        // Actualizar el buffer de la ventana
        window.update_with_buffer(framebuffer.get_buffer(), width * block_size, height * block_size).unwrap();
    }
}
