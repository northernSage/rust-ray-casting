/*
    Simple Ray Casting Rendering

    based on: https://en.wikipedia.org/wiki/Wolfenstein_3D

    ############
    ##\    #####
    ## \   /   #
    #   \ /    #
    #    P     #
    #         ##
    ############

    P = player location
    # = any 3D object
    \/ = player field of view

    We take each column of the console screen buffer and map that
    to a ray, cast within player's field of view. The most important
    aspect is figuring out how far the ray travels before hitting a
    surface, so we can use that to project the ilusion of size and 
    distance.

       screen width
    |----------------|

       player angle     
    \       |       /              
     \      |      /         
      \     |     /          
       \    |    /                   
        \   |   /            
         \  |  /             
          \ | /              
-FOV/2      P      +FOV/2    

    starting angle for FOV = player angle - (FOV / 2)

    FOV angle incremental step = (x <counter from 0 to screen width> / screen width) * FOV
                                  -----------------\ /-------------------------------
                                                    V
                                                   % increment of FOV (1/FOV, 2/FOV, 3/FOV, etc...)

    ray angle = starting angle for FOV + FOV angle incremental step

    Notice here that the smaller the FOV is the larger the interation step will be in relation to
    the total FOV size, this may need to be adjusted later on with a better formula for increment size.

           #
           | -------------
    (check hit wall)     |
           | ------------- 
    (check hit wall)     |-----> distance to wall increments
           | -------------
    (check hit wall)     |
           | -------------
           P 

    Once starting angle calculated, we test each incremental step "forward" in order to check if 
    it hits a "wall" or not. This is how we can determine the approximate distance to the wall.
    To actually check if a wall has been hit or not, we need to calculate the direction that the
    player is looking towards, we can take a unit vector:

      (eye direction x, eye direction y)
    P ---------------------------------> (unit vector expressing player facing direction)

    eye direction x = sin(ray angle)
    eye direction y = cos(ray angle)
    unit vector = (eye direction x, eye direction y)

    and now we can get the testing point by steping forward following the unit vector direction

    test_point_x = player x + eye direction x * distance to wall
    test_point_y = player y + eye direction y * distance to wall

      unit vector   (test point x, test point y)
    P -----------> + --------------------------> (test point)

    Next, we can create the ilusion of distance by making the ceiling and floor larger
    the further away into the horizon the point is. For that we need to now calculate
    the amount of space that ceiling and floor takes up in our console display

    ceiling size = (screen height / 2.0) - (screen high / distance to wall)
                    --------\ /---------    -------------\ /---------------
                             V                            V
              start with upper scren half     the further away we are the more ceiling we see

    floor size = screen heightt = ceiling size
 */                          

mod keyboard;

pub use crate::keyboard::KeyboardState;
use pixel_canvas::{Canvas, Color, input::glutin::event::VirtualKeyCode};

