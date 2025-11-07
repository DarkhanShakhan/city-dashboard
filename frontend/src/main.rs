use macroquad::prelude::*;

mod car;
mod intersection;
mod models;
mod rendering;
mod traffic_light;

use car::{spawn_car, update_cars};
use intersection::generate_intersections;
use models::Car;
use rendering::{draw_car, draw_grass_blocks, draw_road_lines, draw_intersection_markings};
use traffic_light::draw_traffic_lights;

const ROAD_COLOR: Color = GRAY;

#[macroquad::main("City Dashboard")]
async fn main() -> Result<(), macroquad::Error> {
    let mut cars: Vec<Car> = Vec::new();
    let mut last_spawn_time = 0.0;
    let spawn_interval = 1.5; // Spawn a car every 1.5 seconds

    // Generate all intersections at startup
    let intersections = generate_intersections();

    // Track window size to detect resizing
    let mut prev_screen_width = screen_width();
    let mut prev_screen_height = screen_height();

    loop {
        let dt = get_frame_time();

        // Check if window was resized
        let current_width = screen_width();
        let current_height = screen_height();
        if (current_width - prev_screen_width).abs() > 1.0 || (current_height - prev_screen_height).abs() > 1.0 {
            // Window resized - clear all cars to avoid position issues
            cars.clear();
            prev_screen_width = current_width;
            prev_screen_height = current_height;
        }

        clear_background(ROAD_COLOR); // Roads are now the background

        draw_grass_blocks();
        draw_road_lines();
        draw_intersection_markings(&intersections);
        draw_traffic_lights(&intersections);

        // Spawn new cars
        if get_time() - last_spawn_time > spawn_interval {
            spawn_car(&mut cars);
            last_spawn_time = get_time();
        }

        // Update and draw cars
        update_cars(&mut cars, &intersections, dt);
        for car in &cars {
            draw_car(car);
        }

        next_frame().await;
    }
}
