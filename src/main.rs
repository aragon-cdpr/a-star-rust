use macroquad::prelude::*;
use miniquad::window::{screen_size, set_window_size};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const WINDOW_WIDTH: f32 = 1320.0;
const WINDOW_HEIGHT: f32 = 960.0;
const PLANE_WIDTH: f32 = 1200.0;
const PLANE_HEIGHT: f32 = 700.0;
const TILE_SIZE: f32 = 50.0;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: IVec2,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Player {
    position: Vec2,
}

enum ClickType {
    Start,
    Target,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum TileType {
    Player,
    Start,
    Target,
    None,
}

type Plane = Vec<Vec<TileType>>;

#[macroquad::main("A* pathfinding")]
async fn main() {
    let mut plane: Plane = vec![
        vec![TileType::None; (PLANE_WIDTH / TILE_SIZE) as usize];
        (PLANE_HEIGHT / TILE_SIZE) as usize
    ];
    // let mut player = Player {
    //     position: vec2(4.0, 0.0),
    // };
    // plane[player.position.y as usize][player.position.x as usize] = TileType::Player;
    let mut click_type = ClickType::Start;
    set_window_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
    let (mut start, mut target) = (Vec2::new(-1., -1.), Vec2::new(-1., -1.));
    loop {
        if is_mouse_button_pressed(MouseButton::Right) {
            match click_type {
                ClickType::Start => {
                    handle_click(&mut plane, click_type, &mut start);
                    click_type = ClickType::Target;
                }
                ClickType::Target => {
                    handle_click(&mut plane, click_type, &mut target);
                    click_type = ClickType::Start;
                }
            }
        }
        clear_background(DARKGRAY);
        update_path(
            &mut plane,
            IVec2::new(start.x as i32, start.y as i32),
            IVec2::new(target.x as i32, target.y as i32),
        );
        // update_player(&mut player, &mut plane);
        draw_grid(&plane);
        next_frame().await
    }
}

fn draw_grid(plane: &Plane) {
    let (ww, wh) = screen_size();

    for i in 0..(PLANE_HEIGHT / TILE_SIZE) as usize {
        for j in 0..(PLANE_WIDTH / TILE_SIZE) as usize {
            let mut color: Color = Color::from_hex(0x3b3b3b);
            match plane[i][j] {
                TileType::Player => {
                    color = Color::from_hex(0xff0000);
                }
                TileType::Start => {
                    color = Color::from_hex(0x00ff00);
                }
                TileType::Target => {
                    color = Color::from_hex(0x0000ff);
                }
                TileType::None => {
                    if (i + j) % 2 == 0 {
                        color = Color::from_hex(0xb3b3b3);
                    }
                }
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

fn update_player(player: &mut Player, plane: &mut Plane) {
    plane[player.position.y as usize][player.position.x as usize] = TileType::None;
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
    plane[player.position.y as usize][player.position.x as usize] = TileType::Player;
}

fn get_mouse_position_grid_point(mouse_position: &(f32, f32)) -> (f32, f32) {
    let (mouse_x, mouse_y) = mouse_position;
    let (ww, wh) = screen_size();
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

fn handle_click(plane: &mut Plane, click_type: ClickType, point: &mut Vec2) {
    let (px, py) = get_mouse_position_grid_point(&mouse_position());
    if px > -1. && py > -1. {
        plane[point.y as usize][point.x as usize] = TileType::None;
        *point = vec2(px, py);
        match click_type {
            ClickType::Start => {
                plane[py as usize][px as usize] = TileType::Start;
            }
            ClickType::Target => {
                plane[py as usize][px as usize] = TileType::Target;
            }
        }
    }
}

fn update_path(plane: &mut Plane, start: IVec2, target: IVec2) {
    if (start.x < 0 && start.y < 0) || (target.x < 0 && target.y < 0) {
        return;
    }
    // Clear previous path from the grid.
    for row in plane.iter_mut() {
        for tile in row.iter_mut() {
            if *tile == TileType::Player {
                *tile = TileType::None;
            }
        }
    }

    // Perform A* algorithm to find the path.
    let mut open_set = BinaryHeap::new();
    open_set.push(Node {
        position: start,
        cost: 0,
    });
    plane[start.y as usize][start.x as usize] = TileType::Start;

    let mut came_from: Vec<Vec<Option<Vec2>>> =
        vec![vec![None; (PLANE_WIDTH / TILE_SIZE) as usize]; (PLANE_HEIGHT / TILE_SIZE) as usize];
    let mut cost_so_far: Vec<Vec<f32>> =
        vec![
            vec![f32::INFINITY; (PLANE_WIDTH / TILE_SIZE) as usize];
            (PLANE_HEIGHT / TILE_SIZE) as usize
        ];
    cost_so_far[start.y as usize][start.x as usize] = 0.0;

    while let Some(current) = open_set.pop() {
        if current.position == target {
            break;
        }

        for neighbor in get_neighbors(current.position) {
            let new_cost =
                cost_so_far[current.position.y as usize][current.position.x as usize] + 1.0; // Assuming uniform cost

            if new_cost < cost_so_far[neighbor.y as usize][neighbor.x as usize] {
                cost_so_far[neighbor.y as usize][neighbor.x as usize] = new_cost;
                let priority = new_cost + heuristic(neighbor, target); // f(x) = g(x) + h(x)
                open_set.push(Node {
                    position: IVec2::new(neighbor.x as i32, neighbor.y as i32),
                    cost: priority as u32,
                });
                came_from[neighbor.y as usize][neighbor.x as usize] = Some(Vec2::new(
                    current.position.x as f32,
                    current.position.y as f32,
                ));
            }
        }
    }

    // Reconstruct path and update the grid.
    let mut path = Vec::new();
    let mut current = target;
    while current != start {
        path.push(current);
        if let Some(prev) = came_from[current.y as usize][current.x as usize] {
            current = IVec2::new(prev.x as i32, prev.y as i32);
        } else {
            break; // No path found
        }
    }

    for pos in path.iter() {
        plane[pos.y as usize][pos.x as usize] = TileType::Player;
        if target == *pos {
            plane[pos.y as usize][pos.x as usize] = TileType::Target;
        }
    }
}

fn heuristic(a: IVec2, b: IVec2) -> f32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as f32 // Manhattan distance
}

fn get_neighbors(position: IVec2) -> Vec<IVec2> {
    let mut neighbors = Vec::new();
    let (x, y) = (position.x as isize, position.y as isize);

    let directions = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    for (dx, dy) in &directions {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0
            && ny >= 0
            && nx < (PLANE_WIDTH / TILE_SIZE) as isize
            && ny < (PLANE_HEIGHT / TILE_SIZE) as isize
        {
            neighbors.push(IVec2::new(nx as i32, ny as i32));
        }
    }

    neighbors
}
