//! Visual rendering system for the city traffic simulation
//!
//! This module handles all drawing operations for the application:
//! - Environment rendering (grass blocks, roads, intersection markings)
//! - Vehicle rendering with directional sprites
//! - LED display with scrolling text and danger warnings
//! - 2.5D depth effects for visual polish
//!
//! The rendering pipeline is organized into distinct layers:
//! 1. Background (grass blocks with depth edges)
//! 2. Road markings (center lines, crosswalks)
//! 3. Traffic elements (traffic lights, cars)
//! 4. UI overlays (LED display)

use crate::constants::{
    led::*,
    rendering::*,
    road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS},
    vehicle::{CAR_HEIGHT, CAR_WIDTH},
    visual::*,
};
use crate::intersection::Intersection;
use crate::led_chars::get_led_char_pattern;
use crate::models::{Car, Direction};
use macroquad::prelude::*;

// ============================================================================
// Road Rendering
// ============================================================================

/// Draws dashed center lines on all roads
///
/// Creates yellow-white dashed lines to mark road centers:
/// - Vertical lines on 3 vertical roads
/// - Horizontal lines on 2 horizontal roads
///
/// Lines are dashed with 15px segments separated by 10px gaps
pub fn draw_road_lines() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Road positions
    let vertical_positions = [
        screen_width * VERTICAL_ROAD_POSITIONS[0],
        screen_width * VERTICAL_ROAD_POSITIONS[1],
        screen_width * VERTICAL_ROAD_POSITIONS[2],
    ];

    let horizontal_positions = [
        screen_height * HORIZONTAL_ROAD_POSITIONS[0],
        screen_height * HORIZONTAL_ROAD_POSITIONS[1],
    ];

    // Draw vertical road center lines (dashed)
    for &x in &vertical_positions {
        let mut y = 0.0;
        while y < screen_height {
            draw_rectangle(x - LINE_WIDTH / 2.0, y, LINE_WIDTH, DASH_LENGTH, LINE_COLOR);
            y += DASH_LENGTH + DASH_GAP;
        }
    }

    // Draw horizontal road center lines (dashed)
    for &y in &horizontal_positions {
        let mut x = 0.0;
        while x < screen_width {
            draw_rectangle(x, y - LINE_WIDTH / 2.0, DASH_LENGTH, LINE_WIDTH, LINE_COLOR);
            x += DASH_LENGTH + DASH_GAP;
        }
    }
}

// ============================================================================
// Intersection Rendering
// ============================================================================

/// Draws intersection markings and crosswalks
///
/// For each intersection, draws:
/// - Subtle white box outline marking the intersection area
/// - Zebra-striped crosswalks on all 4 sides (top, bottom, left, right)
///
/// # Arguments
/// * `intersections` - All intersections to draw markings for
pub fn draw_intersection_markings(intersections: &[Intersection]) {

    for intersection in intersections {
        let int_x = intersection.x();
        let int_y = intersection.y();

        // Draw intersection box outline
        let box_size = INTERSECTION_SIZE * 2.0;
        draw_rectangle_lines(
            int_x - box_size / 2.0,
            int_y - box_size / 2.0,
            box_size,
            box_size,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.3),
        );

        // Draw crosswalks (zebra stripes) on all 4 sides

        // Top crosswalk (horizontal stripes)
        let top_y = int_y - CROSSWALK_DISTANCE;
        let mut stripe_x = int_x - ROAD_WIDTH / 2.0;
        while stripe_x < int_x + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                stripe_x,
                top_y - CROSSWALK_WIDTH / 2.0,
                CROSSWALK_STRIPE_WIDTH,
                CROSSWALK_WIDTH,
                INTERSECTION_MARK_COLOR,
            );
            stripe_x += CROSSWALK_STRIPE_WIDTH + CROSSWALK_STRIPE_GAP;
        }

        // Bottom crosswalk (horizontal stripes)
        let bottom_y = int_y + CROSSWALK_DISTANCE;
        stripe_x = int_x - ROAD_WIDTH / 2.0;
        while stripe_x < int_x + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                stripe_x,
                bottom_y - CROSSWALK_WIDTH / 2.0,
                CROSSWALK_STRIPE_WIDTH,
                CROSSWALK_WIDTH,
                INTERSECTION_MARK_COLOR,
            );
            stripe_x += CROSSWALK_STRIPE_WIDTH + CROSSWALK_STRIPE_GAP;
        }

        // Left crosswalk (vertical stripes)
        let left_x = int_x - CROSSWALK_DISTANCE;
        let mut stripe_y = int_y - ROAD_WIDTH / 2.0;
        while stripe_y < int_y + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                left_x - CROSSWALK_WIDTH / 2.0,
                stripe_y,
                CROSSWALK_WIDTH,
                CROSSWALK_STRIPE_WIDTH,
                INTERSECTION_MARK_COLOR,
            );
            stripe_y += CROSSWALK_STRIPE_WIDTH + CROSSWALK_STRIPE_GAP;
        }

        // Right crosswalk (vertical stripes)
        let right_x = int_x + CROSSWALK_DISTANCE;
        stripe_y = int_y - ROAD_WIDTH / 2.0;
        while stripe_y < int_y + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                right_x - CROSSWALK_WIDTH / 2.0,
                stripe_y,
                CROSSWALK_WIDTH,
                CROSSWALK_STRIPE_WIDTH,
                INTERSECTION_MARK_COLOR,
            );
            stripe_y += CROSSWALK_STRIPE_WIDTH + CROSSWALK_STRIPE_GAP;
        }
    }
}

