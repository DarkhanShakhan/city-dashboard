//! Environment rendering - grass blocks, intersections, and crosswalks

use crate::constants::{
    rendering::*,
    road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS},
    visual::*,
};
use crate::intersection::Intersection;
use crate::rendering::draw_rounded_rectangle;
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

// NOTE: Grass blocks are now rendered via the Block/BlockObject system.
// See block::generate_grass_blocks() for the new implementation.
// This procedural approach has been replaced with an object-oriented
// approach where Grass objects are added to Block containers.
