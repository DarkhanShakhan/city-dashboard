//! Road rendering - center lines and lane markings

use crate::constants::{
    rendering::{DASH_GAP, DASH_LENGTH, LINE_WIDTH},
    road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS},
    visual::LINE_COLOR,
};
use macroquad::prelude::*;

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