// ============================================================================
// Environment Rendering
// ============================================================================

/// Draws grass blocks in all non-road areas
///
/// Creates a grid of green rectangular blocks between roads, with 2.5D
/// depth edges for visual polish. The grid pattern is calculated from
/// road positions to fill all spaces not occupied by roads.
///
/// # Grid Layout
/// - 3 vertical roads create 4 vertical sections (left edge, 3 gaps, right edge)
/// - 2 horizontal roads create 3 horizontal sections (top edge, 1 gap, bottom edge)
/// - Total: 12 grass blocks in a 4×3 grid
pub fn draw_grass_blocks() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // 3 vertical roads (outer roads closer to edges)
    let vertical_positions = [
        screen_width * VERTICAL_ROAD_POSITIONS[0],
        screen_width * VERTICAL_ROAD_POSITIONS[1],
        screen_width * VERTICAL_ROAD_POSITIONS[2],
    ];

    // 2 horizontal roads (closer to top and bottom edges)
    let horizontal_positions = [
        screen_height * HORIZONTAL_ROAD_POSITIONS[0],
        screen_height * HORIZONTAL_ROAD_POSITIONS[1],
    ];

    // Calculate grass block boundaries
    // We need to draw grass blocks in the spaces between roads
    let x_boundaries = [
        0.0,
        vertical_positions[0] - ROAD_WIDTH / 2.0,
        vertical_positions[0] + ROAD_WIDTH / 2.0,
        vertical_positions[1] - ROAD_WIDTH / 2.0,
        vertical_positions[1] + ROAD_WIDTH / 2.0,
        vertical_positions[2] - ROAD_WIDTH / 2.0,
        vertical_positions[2] + ROAD_WIDTH / 2.0,
        screen_width,
    ];

    let y_boundaries = [
        0.0,
        horizontal_positions[0] - ROAD_WIDTH / 2.0,
        horizontal_positions[0] + ROAD_WIDTH / 2.0,
        horizontal_positions[1] - ROAD_WIDTH / 2.0,
        horizontal_positions[1] + ROAD_WIDTH / 2.0,
        screen_height,
    ];

    // Draw grass blocks in a grid (skip road areas)
    for i in (0..x_boundaries.len() - 1).step_by(2) {
        for j in (0..y_boundaries.len() - 1).step_by(2) {
            let x = x_boundaries[i];
            let y = y_boundaries[j];
            let width = x_boundaries[i + 1] - x;
            let height = y_boundaries[j + 1] - y;

            // Draw grass block
            draw_rectangle(x, y, width, height, GRASS_COLOR);

            // Add depth edge for 2.5D effect
            draw_rectangle(
                x + width,
                y,
                DEPTH_OFFSET,
                height,
                GRASS_DEPTH_COLOR,
            );
            draw_rectangle(
                x,
                y + height,
                width + DEPTH_OFFSET,
                DEPTH_OFFSET,
                GRASS_DEPTH_COLOR,
            );
        }
    }
}

// ============================================================================
// Vehicle Rendering
// ============================================================================

