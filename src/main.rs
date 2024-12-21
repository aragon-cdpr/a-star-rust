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

enum ClickType {
    Start,
    Target,
}

#[macroquad::main("A* pathfinding")]
async fn main() {
    let mut player = Player {
        position: vec2(4.0, 0.0),
    };
    let mut click_type = ClickType::Start;
    set_window_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    let (mut start, mut target) = (vec2(-1., -1.), vec2(-1., -1.));
    loop {
        if is_mouse_button_pressed(MouseButton::Right) {
            match click_type {
                ClickType::Start => {
                    let p = get_mouse_position_grid_point(&mouse_position());
                    if p.0 > -1. && p.1 > -1. {
                        start = p.into();
                        click_type = ClickType::Target;
                    }
                }
                ClickType::Target => {
                    let p = get_mouse_position_grid_point(&mouse_position());
                    if p.0 > -1. && p.1 > -1. {
                        target = p.into();
                        click_type = ClickType::Start;
                    }
                }
            }
        }
        clear_background(DARKGRAY);
        update_player(&mut player);
        draw_grid(&player, start, target);
        next_frame().await
    }
}

fn draw_grid(player: &Player, start: Vec2, target: Vec2) {
    let (ww, wh) = screen_size();
    let (px, py) = player.position.into();
    let (sx, sy) = start.into();
    let (tx, ty) = target.into();
    for i in 0..(PLANE_HEIGHT / TILE_SIZE) as isize {
        for j in 0..(PLANE_WIDTH / TILE_SIZE) as isize {
            let mut color: Color = Color::from_hex(0x3b3b3b);
            if (i + j) % 2 == 0 {
                color = Color::from_hex(0xb3b3b3);
            }
            if sx == j as f32 && sy == i as f32 {
                color = Color::from_hex(0x00ff00);
            }
            if tx == j as f32 && ty == i as f32 {
                color = Color::from_hex(0x0000ff);
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

fn get_mouse_position_grid_point(mouse_position: &(f32, f32)) -> (f32, f32) {
    let (mouse_x, mouse_y) = mouse_position;
    let (ww, wh) = screen_size();
    let cells_horizontal = PLANE_WIDTH / TILE_SIZE;
    let cells_vertical = PLANE_HEIGHT / TILE_SIZE;
    let horizontal_padding = (ww - PLANE_WIDTH) / 2.;
    let vertical_padding = (wh - PLANE_HEIGHT) / 2.;
    let (mut px, mut py): (f32, f32) = (-1., -1.);

    if *mouse_x >= horizontal_padding
        && *mouse_y >= vertical_padding
        && *mouse_x <= ww - horizontal_padding
        && *mouse_y <= wh - vertical_padding
    {
        px = (*mouse_x - horizontal_padding) / TILE_SIZE;
        py = (*mouse_y - vertical_padding) / TILE_SIZE;
    }

    return (px.floor(), py.floor());
}
