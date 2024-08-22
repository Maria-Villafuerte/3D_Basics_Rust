mod framebuffer;
mod maploader;
mod cast_ray;
mod jugador;

use std::env;
use std::time::Duration;
use std::thread::sleep;
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

fn render2d(framebuffer: &mut Framebuffer, block_size: usize, player: &Player) {
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

fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>) {
    let num_rays = framebuffer.width; // Un rayo por cada columna de píxeles
    let hh = framebuffer.height as f32 / 2.0;
    let fov_adjustment = player.fov / framebuffer.width as f32;
    let distance_to_projection_plane = (framebuffer.width as f32 / 2.0) / (player.fov / 2.0).tan();

    // Dibujar cielo y suelo
    let sky_color = 0x87CEEB;
    let ground_color = 0x006400;

    framebuffer.set_current_color(sky_color);
    framebuffer.draw_rectangle(0, 0, framebuffer.width, framebuffer.height / 2);

    framebuffer.set_current_color(ground_color);
    framebuffer.draw_rectangle(0, framebuffer.height / 2, framebuffer.width, framebuffer.height / 2);

    // Dibujar las paredes
    for i in 0..num_rays {
        let ray_angle = player.a - (player.fov / 2.0) + (i as f32 * fov_adjustment);
        let intersect = cast_ray(framebuffer, maze, player, ray_angle, 50, false);

        let distance = intersect.distance * (ray_angle - player.a).cos(); // Corrección del efecto "fish-eye"
        let wall_height = (framebuffer.height as f32 / distance) * distance_to_projection_plane;

        let wall_top = (hh - wall_height / 2.0).max(0.0) as usize;
        let wall_bottom = (hh + wall_height / 2.0).min(framebuffer.height as f32) as usize;

        framebuffer.set_current_color(0xFF0000); // Color de la pared
        for y in wall_top..wall_bottom {
            framebuffer.point(i, y);
        }
    }
}

fn main() {
    let current_dir = env::current_dir().unwrap();
    println!("Directorio actual: {:?}", current_dir);
    let frame_delay = Duration::from_millis(16);
    let maze: Vec<Vec<char>> = load_maze("./maze.txt");
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

    let mut mode = "2D";

    while window.is_open() && !window.is_key_down(Key::O) {
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
            sleep(Duration::from_millis(200)); // Evitar cambio rápido
        }

        process_events(&window, &mut player, &maze);
        framebuffer.clear();

        if mode == "2D" {
            render2d(&mut framebuffer, block_size, &player);
        } else {
            render3d(&mut framebuffer, &player, &maze);
        }

        window.update_with_buffer(framebuffer.get_buffer(), framebuffer.width, framebuffer.height)
            .unwrap();
        sleep(frame_delay);
    }
}