/// Draws a car with directional sprite and depth effect
///
/// Renders a colored car rectangle with:
/// - Orientation based on travel direction
/// - 2.5D depth edges (darker shading on right and bottom)
/// - Windshield window positioned based on direction
///
/// # Arguments
/// * `car` - The car to render
///
/// # Car Dimensions
/// - Width: 20px, Height: 35px (rotated based on direction)
/// - Window size: ~60% of car width, ~30% of car height
pub fn draw_car(car: &Car) {
    let car_x = car.x();
    let car_y = car.y();

    let (width, height) = match car.direction {
        Direction::Down | Direction::Up => (CAR_WIDTH, CAR_HEIGHT),
        Direction::Left | Direction::Right => (CAR_HEIGHT, CAR_WIDTH),
    };

    // Draw car body
    draw_rectangle(
        car_x - width / 2.0,
        car_y - height / 2.0,
        width,
        height,
        car.color,
    );

    // Draw depth edge
    draw_rectangle(
        car_x - width / 2.0 + width,
        car_y - height / 2.0,
        DEPTH_OFFSET,
        height,
        Color::new(car.color.r * 0.5, car.color.g * 0.5, car.color.b * 0.5, 1.0),
    );
    draw_rectangle(
        car_x - width / 2.0,
        car_y - height / 2.0 + height,
        width,
        DEPTH_OFFSET,
        Color::new(car.color.r * 0.5, car.color.g * 0.5, car.color.b * 0.5, 1.0),
    );

    // Draw windows
    match car.direction {
        Direction::Down => {
            draw_rectangle(
                car_x - width / 3.0,
                car_y - height / 4.0,
                width * 0.6,
                height * 0.3,
                CAR_WINDOW_COLOR,
            );
        }
        Direction::Up => {
            draw_rectangle(
                car_x - width / 3.0,
                car_y - height / 6.0,
                width * 0.6,
                height * 0.3,
                CAR_WINDOW_COLOR,
            );
        }
        Direction::Right => {
            draw_rectangle(
                car_x - height / 6.0,
                car_y - width / 3.0,
                height * 0.3,
                width * 0.6,
                CAR_WINDOW_COLOR,
            );
        }
        Direction::Left => {
            draw_rectangle(
                car_x - height / 4.0,
                car_y - width / 3.0,
                height * 0.3,
                width * 0.6,
                CAR_WINDOW_COLOR,
            );
        }
    }
}

// ============================================================================
// Decorative Elements (Deprecated)
// ============================================================================

/// Placeholder for removed building/parking lot feature
///
/// This function was previously used to draw a guarded building with
/// parking lot, but that feature has been removed. Kept for compatibility
/// with the main rendering pipeline.
///
/// # Arguments
/// * `_time` - Unused simulation time
/// * `_cars` - Unused car list
pub fn draw_guarded_building(_time: f64, _cars: &[Car]) {
    // Function removed - no parking lot or buildings
}

// ============================================================================
// LED Display Rendering
// ============================================================================