fn main() {
    let canvas = Canvas::new(512, 512)
        .title("Ray Casting Simulation")
        .state(KeyboardState::new())
        .input(KeyboardState::handle_input);

    // player
    let mut player_x: f64 = 8.0;
    let mut player_y: f64 = 8.0;
    let mut player_vision_angle: f64 = 0.0;

    // map
    let map_height: u16 = 16;
    let map_width: u16 = 16;
    let map: Vec<char> =
        "################\
        #..............#\
        #..............#\
        #......####....#\
        #..............#\
        #......#########\
        #..............#\
        #............###\
        #..............#\
        #..............#\
        #..............#\
        #.##...........#\
        #......#.......#\
        #......#.......#\
        ################".chars().collect();
        
        // FOV
        let field_of_view_angle = 3.14159 / 4.0; // pi * 1/4 (very narrow field of view)

        // distance to wall
        let max_wall_check_depth: f64 = 16.0; // follows map size 

    canvas.render(move |keyboard, image| {
        let width = image.width();

        let mut ceiling_lower_boundary: f64;
        let mut floor_upper_boundary: f64;

        // FOV
        let mut start_of_fov_angle: f64;

        // ray casting
        let mut ray_angle: f64;
        let mut unit_ray_x: f64;
        let mut unit_ray_y: f64;

        // distance to wall
        let mut distance_to_wall: f64;
        let mut hit_wall: bool;
        let mut test_x: u16;
        let mut test_y: u16;

        // paint canvas
        let mut pixel_color;

        let mut wall_color_shade: u8;

        let mut shade_multiplier: f64;

        match keyboard.key_pressed() {
            Some(VirtualKeyCode::A) => player_vision_angle -= 0.1,
            Some(VirtualKeyCode::D) => player_vision_angle += 0.1,
            Some(VirtualKeyCode::W) => {
                player_x += player_vision_angle.sin() * 0.2;
                player_y += player_vision_angle.cos() * 0.2;

                if map[(player_y as u16 * map_width + player_x as u16) as usize] == '#' {
                    player_x -= player_vision_angle.sin() * 0.2;
                    player_y -= player_vision_angle.cos() * 0.2; 
                }
            },
            Some(VirtualKeyCode::S) => {
                player_x -= player_vision_angle.sin() * 0.2;
                player_y -= player_vision_angle.cos() * 0.2;

                if map[(player_y as u16 * map_width + player_x as u16) as usize] == '#' {
                    player_x += player_vision_angle.sin() * 0.2;
                    player_y += player_vision_angle.cos() * 0.2; 
                }
            },
            _ => (),
        }

        for (y, row) in image.chunks_mut(width).enumerate() {

            for (x, pixel) in row.iter_mut().enumerate() {

                // starting ray angle for FOV swip
                start_of_fov_angle = player_vision_angle - (field_of_view_angle / 2.0);
                ray_angle = start_of_fov_angle + (x as f64 / width as f64) * field_of_view_angle;

                // distance to wall logic
                hit_wall = false;
                distance_to_wall = 0.0;

                // ray unit vector (direction of ray vector)
                unit_ray_x = ray_angle.sin();
                unit_ray_y = ray_angle.cos();

                // scalar horizon stepping 
                while !hit_wall && distance_to_wall < max_wall_check_depth {
                    distance_to_wall += 0.1;

                    // test point, all walls are in integer boundaries so we don't care for non-int values
                    test_x = (player_x + unit_ray_x * distance_to_wall) as u16;
                    test_y = (player_y + unit_ray_y * distance_to_wall) as u16;

                    // test if point is out of bounds
                    if test_x >= map_width || test_y >= map_height {
                        hit_wall = true;
                        distance_to_wall = max_wall_check_depth;
                    } else {
                        // we are in bounds, check if test point is hitting a wall
                        if map[(test_y * map_width + test_x) as usize] == '#' {
                            hit_wall = true;
                        }
                    }
                }

                floor_upper_boundary = (width as f64 / 2.0) - (width as f64 / distance_to_wall);
                ceiling_lower_boundary = width as f64 - floor_upper_boundary;

                // floor
                if y < floor_upper_boundary as usize {
                    shade_multiplier =  y as f64 * (- 0.95 / floor_upper_boundary) + 1.0;
                    pixel_color = Color { 
                        r: (140.0 * shade_multiplier) as u8, 
                        g: (40.0 * shade_multiplier) as u8, 
                        b: (5.0 * shade_multiplier) as u8
                    };
                    // wall
                } else if y > floor_upper_boundary as usize && y <= ceiling_lower_boundary as usize {
                    wall_color_shade = (-13.4375 * distance_to_wall + 235.0) as u8;
                    pixel_color = Color { r: wall_color_shade, g: wall_color_shade, b: wall_color_shade };
                    // ceiling
                } else {
                    // shade_multiplier = (0.9 / ((width-1) as f64 - ceiling_lower_boundary)) * (y as f64 - (width-1) as f64) + 1.0;
                    // pixel_color = Color { 
                    //     r: (140.0 * shade_multiplier) as u8, 
                    //     g: (40.0 * shade_multiplier) as u8, 
                    //     b: (5.0 * shade_multiplier) as u8
                    // };
                    pixel_color = Color { r: 0, g: 0, b: 0 }
                }

                *pixel = pixel_color;

            }
        }
    });
}