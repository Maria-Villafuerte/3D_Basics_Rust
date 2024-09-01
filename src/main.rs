mod maze;
use maze::load_maze;

mod framebuffer;
use framebuffer::Framebuffer;

mod player;
use player::Player;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use splash_screen:: show_start_screen  ;
use core::f32::consts::PI;

use nalgebra_glm::{Vec2, distance};
use std::time::Duration;

use once_cell::sync::Lazy;
use std::sync::Arc; 

mod caster;
use caster::{cast_ray, cast_ray_minimap};

mod texture;
use texture::Texture;

mod splash_screen;

mod audio;

mod textrender;
use textrender::TextRenderer; 

use std::time::Instant;

use gilrs::Gilrs;


// Imagenes del juego
static SKY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("src\\assets\\sky1.png")));
static FLOOR: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("src\\assets\\floor.png")));
static WALL: Lazy<Arc<Texture>> = Lazy::new(||  Arc::new(Texture::new("src\\assets\\wall.png")));
static MONSTER: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("src\\assets\\monster.png")));

fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {
    let default_color = 0x000000;

    match cell {
        '+' | '-' | '|' | 'g' => WALL.get_pixel_color(tx, ty),
        _ => default_color,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '-' | '|' | 'g' => WALL.get_pixel_color(0, 0),                                
        _ => 0x717171,                                  
    };

    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, z_buffer: &mut [f32]){

    let block_size = 100; 
    let hh =  framebuffer.height as f32/2.0;
    let num_rays = framebuffer.width; 

     // Pre-calculate the sky texture offset based on player's angle
     let sky_offset = ((player.a / (2.0 * std::f32::consts::PI)) * SKY.width as f32) as u32;

     for i in 0..framebuffer.width {
         // Render textured sky
         for j in 0..(framebuffer.height / 2) {
             let texture_x = (sky_offset + i as u32) % SKY.width;
             let texture_y = (j as f32 / (framebuffer.height / 2) as f32 * SKY.height as f32) as u32;
             let color = SKY.get_pixel_color(texture_x, texture_y);
             framebuffer.set_current_color(color);
             framebuffer.point(i, j);
         }
 
         // Render textured floor
         for j in (framebuffer.height / 2)..framebuffer.height {
             let straight_distance = (player.pos.y * block_size as f32) / (j as f32 - hh);
             let ray_angle = player.a - player.fov / 2.0 + player.fov * i as f32 / framebuffer.width as f32;
             
             let floor_x = player.pos.x + straight_distance * ray_angle.cos();
             let floor_y = player.pos.y + straight_distance * ray_angle.sin();
 
             let texture_x = (floor_x * FLOOR.width as f32 / (maze.len() * block_size) as f32) as u32 % FLOOR.width;
             let texture_y = (floor_y * FLOOR.height as f32 / (maze[0].len() * block_size) as f32) as u32 % FLOOR.height;
 
             let color = FLOOR.get_pixel_color(texture_x, texture_y);
             framebuffer.set_current_color(color);
             framebuffer.point(i, j);
         }
     }

    for i in 0..num_rays {
        let current_ray = i as f32/ num_rays as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);


        let distance = intersect.distance * (a - player.a).cos();

        let stake_height = (framebuffer.height as f32 / distance) * 50.0; 

        let stake_top = (hh - (stake_height / 2.0 )) as usize; 
        let stake_bottom = (hh + (stake_height / 2.0 )) as usize;

        z_buffer[i] = distance;

        for y in stake_top..stake_bottom{
            let ty = (y as f32- stake_top as f32) / (stake_bottom  as f32 - stake_top as f32) * 128.0;
            let tx = intersect.tx; 

            let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }

}

fn render_monsters(framebuffer: &mut Framebuffer, player: &Player, monsters: &[Vec2], z_buffer: &mut [f32]) {
    for monster in monsters {
        render_monster(framebuffer, player, monster, z_buffer);
    }
}

