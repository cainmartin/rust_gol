mod world;
mod defines;

use crate::world::*;
use crate::defines::*;
use minifb::{ Key, Window, WindowOptions };

fn main() {

    println!("Initializing the game of life");

    let mut world = World::new();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut scaled_buffer: Vec<u32> = vec![0; SCALED_WIDTH * SCALED_HEIGHT];

    let mut window = Window::new(
        "Game Of Life - Rust Edition",
        SCALED_WIDTH,
        SCALED_HEIGHT,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit frame rate
    window.set_target_fps(FPS);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if window.is_key_released(Key::F) {
            world.reset_world();
        }

        update_buffer(&mut world, &mut buffer);
        scale_buffer(&mut buffer, &mut scaled_buffer);

        // We unwrap here as we want this code to exit if it fails
        window
            .update_with_buffer(&scaled_buffer, SCALED_WIDTH, SCALED_HEIGHT)
            .unwrap();

        // Update the GOL world
        world.tick();
    }
}


fn update_buffer(world: &mut World, buffer: &mut Vec<u32>) {
    let mut count: u32 = 0;
    let height = HEIGHT as u32;
    let width = WIDTH as u32;

    for iter in buffer.iter_mut() {
        let row = count / height;
        let col = count % width; 
        let cell = world.get_value_at(row, col);
        if cell == Cell::Dead {
            *iter = DEAD_COLOR;
        } else {
            *iter = LIVE_COLOR;
        }
        count += 1;
    }
}

fn scale_buffer(buffer: &mut Vec<u32>, scaled_buffer: &mut Vec<u32>) {
    // Fill the window buffer and scale it to the window size
    for y in 0..HEIGHT {
        for x in 0..WIDTH {

            let color = buffer[y * WIDTH + x];

            for dy in 0..PIXEL_SIZE {
                for dx in 0..PIXEL_SIZE {
                    let sx = x * PIXEL_SIZE + dx;
                    let sy = y * PIXEL_SIZE + dy;
                    scaled_buffer[sy * SCALED_WIDTH + sx] = color;
                }
            }
        }
    }
}