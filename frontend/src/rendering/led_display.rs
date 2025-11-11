//! LED matrix display rendering - scrolling text and danger warnings

use crate::constants::{
    led::*,
    rendering::*,
    road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS},
    visual::{DEPTH_OFFSET, ROAD_WIDTH},
};
use crate::led_chars::get_led_char_pattern;
use macroquad::prelude::*;

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

        let scroll_speed = if danger_mode { 0.0 } else { LED_SCROLL_SPEED };

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

// ============================================================================
// Configurable LED Display API (for BlockObjects)
// ============================================================================

/// Draws an LED display at a specific position with custom configuration
///
/// This is the core rendering function used by LED Display BlockObjects.
///
/// # Arguments
/// * `x` - X position in pixels
/// * `y` - Y position in pixels  
/// * `width` - Width in pixels
/// * `height` - Height in pixels
/// * `text` - Text to display
/// * `mode` - Display mode
/// * `theme` - Color theme
/// * `time` - Current time for animations
pub fn draw_led_display_at(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: &str,
    mode: &crate::led_display_object::LEDDisplayMode,
    theme: &crate::led_display_object::LEDColorTheme,
    time: f64,
) {
    use crate::led_display_object::LEDDisplayMode;
    
    // Outer frame
    draw_rectangle(
        x - FRAME_THICKNESS,
        y - FRAME_THICKNESS,
        width + FRAME_THICKNESS * 2.0,
        height + FRAME_THICKNESS * 2.0,
        FRAME_COLOR_OUTER,
    );

    // Inner frame (beveled effect)
    draw_rectangle(
        x - FRAME_THICKNESS / 2.0,
        y - FRAME_THICKNESS / 2.0,
        width + FRAME_THICKNESS,
        height + FRAME_THICKNESS,
        FRAME_COLOR_INNER,
    );

    // LED display background
    draw_rectangle(x, y, width, height, LED_BG_COLOR);

    // Inner bezel
    draw_rectangle_lines(x, y, width, height, 2.0, LED_BORDER_COLOR);

    // Corner screws
    let screw_offset = FRAME_THICKNESS * 0.3;
    draw_screw(x - FRAME_THICKNESS + screw_offset, y - FRAME_THICKNESS + screw_offset);
    draw_screw(x + width + FRAME_THICKNESS - screw_offset, y - FRAME_THICKNESS + screw_offset);
    draw_screw(x - FRAME_THICKNESS + screw_offset, y + height + FRAME_THICKNESS - screw_offset);
    draw_screw(x + width + FRAME_THICKNESS - screw_offset, y + height + FRAME_THICKNESS - screw_offset);

    let dot_pitch = LED_DOT_SIZE + LED_SPACING;
    let matrix_width = width - (LED_PADDING * 2.0);
    let matrix_height = height - (LED_PADDING * 2.0);
    let cols = (matrix_width / dot_pitch) as usize;
    let rows = (matrix_height / dot_pitch) as usize;

    // Draw LED matrix background (all dots dim)
    for row in 0..rows {
        for col in 0..cols {
            let dot_x = x + LED_PADDING + (col as f32 * dot_pitch);
            let dot_y = y + LED_PADDING + (row as f32 * dot_pitch);
            draw_rectangle(dot_x, dot_y, LED_DOT_SIZE, LED_DOT_SIZE, theme.off_color);
        }
    }

    // Show text based on mode
    let show_text = match mode {
        LEDDisplayMode::Flashing => ((time * LED_FLASH_SPEED as f64) % 1.0) > 0.5,
        _ => true,
    };

    if show_text {
        let is_scrolling = matches!(mode, LEDDisplayMode::Scrolling);
        let scroll_speed = if is_scrolling { LED_SCROLL_SPEED } else { 0.0 };

        let start_col = if !is_scrolling {
            let text_width_dots = text.len() * (LED_CHAR_WIDTH + LED_CHAR_SPACING);
            ((cols as i32 - text_width_dots as i32) / 2).max(0)
        } else {
            0
        };

        let total_text_width = text.len() * (LED_CHAR_WIDTH + LED_CHAR_SPACING);
        let scroll_offset_dots = if is_scrolling {
            ((time as f32 * scroll_speed / dot_pitch) as usize) % total_text_width
        } else {
            0
        };

        let instances = if is_scrolling { 2 } else { 1 };

        for instance in 0..instances {
            for (char_idx, c) in text.chars().enumerate() {
                let char_col_start = if is_scrolling {
                    let base_pos = (char_idx * (LED_CHAR_WIDTH + LED_CHAR_SPACING)) as i32 - scroll_offset_dots as i32;
                    base_pos + (instance * total_text_width as i32)
                } else {
                    start_col + (char_idx * (LED_CHAR_WIDTH + LED_CHAR_SPACING)) as i32
                };

                let pattern = get_led_char_pattern(c);

                for row in 0..LED_CHAR_HEIGHT {
                    for col in 0..LED_CHAR_WIDTH {
                        let led_col = char_col_start + col as i32;
                        if led_col < 0 || led_col >= cols as i32 {
                            continue;
                        }

                        if pattern[row] & (1 << (LED_CHAR_WIDTH - 1 - col)) != 0 {
                            let dot_x = x + LED_PADDING + (led_col as f32 * dot_pitch);
                            let v_center = rows.saturating_sub(LED_CHAR_HEIGHT) / 2;
                            let dot_y = y + LED_PADDING + ((v_center + row) as f32 * dot_pitch);
                            draw_rectangle(dot_x, dot_y, LED_DOT_SIZE, LED_DOT_SIZE, theme.on_color);
                            draw_rectangle(
                                dot_x - 0.5,
                                dot_y - 0.5,
                                LED_DOT_SIZE + 1.0,
                                LED_DOT_SIZE + 1.0,
                                Color::new(theme.on_color.r, theme.on_color.g, theme.on_color.b, 0.3),
                            );
                        }
                    }
                }
            }
        }
    }

    // Support poles
    let pole_start_y = y + height + FRAME_THICKNESS;
    let pole_spacing = width * 0.25;
    draw_pole(x + pole_spacing, pole_start_y);
    draw_pole(x + width - pole_spacing, pole_start_y);
}

fn draw_screw(x: f32, y: f32) {
    draw_circle(x, y, SCREW_SIZE / 2.0, SCREW_COLOR);
    draw_circle(x, y, SCREW_SIZE / 4.0, SCREW_CENTER_COLOR);
}

fn draw_pole(x: f32, y: f32) {
    draw_rectangle(x - POLE_WIDTH / 2.0, y, POLE_WIDTH, POLE_HEIGHT, POLE_COLOR);
    draw_rectangle(
        x - POLE_WIDTH / 2.0 + POLE_WIDTH,
        y,
        DEPTH_OFFSET * 0.6,
        POLE_HEIGHT,
        POLE_DEPTH_COLOR,
    );
}