fn render_monster(framebuffer: &mut Framebuffer, player: &Player, pos: &Vec2, z_buffer: &mut [f32]) {
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);
    let relative_angle = sprite_a - player.a;

    // Ajuste del ángulo relativo para mantenerlo dentro de -PI a PI
    let relative_angle = if relative_angle > PI {
        relative_angle - 2.0 * PI
    } else if relative_angle < -PI {
        relative_angle + 2.0 * PI
    } else {
        relative_angle
    };

    // Verificar si el sprite está dentro del campo de visión
    if relative_angle.abs() > player.fov / 2.0 {
        return;
    }

    let sprite_d = distance(&player.pos, pos);

    if sprite_d < 10.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 70.0;
    let start_x = (screen_width / 2.0) + (relative_angle * (screen_width / player.fov)) - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width) - 1;
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height) - 1;
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
        for x in start_x..end_x {
            for y in start_y..end_y {
                let tx = ((x - start_x) * 128 / sprite_size as usize) as u32;
                let ty = ((y - start_y) * 128 / sprite_size as usize) as u32;
                
                let color = MONSTER.get_pixel_color(tx, ty);
                if color != 0x443935 { // You might need to adjust this transparent color for the monster texture
                    framebuffer.set_current_color(color);
                    framebuffer.point(x, y);
                }
                z_buffer[x] = sprite_d;
            }
        }
    }
}

fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, minimap_x: usize, minimap_y: usize, minimap_scale: f32, monsters: &[Vec2], finish_line: &Vec2) {
    let block_size = (100.0 * minimap_scale) as usize; 

    // Add a semi-transparent background
    for x in minimap_x..minimap_x + (maze[0].len() * block_size) {
        for y in minimap_y..minimap_y + (maze.len() * block_size) {
            framebuffer.set_current_color(0x80000000);  // Semi-transparent black
            framebuffer.point(x, y);
        }
    }

    // Render maze
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            let xo = minimap_x + i * block_size;
            let yo = minimap_y + j * block_size;
            draw_cell(framebuffer, xo, yo, block_size, cell);
        }
    }

    // Render player
    let player_x = minimap_x + (player.pos.x * minimap_scale) as usize;
    let player_y = minimap_y + (player.pos.y * minimap_scale) as usize;
    framebuffer.set_current_color(0x00A6FF);  // Blue color for player
    framebuffer.point(player_x, player_y);

    // Render monsters
    for monster in monsters {
        let monster_x = minimap_x + (monster.x * minimap_scale) as usize;
        let monster_y = minimap_y + (monster.y * minimap_scale) as usize;
        framebuffer.set_current_color(0xFF0000);  // Red color for monsters
        framebuffer.point(monster_x, monster_y);
    }

    // Render finish line
    let finish_x = minimap_x + (finish_line.x * minimap_scale) as usize;
    let finish_y = minimap_y + (finish_line.y * minimap_scale) as usize;
    framebuffer.set_current_color(0x00FF00);  // Green color for finish line
    framebuffer.point(finish_x, finish_y);

    // Render player's view direction
    let view_x = player_x + (player.a.cos() * 10.0) as usize;
    let view_y = player_y + (player.a.sin() * 10.0) as usize;
    framebuffer.set_current_color(0xFFFF00);  // Yellow color for view direction
    draw_line(framebuffer, player_x, player_y, view_x, view_y);
}

