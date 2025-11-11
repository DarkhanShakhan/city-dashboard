//! Rendering utility functions

use macroquad::prelude::*;

/// Draws a rectangle with rounded corners
///
/// # Arguments
/// * `x` - X position of the rectangle
/// * `y` - Y position of the rectangle
/// * `width` - Width of the rectangle
/// * `height` - Height of the rectangle
/// * `corner_radius` - Radius of the rounded corners
/// * `color` - Fill color
pub fn draw_rounded_rectangle(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    corner_radius: f32,
    color: Color,
) {
    let radius = corner_radius.min(width / 2.0).min(height / 2.0);

    // Draw center rectangle (full width and height minus corners)
    draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);

    // Draw left and right side rectangles to fill corner gaps
    draw_rectangle(x, y + radius, radius, height - 2.0 * radius, color);
    draw_rectangle(
        x + width - radius,
        y + radius,
        radius,
        height - 2.0 * radius,
        color,
    );

    // Draw corner circles
    draw_circle(x + radius, y + radius, radius, color); // Top-left
    draw_circle(x + width - radius, y + radius, radius, color); // Top-right
    draw_circle(x + radius, y + height - radius, radius, color); // Bottom-left
    draw_circle(x + width - radius, y + height - radius, radius, color); // Bottom-right
}
