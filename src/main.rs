use macroquad::prelude::*;
use std::{thread, time};

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

fn draw_grid(screen_width: f32, grid_size: i32) {
    for i in 0..grid_size {
        draw_line(
            0.0,
            (i * 10) as f32,
            screen_width,
            (i * 10) as f32,
            1.0,
            BLACK,
        );
        draw_line(
            (i * 10) as f32,
            0.0,
            (i * 10) as f32,
            screen_width,
            1.0,
            BLACK,
        );
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Tile {
    x: f32,
    y: f32,
    color: Color,
    link_tile: [f32; 2],
}

fn draw_tiles(tile_vec: &Vec<Tile>) {
    for tile in tile_vec.iter() {
        draw_rectangle(tile.x * 10.0, tile.y * 10.0, 10.0, 10.0, tile.color);
    }
}

fn on_top_of_existing_tiles(tile_vec: &Vec<Tile>, input_tile: &Tile) -> bool {
    for tile in tile_vec.iter() {
        if input_tile.x == tile.x && input_tile.y == tile.y {
            return true;
        }
    }
    return false;
}

fn target_tile_found(tile_vec: &Vec<Tile>, input_tile: &Tile) -> bool {
    for tile in tile_vec.iter() {
        if input_tile.x == tile.x && input_tile.y == tile.y && tile.color == RED {
            return true;
        }
    }
    return false;
}

fn out_of_bounds(input_tile: &Tile, screen_width: f32, screen_height: f32) -> bool {
    if input_tile.x < 0.0 {
        return true;
    } else if input_tile.y < 0.0 {
        return true;
    } else if input_tile.x > ((screen_width / 10.0) - 1.0).floor() {
        return true;
    } else if input_tile.y > ((screen_height / 10.0) - 1.0).floor() {
        return true;
    }
    return false;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state_integer = 0;

    let screen_width = screen_width();
    let screen_height = screen_height();
    let grid_size = ((screen_width / 10.0) + 1.0) as i32;
    let mut link_tile_params = [0.1, 0.1];

    let mut tile_vec: Vec<Tile> = Vec::new();
    let mut previous_cycle_tile_vec = Vec::new();

    loop {
        //set start tile
        if state_integer == 0 {
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            if is_mouse_button_pressed(MouseButton::Left) {
                let start_tile = Tile {
                    x: (mouse_position().0 / 10.0).floor(),
                    y: (mouse_position().1 / 10.0).floor(),
                    color: BLUE,
                    link_tile: [0.0, 0.0],
                };
                tile_vec.push(start_tile);
            }
            if is_mouse_button_released(MouseButton::Left) {
                state_integer = 1;
            }
        }
        //set end tile
        else if state_integer == 1 {
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            draw_tiles(&tile_vec);
            if is_mouse_button_pressed(MouseButton::Left) {
                let end_tile = Tile {
                    x: (mouse_position().0 / 10.0).floor(),
                    y: (mouse_position().1 / 10.0).floor(),
                    color: RED,
                    link_tile: [0.0, 0.0],
                };
                if on_top_of_existing_tiles(&tile_vec, &end_tile) {
                } else {
                    tile_vec.push(end_tile);
                }
            }
            if tile_vec.len() == 2 && is_mouse_button_released(MouseButton::Left) {
                state_integer = 2;
            }
        }
        //set the wall tiles that cannot be passed
        else if state_integer == 2 {
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            draw_tiles(&tile_vec);
            if is_mouse_button_down(MouseButton::Left) {
                let wall_pos = Tile {
                    x: (mouse_position().0 / 10.0).floor(),
                    y: (mouse_position().1 / 10.0).floor(),
                    color: BLACK,
                    link_tile: [0.0, 0.0],
                };
                if tile_vec.contains(&wall_pos) {
                } else {
                    if on_top_of_existing_tiles(&tile_vec, &wall_pos) {
                    } else {
                        tile_vec.push(wall_pos);
                    }
                }
            } else if is_mouse_button_pressed(MouseButton::Middle) {
                state_integer = 3;
            };
        } else if state_integer == 3 {
            for tile in tile_vec.iter() {
                if tile.color == BLUE {
                    previous_cycle_tile_vec.push(*tile);
                    state_integer = 4;
                    break;
                }
            }
        }
        //pathfinding
        else if state_integer == 4 {
            clear_background(WHITE);
            draw_tiles(&tile_vec);
            draw_grid(screen_width, grid_size);

            let mut temp_tile_vec = Vec::new();

            for tile in previous_cycle_tile_vec.iter() {
                let new_tile_right = Tile {
                    x: tile.x + 1.0,
                    y: tile.y,
                    color: SKYBLUE,
                    link_tile: [tile.x, tile.y],
                };
                temp_tile_vec.push(new_tile_right);

                let new_tile_left = Tile {
                    x: tile.x - 1.0,
                    y: tile.y,
                    color: SKYBLUE,
                    link_tile: [tile.x, tile.y],
                };
                temp_tile_vec.push(new_tile_left);

                let new_tile_up = Tile {
                    x: tile.x,
                    y: tile.y + 1.0,
                    color: SKYBLUE,
                    link_tile: [tile.x, tile.y],
                };
                temp_tile_vec.push(new_tile_up);

                let new_tile_down = Tile {
                    x: tile.x,
                    y: tile.y - 1.0,
                    color: SKYBLUE,
                    link_tile: [tile.x, tile.y],
                };
                temp_tile_vec.push(new_tile_down);
            }
            previous_cycle_tile_vec.clear();

            for tile in temp_tile_vec.iter() {
                if out_of_bounds(tile, screen_width, screen_height) {
                } else if target_tile_found(&tile_vec, tile) {
                    for i in 0..tile_vec.len() {
                        if tile_vec[i].color == RED {
                            tile_vec[i].link_tile = tile.link_tile;
                            link_tile_params = tile.link_tile;
                        }
                    }
                    state_integer = 5;
                } else if on_top_of_existing_tiles(&tile_vec, tile) {
                } else if tile_vec.contains(tile) {
                } else {
                    tile_vec.push(*tile);
                    previous_cycle_tile_vec.push(*tile);
                }
            }

            thread::sleep(time::Duration::from_millis(100));
        } else if state_integer == 5 {
            clear_background(WHITE);
            draw_tiles(&tile_vec);
            draw_grid(screen_width, grid_size);

            for i in 0..tile_vec.len() {
                if tile_vec[i].color == BLUE {
                } else if [tile_vec[i].x, tile_vec[i].y] == link_tile_params {
                    tile_vec[i].color = ORANGE;
                    link_tile_params = tile_vec[i].link_tile;
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
        next_frame().await
    }
}
