use macroquad::{prelude::{*, camera::mouse}, miniquad::start};

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

#[derive(PartialEq)]
struct Tile{
    x: f32,
    y: f32,
    color: Color,
}

fn draw_tiles(tile_vec: &Vec<Tile>){
    for tile in tile_vec.iter(){
        draw_rectangle(tile.x, tile.y, 10.0, 10.0, tile.color);
    }
}


#[macroquad::main(window_conf)]
async fn main() {

    let mut state_integer = 0;
    
    let screen_width = screen_width();
    let grid_size = ((screen_width / 10.0) + 1.0) as i32;
    
    let mut tile_vec: Vec<Tile> = Vec::new();


    loop{

        if state_integer == 0{
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            println!("{:?}", mouse_position());
            if is_mouse_button_pressed(MouseButton::Left){
                let start_pos = Tile{
                    x: (mouse_position().0 / 10.0).floor() * 10.0,
                    y: (mouse_position().1 / 10.0).floor() * 10.0,
                    color: BLUE,
                };
                tile_vec.push(start_pos);
            }
            if is_mouse_button_released(MouseButton::Left){
                state_integer = 1;
            }
        }

        else if state_integer == 1{
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            println!("{:?}", mouse_position());
            draw_tiles(&tile_vec);
            if is_mouse_button_pressed(MouseButton::Left){
                let end_pos = Tile{
                    x: (mouse_position().0 / 10.0).floor() * 10.0,
                    y: (mouse_position().1 / 10.0).floor() * 10.0,
                    color: RED,
                };
                tile_vec.push(end_pos);
            }
            if is_mouse_button_released(MouseButton::Left){
                state_integer = 2;
            }
        }

        else if state_integer == 2{
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            draw_tiles(&tile_vec);
            if is_mouse_button_down(MouseButton::Left){
                let wall_pos = Tile{
                    x: (mouse_position().0 / 10.0).floor() * 10.0,
                    y: (mouse_position().1 / 10.0).floor() * 10.0,
                    color: BLACK,
                };
                if tile_vec.contains(&wall_pos){}
                else{
                    tile_vec.push(wall_pos);
                }
                
            }
            else if is_mouse_button_pressed(MouseButton::Middle){
                state_integer = 3;
            };
        }

        else if state_integer == 3{
            clear_background(WHITE);
            draw_grid(screen_width, grid_size);
            draw_tiles(&tile_vec);
            let mp = mouse_position();
            println!("{:?}", mp);
        }
        

        


        next_frame().await
    }
    
}

