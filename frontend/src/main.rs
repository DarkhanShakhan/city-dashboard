use macroquad::prelude::*;

mod block;
mod car;
mod city;
mod constants;
mod input;
mod intersection;
mod models;
mod rendering;
mod road;
mod spawner;
mod traffic_light;

use city::City;
use input::{handle_input, WindowState};
use intersection::generate_intersections;

// ============================================================================
// Configuration Constants
// ============================================================================

use constants::{visual::ROAD_COLOR, window::RESIZE_THRESHOLD};

// ============================================================================
// Main Application
// ============================================================================

#[macroquad::main("City Dashboard")]
async fn main() -> Result<(), macroquad::Error> {
    // ========================================================================
    // Initialization
    // ========================================================================

    // Initialize city with intersections
    let mut city = City::new();
    let intersections = generate_intersections();
    for intersection in intersections {
        city.add_intersection(intersection);
    }

    // Initialize window state tracking
    let mut window_state = WindowState::new();

    // Initialize control modes
    let mut all_lights_red = false; // Emergency traffic stop mode
    let mut danger_mode = false;     // Danger warning on LED display

    // ========================================================================
    // Main Game Loop
    // ========================================================================

    loop {
        let dt = get_frame_time();
        let current_time = get_time();

        // --------------------------------------------------------------------
        // Input Processing
        // --------------------------------------------------------------------

        (all_lights_red, danger_mode) = handle_input(all_lights_red, danger_mode);

        // --------------------------------------------------------------------
        // Window Resize Handling
        // --------------------------------------------------------------------

        if window_state.check_resize(RESIZE_THRESHOLD) {
            // Clear all cars on resize to prevent positioning issues
            // Cars will naturally respawn at correct positions
            city.clear_cars();
        }

        // --------------------------------------------------------------------
        // Update Phase
        // --------------------------------------------------------------------

        city.update(dt, all_lights_red);

        // --------------------------------------------------------------------
        // Render Phase
        // --------------------------------------------------------------------

        // Clear screen with road color
        clear_background(ROAD_COLOR);

        // Render in layers: environment -> traffic -> overlays
        city.render_environment();
        city.render_traffic(all_lights_red);
        city.render_overlays(current_time, danger_mode);

        // Present frame and wait for next
        next_frame().await;
    }
}
