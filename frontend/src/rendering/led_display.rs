//! LED matrix display rendering - configurable displays for BlockObjects
//!
//! This module provides the core rendering functions for LED matrix displays.
//! LED displays are typically created as BlockObjects using the led_display_object module.

use crate::constants::{
    led::*,
    visual::DEPTH_OFFSET,
};
use crate::led_chars::get_led_char_pattern;
use macroquad::prelude::*;

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
    draw_screw(
        x - FRAME_THICKNESS + screw_offset,
        y - FRAME_THICKNESS + screw_offset,
    );
    draw_screw(
        x + width + FRAME_THICKNESS - screw_offset,
        y - FRAME_THICKNESS + screw_offset,
    );
    draw_screw(
        x - FRAME_THICKNESS + screw_offset,
        y + height + FRAME_THICKNESS - screw_offset,
    );
    draw_screw(
        x + width + FRAME_THICKNESS - screw_offset,
        y + height + FRAME_THICKNESS - screw_offset,
    );

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
                    let base_pos = (char_idx * (LED_CHAR_WIDTH + LED_CHAR_SPACING)) as i32
                        - scroll_offset_dots as i32;
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
                            draw_rectangle(
                                dot_x,
                                dot_y,
                                LED_DOT_SIZE,
                                LED_DOT_SIZE,
                                theme.on_color,
                            );
                            draw_rectangle(
                                dot_x - 0.5,
                                dot_y - 0.5,
                                LED_DOT_SIZE + 1.0,
                                LED_DOT_SIZE + 1.0,
                                Color::new(
                                    theme.on_color.r,
                                    theme.on_color.g,
                                    theme.on_color.b,
                                    0.3,
                                ),
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
