use macroquad::prelude::*;

mod block;
mod car;
mod city;
mod constants;
mod input;
mod intersection;
mod led_chars;
mod led_display_object;
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
// Helper Functions
// ============================================================================

/// Creates the LED display block
///
/// This block is positioned between the first and second vertical roads
/// in the top row of the city grid.
fn create_led_display_block() -> block::Block {
    use block::Block;
    use led_display_object::LEDDisplay;
    use constants::road_network::{VERTICAL_ROAD_POSITIONS, HORIZONTAL_ROAD_POSITIONS};
    use constants::visual::ROAD_WIDTH;

    let v1 = VERTICAL_ROAD_POSITIONS[0];
    let v2 = VERTICAL_ROAD_POSITIONS[1];
    let h1 = HORIZONTAL_ROAD_POSITIONS[0];

    let block_x = v1 + (ROAD_WIDTH / 2.0) / screen_width();
    let block_y = 0.0;
    let block_width = v2 - (ROAD_WIDTH / 2.0) / screen_width() - block_x;
    let block_height = h1 - (ROAD_WIDTH / 2.0) / screen_height();

    let mut display_block = Block::new(block_x, block_y, block_width, block_height, 0);

    // Add LED display to the block
    let led = LEDDisplay::new("  WELCOME TO CITY  ")
        .with_position(0.1, 0.3)
        .with_size(0.8, 0.4);
    display_block.add_object(Box::new(led));

    display_block
}

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

    // Add grass blocks to the city
    use block::generate_grass_blocks;
    let grass_blocks = generate_grass_blocks();
    for grass_block in grass_blocks {
        city.add_block(grass_block);
    }

    // Create and add LED display block
    city.add_block(create_led_display_block());

    // Initialize window state tracking
    let mut window_state = WindowState::new();

    // Initialize control modes
    let mut all_lights_red = false; // Emergency traffic stop mode
    let mut danger_mode = false;     // Danger warning on LED display
    let mut barrier_open = false;    // Barrier gate state (false = closed/down)

    // ========================================================================
    // Main Game Loop
    // ========================================================================

    loop {
        let dt = get_frame_time();
        let current_time = get_time();

        // --------------------------------------------------------------------
        // Input Processing
        // --------------------------------------------------------------------

        let (new_all_lights_red, new_danger_mode, toggle_scada, reset_scada, toggle_barrier) =
            handle_input(all_lights_red, danger_mode);
        all_lights_red = new_all_lights_red;
        danger_mode = new_danger_mode;

        // Handle SCADA toggle for all buildings
        if toggle_scada {
            city.toggle_all_scada();
        }

        // Handle SCADA reset
        if reset_scada {
            city.reset_all_scada();
        }

        // Handle barrier toggle
        if toggle_barrier {
            barrier_open = !barrier_open;
        }

        // --------------------------------------------------------------------
        // Window Resize Handling
        // --------------------------------------------------------------------

        if window_state.check_resize(RESIZE_THRESHOLD) {
            // Clear all cars on resize to prevent positioning issues
            // Cars will naturally respawn at correct positions
            city.clear_cars();

            // Regenerate all blocks with new screen dimensions
            // Since ROAD_WIDTH is in pixels, percentage calculations need to be updated
            city.clear_blocks();

            // Recreate grass blocks with updated percentages
            let grass_blocks = generate_grass_blocks();
            for grass_block in grass_blocks {
                city.add_block(grass_block);
            }

            // Recreate LED display block with updated percentages
            city.add_block(create_led_display_block());
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
        city.render_environment(current_time, danger_mode, barrier_open);
        city.render_traffic(all_lights_red);
        city.render_overlays(current_time, danger_mode, barrier_open);

        // Present frame and wait for next
        next_frame().await;
    }
}