/// Draws an LED matrix display with scrolling text or danger warning
///
/// Located in the second grass block of the first row (between first and
/// second vertical roads), this display shows:
/// - Normal mode: "WELCOME TO CITY" scrolling right-to-left in green
/// - Danger mode: "DANGER" centered and flashing in red
///
/// The display features:
/// - Realistic LED dot matrix (5x7 character patterns)
/// - All LED dots visible (bright when ON, dim when OFF)
/// - Industrial frame with corner screws
/// - Support poles underneath
/// - Seamless infinite scrolling in normal mode
/// - 3 flashes per second in danger mode
///
/// # Arguments
/// * `time` - Current simulation time for animation
/// * `danger_mode` - If true, shows red flashing "DANGER" text
pub fn draw_led_display(time: f64, danger_mode: bool) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Calculate position for second grass block in first row
    // First row is from y=0 to first horizontal road
    let vertical_positions = [
        screen_width * VERTICAL_ROAD_POSITIONS[0],
        screen_width * VERTICAL_ROAD_POSITIONS[1],
        screen_width * VERTICAL_ROAD_POSITIONS[2],
    ];

    let horizontal_positions = [
        screen_height * HORIZONTAL_ROAD_POSITIONS[0],
        screen_height * HORIZONTAL_ROAD_POSITIONS[1],
    ];

    // Second block in first row is between first and second vertical roads
    let block_x = vertical_positions[0] + ROAD_WIDTH / 2.0;
    let block_y = 0.0;
    let block_width =
        vertical_positions[1] - ROAD_WIDTH / 2.0 - vertical_positions[0] - ROAD_WIDTH / 2.0;
    let block_height = horizontal_positions[0] - ROAD_WIDTH / 2.0;

    // LED display dimensions (smaller, just for text)
    let display_width = block_width * 0.8;
    let display_x = block_x + (block_width - display_width) / 2.0;
    let display_y = block_y + (block_height - LED_DISPLAY_HEIGHT) / 2.0;

    // Outer frame
    draw_rectangle(
        display_x - FRAME_THICKNESS,
        display_y - FRAME_THICKNESS,
        display_width + FRAME_THICKNESS * 2.0,
        LED_DISPLAY_HEIGHT + FRAME_THICKNESS * 2.0,
        FRAME_COLOR_OUTER,
    );

    // Inner frame (beveled effect)
    draw_rectangle(
        display_x - FRAME_THICKNESS / 2.0,
        display_y - FRAME_THICKNESS / 2.0,
        display_width + FRAME_THICKNESS,
        LED_DISPLAY_HEIGHT + FRAME_THICKNESS,
        FRAME_COLOR_INNER,
    );

    // Draw LED display background
    draw_rectangle(
        display_x,
        display_y,
        display_width,
        LED_DISPLAY_HEIGHT,
        LED_BG_COLOR,
    );

    // Draw inner bezel/border
    draw_rectangle_lines(
        display_x,
        display_y,
        display_width,
        LED_DISPLAY_HEIGHT,
        2.0,
        LED_BORDER_COLOR,
    );

    // Corner screws for realism
    let screw_offset = FRAME_THICKNESS * 0.3;

    // Top-left screw
    draw_circle(
        display_x - FRAME_THICKNESS + screw_offset,
        display_y - FRAME_THICKNESS + screw_offset,
        SCREW_SIZE / 2.0,
        SCREW_COLOR,
    );
    draw_circle(
        display_x - FRAME_THICKNESS + screw_offset,
        display_y - FRAME_THICKNESS + screw_offset,
        SCREW_SIZE / 4.0,
        SCREW_CENTER_COLOR,
    );

    // Top-right screw
    draw_circle(
        display_x + display_width + FRAME_THICKNESS - screw_offset,
        display_y - FRAME_THICKNESS + screw_offset,
        SCREW_SIZE / 2.0,
        SCREW_COLOR,
    );
    draw_circle(
        display_x + display_width + FRAME_THICKNESS - screw_offset,
        display_y - FRAME_THICKNESS + screw_offset,
        SCREW_SIZE / 4.0,
        SCREW_CENTER_COLOR,
    );

    // Bottom-left screw
    draw_circle(
        display_x - FRAME_THICKNESS + screw_offset,
        display_y + LED_DISPLAY_HEIGHT + FRAME_THICKNESS - screw_offset,
        SCREW_SIZE / 2.0,
        SCREW_COLOR,
    );
    draw_circle(
        display_x - FRAME_THICKNESS + screw_offset,
        display_y + LED_DISPLAY_HEIGHT + FRAME_THICKNESS - screw_offset,
        SCREW_SIZE / 4.0,
        SCREW_CENTER_COLOR,
    );

    // Bottom-right screw
    draw_circle(
        display_x + display_width + FRAME_THICKNESS - screw_offset,
        display_y + LED_DISPLAY_HEIGHT + FRAME_THICKNESS - screw_offset,
        SCREW_SIZE / 2.0,
        SCREW_COLOR,
    );
    draw_circle(
        display_x + display_width + FRAME_THICKNESS - screw_offset,
        display_y + LED_DISPLAY_HEIGHT + FRAME_THICKNESS - screw_offset,
        SCREW_SIZE / 4.0,
        SCREW_CENTER_COLOR,
    );

    let dot_pitch = LED_DOT_SIZE + LED_SPACING;

    // Calculate LED matrix dimensions
    let matrix_width = display_width - (LED_PADDING * 2.0);
    let matrix_height = LED_DISPLAY_HEIGHT - (LED_PADDING * 2.0);
    let cols = (matrix_width / dot_pitch) as usize;
    let rows = (matrix_height / dot_pitch) as usize;

    // Determine LED colors based on mode
    let (led_on_color, led_off_color) = if danger_mode {
        (LED_DANGER_ON_COLOR, LED_DANGER_OFF_COLOR)
    } else {
        (LED_ON_COLOR, LED_OFF_COLOR)
    };

    // Draw LED dot matrix background (all dots dim)
    for row in 0..rows {
        for col in 0..cols {
            let dot_x = display_x + LED_PADDING + (col as f32 * dot_pitch);
            let dot_y = display_y + LED_PADDING + (row as f32 * dot_pitch);
            draw_rectangle(dot_x, dot_y, LED_DOT_SIZE, LED_DOT_SIZE, led_off_color);
        }
    }

    // Check if we should show the text (flashing effect in danger mode)
    let show_text = if danger_mode {
        ((time * LED_FLASH_SPEED as f64) % 1.0) > 0.5 // Flash on/off
    } else {
        true // Always show in normal mode
    };

    if show_text {
        // Text setup
        let scroll_text = if danger_mode {
            "DANGER"
        } else {
            "  WELCOME TO CITY  "
        };

        let scroll_speed = if danger_mode {
            0.0
        } else {
            LED_SCROLL_SPEED
        };

        // Calculate starting position for centered text in danger mode
        let danger_start_col = if danger_mode {
            // Calculate actual text width (number of characters * (width + spacing))
            let text_width_dots = scroll_text.len() * (LED_CHAR_WIDTH + LED_CHAR_SPACING);
            // Center it: (total_columns - text_width) / 2
            ((cols as i32 - text_width_dots as i32) / 2).max(0)
        } else {
            0
        };

        // Calculate scroll position in terms of dot columns for normal mode
        let total_text_width = scroll_text.len() * (LED_CHAR_WIDTH + LED_CHAR_SPACING);
        let scroll_offset_dots = if danger_mode {
            0
        } else {
            // Seamless infinite scroll - loop when text completes full cycle
            ((time as f32 * scroll_speed / dot_pitch) as usize) % total_text_width
        };

        // Draw text (twice for seamless scrolling loop in normal mode)
        let instances = if danger_mode { 1 } else { 2 };

        for instance in 0..instances {
            for (char_idx, c) in scroll_text.chars().enumerate() {
                let char_col_start = if danger_mode {
                    danger_start_col + (char_idx * (LED_CHAR_WIDTH + LED_CHAR_SPACING)) as i32
                } else {
                    let base_pos = (char_idx * (LED_CHAR_WIDTH + LED_CHAR_SPACING)) as i32
                        - scroll_offset_dots as i32;
                    base_pos + (instance * total_text_width as i32)
                };

                // Get the LED pattern for this character
                let pattern = get_led_char_pattern(c);

                // Draw the character's LED pattern
                for row in 0..LED_CHAR_HEIGHT {
                    for col in 0..LED_CHAR_WIDTH {
                        let led_col = char_col_start + col as i32;

                        // Skip if outside display bounds
                        if led_col < 0 || led_col >= cols as i32 {
                            continue;
                        }

                        // Check if this LED should be on
                        if pattern[row] & (1 << (LED_CHAR_WIDTH - 1 - col)) != 0 {
                            let dot_x = display_x + LED_PADDING + (led_col as f32 * dot_pitch);
                            let dot_y = display_y
                                + LED_PADDING
                                + ((rows / 2 - LED_CHAR_HEIGHT / 2 + row) as f32 * dot_pitch);

                            // Draw bright LED with slight glow effect
                            draw_rectangle(dot_x, dot_y, LED_DOT_SIZE, LED_DOT_SIZE, led_on_color);
                            // Small glow
                            draw_rectangle(
                                dot_x - 0.5,
                                dot_y - 0.5,
                                LED_DOT_SIZE + 1.0,
                                LED_DOT_SIZE + 1.0,
                                Color::new(led_on_color.r, led_on_color.g, led_on_color.b, 0.3),
                            );
                        }
                    }
                }
            }
        }
    }

    // Draw small support poles underneath the display
    let pole_start_y = display_y + LED_DISPLAY_HEIGHT + FRAME_THICKNESS;
    let pole_spacing = display_width * 0.25;

    // Left pole
    draw_rectangle(
        display_x + pole_spacing - POLE_WIDTH / 2.0,
        pole_start_y,
        POLE_WIDTH,
        POLE_HEIGHT,
        POLE_COLOR,
    );
    // Pole depth
    draw_rectangle(
        display_x + pole_spacing - POLE_WIDTH / 2.0 + POLE_WIDTH,
        pole_start_y,
        DEPTH_OFFSET * 0.6,
        POLE_HEIGHT,
        POLE_DEPTH_COLOR,
    );

    // Right pole
    draw_rectangle(
        display_x + display_width - pole_spacing - POLE_WIDTH / 2.0,
        pole_start_y,
        POLE_WIDTH,
        POLE_HEIGHT,
        POLE_COLOR,
    );
    // Pole depth
    draw_rectangle(
        display_x + display_width - pole_spacing - POLE_WIDTH / 2.0 + POLE_WIDTH,
        pole_start_y,
        DEPTH_OFFSET * 0.6,
        POLE_HEIGHT,
        POLE_DEPTH_COLOR,
    );
}
