use macroquad::prelude::*;
use miniquad::window::{screen_size, set_window_size};

const WINDOW_WIDTH: f32 = 1320.0;
const WINDOW_HEIGHT: f32 = 960.0;
const PLANE_WIDTH: f32 = 1200.0;
const PLANE_HEIGHT: f32 = 700.0;
const TILE_SIZE: f32 = 50.0;

struct Player {
    position: Vec2,
}

#[macroquad::main("A* pathfinding")]
async fn main() {
    let mut player = Player {
        position: vec2(4.0, 0.0),
    };
    set_window_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    loop {
        clear_background(DARKGRAY);
        update_player(&mut player);
        draw_grid(&player);
        next_frame().await
    }
}

fn draw_grid(player: &Player) {
    let (ww, wh) = screen_size();
    let (px, py) = player.position.into();
    for i in 0..(PLANE_HEIGHT / TILE_SIZE) as isize {
        for j in 0..(PLANE_WIDTH / TILE_SIZE) as isize {
            let mut color: Color = Color::from_hex(0x3b3b3b);
            if (i + j) % 2 == 0 {
                color = Color::from_hex(0xb3b3b3);
            }
            if px == j as f32 && py == i as f32 {
                color = Color::from_hex(0xff0000);
            }
            draw_rectangle(
                (ww - PLANE_WIDTH) / 2. + j as f32 * TILE_SIZE,
                (wh - PLANE_HEIGHT) / 2. + i as f32 * TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE,
                color,
            );
        }
    }
}

fn update_player(player: &mut Player) {
    if is_key_pressed(KeyCode::W) {
        if player.position.y > 0.0 {
            player.position.y -= 1.0;
        }
    }
    if is_key_pressed(KeyCode::A) {
        if player.position.x > 0.0 {
            player.position.x -= 1.0;
        }
    }
    if is_key_pressed(KeyCode::S) {
        if player.position.y < (PLANE_HEIGHT / TILE_SIZE) - 1. {
            player.position.y += 1.0;
        }
    }
    if is_key_pressed(KeyCode::D) {
        if player.position.x < (PLANE_WIDTH / TILE_SIZE) - 1. {
            player.position.x += 1.0;
        }
    }
}