// Helper function to draw a line (you may need to implement this)
fn draw_line(framebuffer: &mut Framebuffer, x1: usize, y1: usize, x2: usize, y2: usize) {
    // Implement line drawing algorithm here (e.g., Bresenham's line algorithm)
    // This is a simplified version, you might want to implement a proper line drawing function
    let dx = (x2 as i32 - x1 as i32).abs();
    let dy = (y2 as i32 - y1 as i32).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1 as i32;
    let mut y = y1 as i32;

    loop {
        framebuffer.point(x as usize, y as usize);
        if x == x2 as i32 && y == y2 as i32 { break; }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

fn main() {
    let mut show_minimap = false;
    let font_data = std::fs::read("src\\assets\\Montserrat-Medium.ttf").expect("failed to read font file");
    let text_renderer = TextRenderer::new(&font_data, 24.0);
    let (maze, finish_line) = load_maze("./maze.txt");
    let mut gilrs = Gilrs::new().unwrap();

    let window_width = 1300;
    let window_height = 900;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    framebuffer.set_background_color(0x333355);

    audio::play_background_music(); 

    let mut window = Window::new(
        "Rust Graphics - Maze Escape",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();
    
    window.set_position(100, 100);
    window.set_cursor_visibility(false);

// Show welcome screen
let mut welcome_screen = true;
while welcome_screen && window.is_open() {
    framebuffer.clear();
    framebuffer.set_background_color(0x000000); // Black background

    text_renderer.render_text(&mut framebuffer, "Welcome to Maze Escape!", 100.0, 100.0, 0xFFFFFF);
    text_renderer.render_text(&mut framebuffer, "Instructions:", 100.0, 150.0, 0xFFFFFF);
    text_renderer.render_text(&mut framebuffer, "- Use W, A, S, D keys or gamepad to move", 120.0, 200.0, 0xFFFFFF);
    text_renderer.render_text(&mut framebuffer, "- Use mouse or right analog stick to look around", 120.0, 250.0, 0xFFFFFF);
    text_renderer.render_text(&mut framebuffer, "- Press 'M' to toggle minimap", 120.0, 300.0, 0xFFFFFF);
    text_renderer.render_text(&mut framebuffer, "- Reach the green finish line to win", 120.0, 350.0, 0xFFFFFF);
    text_renderer.render_text(&mut framebuffer, "Press SPACE to start the game", 100.0, 450.0, 0xFFFFFF);

    window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();

    if window.is_key_down(Key::Space) {
        welcome_screen = false;
    }

    if window.is_key_down(Key::Escape) {
        return;
    }
}

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI/1.3,
        fov: PI/4.0,
    };    
    
    let monsters = vec![
        Vec2::new(260.0, 260.0), Vec2::new(178.0, 717.0),
        Vec2::new(1008.0, 155.0), Vec2::new(480.0, 329.0),
        Vec2::new(1096.0, 558.0),
    ];

    let finish_line = Vec2::new(500.0, 220.0);
    let mut game_state = "PLAY".to_string();

    let minimap_scale = 0.25;  // Increased scale for better visibility
    let minimap_x = 20;  // 20 pixels from the left edge
    let minimap_y = 20;  // 20 pixels from the top edge
    let mut last_frame_time = Instant::now();

    while window.is_open() {
        if window.is_key_down(Key::Escape) { break; }
        if window.is_key_pressed(Key::M, KeyRepeat::No) { show_minimap = !show_minimap; }

        let now = Instant::now();
        let delta_time = now.duration_since(last_frame_time);
        last_frame_time = now;
        let fps = 1.0 / delta_time.as_secs_f32();
        
        player.process_events(&window, &mut gilrs, &maze);
        framebuffer.clear();
        let mut z_buffer = vec![f32::INFINITY; window_width];
        match game_state.as_str() {
            "PLAY" => {
                render3d(&mut framebuffer, &player, &maze, &mut z_buffer); 
                render_monsters(&mut framebuffer, &player, &monsters, &mut z_buffer);
                
                if show_minimap {
                    render_minimap(&mut framebuffer, &player, &maze, minimap_x, minimap_y, minimap_scale, &monsters, &finish_line);
                }
                
                // Check if player has reached the finish line
                let distance_to_finish = nalgebra_glm::distance(&player.pos, &finish_line);
                if distance_to_finish < 50.0 {
                    game_state = "SUCCESS".to_string();
                    println!("Player reached finish line! Distance: {}", distance_to_finish);
                }
    
                // Debug information
                text_renderer.render_text(&mut framebuffer, &format!("FPS: {:.2}", fps), window_width as f32 - 150.0, 20.0, 0xFFFFFF);
                text_renderer.render_text(&mut framebuffer, &format!("Player Pos: ({:.2}, {:.2})", player.pos.x, player.pos.y), 20.0, 20.0, 0xFFFFFF);
                text_renderer.render_text(&mut framebuffer, &format!("Finish Line: ({:.2}, {:.2})", finish_line.x, finish_line.y), 20.0, 50.0, 0xFFFFFF);
                text_renderer.render_text(&mut framebuffer, &format!("Distance to Finish: {:.2}", distance_to_finish), 20.0, 80.0, 0xFFFFFF);
                text_renderer.render_text(&mut framebuffer, "Press 'M' to toggle minimap", 20.0, window_height as f32 - 30.0, 0xFFFFFF);
            },
            "SUCCESS" => {
                framebuffer.clear();
                framebuffer.set_background_color(0x000000); 
                text_renderer.render_text(&mut framebuffer, "¡Felicidades! Has llegado a la meta.", 100.0, 100.0, 0xFFFFFF);
                text_renderer.render_text(&mut framebuffer, "Presiona \"E\" para salir del juego.", 100.0, 150.0, 0xFFFFFF);
                if window.is_key_down(Key::E) { break; }
            },
            _ => {}
        }
        
        window.update_with_buffer(&framebuffer.buffer, window_width, window_height).unwrap();
        std::thread::sleep(frame_delay);
    }
}
