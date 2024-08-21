use nalgebra_glm::Vec2;
use minifb::{Key, Window};

pub struct Player {
    pub pos: Vec2,
    pub a: f32,  // Ángulo de visión
    pub fov: f32, // Campo de visión
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32) -> Self {
        Player { pos, a, fov }
    }
}

static mut LAST_MOUSE_X: f32 = 0.0;

pub fn process_events(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>) {
    const MOVE_SPEED: f32 = 0.5; // Velocidad de movimiento lenta
    const ROTATION_SPEED: f32 = 0.002; // Velocidad de rotación lenta
    const STRAFE_SPEED: f32 = 0.5; // Velocidad de desplazamiento lateral lenta

    let (mouse_x, _) = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap_or((0.0, 0.0));

    unsafe {
        let mouse_dx = mouse_x - LAST_MOUSE_X;
        LAST_MOUSE_X = mouse_x;

        player.a += mouse_dx * ROTATION_SPEED;
    }

    // Calcular el movimiento hacia adelante y hacia atrás
    let forward_x = player.pos.x + MOVE_SPEED * player.a.cos();
    let forward_y = player.pos.y + MOVE_SPEED * player.a.sin();
    let backward_x = player.pos.x - MOVE_SPEED * player.a.cos();
    let backward_y = player.pos.y - MOVE_SPEED * player.a.sin();

    // Calcular el movimiento de strafe (desplazamiento lateral)
    let right_vector = nalgebra_glm::rotate_vec2(&Vec2::new(player.a.cos(), player.a.sin()), -std::f32::consts::PI / 2.0);
    let strafe_right_x = player.pos.x + STRAFE_SPEED * right_vector.x;
    let strafe_right_y = player.pos.y + STRAFE_SPEED * right_vector.y;
    let strafe_left_x = player.pos.x - STRAFE_SPEED * right_vector.x;
    let strafe_left_y = player.pos.y - STRAFE_SPEED * right_vector.y;

    // Comprobación de las teclas presionadas y actualización de la posición del jugador
    if window.is_key_down(Key::S) {
        player.pos.x = forward_x;
        player.pos.y = forward_y;
    }
    if window.is_key_down(Key::W) {
        player.pos.x = backward_x;
        player.pos.y = backward_y;
    }
    if window.is_key_down(Key::D) {
        player.pos.x = strafe_right_x;
        player.pos.y = strafe_right_y;
    }
    if window.is_key_down(Key::A) {
        player.pos.x = strafe_left_x;
        player.pos.y = strafe_left_y;
    }
}
