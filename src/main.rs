use macroquad::prelude::*;
use std::{env, thread, time};

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 800,
        fullscreen: false,
        ..Default::default()
    }
}

fn draw_grid(screen_width: f32, grid_size:i32){
    for i in 0..grid_size{
        draw_line(0.0, (i*10) as f32, screen_width, (i*10) as f32, 1.0, BLACK);
        draw_line((i*10) as f32,0.0, (i*10) as f32, screen_width, 1.0, BLACK);
    }
}

#[derive(PartialEq, Copy, Clone)]
struct Tile{
    x: f32,
    y: f32,
    color: Color,
}

fn draw_tiles(tile_vec: &Vec<Tile>){
    for tile in tile_vec.iter(){
        draw_rectangle(tile.x * 10.0, tile.y * 10.0, 10.0, 10.0, tile.color);
    }
}

fn on_top_of_existing_tiles(tile_vec: &Vec<Tile>, input_tile:&Tile) -> bool{
    for tile in tile_vec.iter(){
        if input_tile.x == tile.x && input_tile.y == tile.y{
            return true
        }
    }
    false
}

fn out_of_bounds(input_tile:&Tile, screen_width: f32, screen_height: f32) -> bool{
    if input_tile.x < 0.0{
        return true
    }
    else if input_tile.y < 0.0{
        return true
    }
    else if input_tile.x > ((screen_width / 10.0) - 1.0).floor(){
        return true
    }
    else if input_tile.y > ((screen_height / 10.0) - 1.0).floor(){
        return true
    }
    return false
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut state_integer = 0;
    
    let screen_width = screen_width();
    let screen_height = screen_height();
    let grid_size = ((screen_width / 10.0) + 1.0) as i32;
    
    let mut tile_vec: Vec<Tile> = Vec::new();
    let mut start: [f32; 2] = [0.0, 0.0];
    let mut end: [f32; 2] = [0.0, 0.0];
    let mut path_vec: Vec<Vec<[bool; 2]>> = Vec::new();
    let mut previous_cycle_tile_vec = Vec::new();
    


    loop{

        //set start tile
        if state_integer == 0{
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            if is_mouse_button_pressed(MouseButton::Left){
                let start_tile = Tile{
                    x: (mouse_position().0 / 10.0).floor(),
                    y: (mouse_position().1 / 10.0).floor(),
                    color: BLUE,
                };
                start = [start_tile.x, start_tile.y];
                tile_vec.push(start_tile);
            }
            if is_mouse_button_released(MouseButton::Left){
                state_integer = 1;
            }
        }

        //set end tile
        else if state_integer == 1{
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            draw_tiles(&tile_vec);
            if is_mouse_button_pressed(MouseButton::Left){
                let end_tile = Tile{
                    x: (mouse_position().0 / 10.0).floor(),
                    y: (mouse_position().1 / 10.0).floor(),
                    color: RED,
                };
                if on_top_of_existing_tiles(&tile_vec, &end_tile){}
                else{
                    end = [end_tile.x, end_tile.y];
                    tile_vec.push(end_tile);
                }
            }
            if tile_vec.len() == 2 && is_mouse_button_released(MouseButton::Left){
                state_integer = 2;
            }
        }

        //set the wall tiles that cannot be passed
        else if state_integer == 2{
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            draw_tiles(&tile_vec);
            if is_mouse_button_down(MouseButton::Left){
                let wall_pos = Tile{
                    x: (mouse_position().0 / 10.0).floor(),
                    y: (mouse_position().1 / 10.0).floor(),
                    color: BLACK,
                };
                if tile_vec.contains(&wall_pos){}
                else{
                    if on_top_of_existing_tiles(&tile_vec, &wall_pos){}
                    else{
                        tile_vec.push(wall_pos);
                    }
                }  
            }
            else if is_mouse_button_pressed(MouseButton::Middle){
                state_integer = 3;
            };
        }
        else if state_integer == 3{
            for tile in tile_vec.iter(){
                if tile.color == BLUE{
                    previous_cycle_tile_vec.push(*tile);
                    state_integer = 4;
                    break;
                }
            }
        }
        //pathfinding
        else if state_integer == 4{
            
            clear_background(WHITE);
            draw_tiles(&tile_vec);
            draw_grid(screen_width, grid_size);
            
            let mut temp_tile_vec = Vec::new();

            for tile in previous_cycle_tile_vec.iter(){
                    let new_tile_01 = Tile{
                        x: tile.x+1.0,
                        y: tile.y,
                        color: SKYBLUE,
                    };
                    temp_tile_vec.push(new_tile_01);
                    let new_tile_02 = Tile{
                        x: tile.x-1.0,
                        y: tile.y,
                        color: SKYBLUE,
                    };
                    temp_tile_vec.push(new_tile_02);
                    let new_tile_03 = Tile{
                        x: tile.x,
                        y: tile.y+1.0,
                        color: SKYBLUE,
                    };
                    temp_tile_vec.push(new_tile_03);
                    let new_tile_04 = Tile{
                        x: tile.x,
                        y: tile.y-1.0,
                        color: SKYBLUE,
                    };
                    temp_tile_vec.push(new_tile_04);
            }
            previous_cycle_tile_vec.clear();
            for tile in temp_tile_vec.iter(){
                if out_of_bounds(tile, screen_width, screen_height){}
                else if on_top_of_existing_tiles(&tile_vec, tile){}
                else if tile_vec.contains(tile){}
                else{
                    tile_vec.push(*tile);
                    previous_cycle_tile_vec.push(*tile);
                }
            }
            thread::sleep(time::Duration::from_millis(100));
        }
        next_frame().await
    }
    
}

