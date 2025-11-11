//! Environment rendering - grass blocks, intersections, and crosswalks

use crate::constants::{
    rendering::*,
    road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS},
    visual::*,
};
use crate::intersection::Intersection;
use macroquad::prelude::*;

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
