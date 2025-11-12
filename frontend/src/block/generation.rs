//! Block generation functions
//!
//! Provides functions for generating the city grid of blocks.

use crate::block::{Block, Building, Fence, Grass};
use crate::constants::{
    road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS},
    visual::ROAD_WIDTH,
};
use macroquad::prelude::*;

/// Generates all grass blocks for the city grid
///
/// Creates a 4×3 grid of blocks (12 total) in the spaces between roads.
/// Each block contains a Grass object as the base. Some blocks may have
/// additional objects (like Buildings) placed on top of the grass.
///
/// # Returns
/// Vector of Block instances, each containing at least a Grass object
pub fn generate_grass_blocks() -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut block_id = 1; // Start from 1 (0 is reserved for LED display block)

    // Calculate boundaries in percentage coordinates
    let x_boundaries_percent = [
        0.0,
        VERTICAL_ROAD_POSITIONS[0] - (ROAD_WIDTH / 2.0) / screen_width(),
        VERTICAL_ROAD_POSITIONS[0] + (ROAD_WIDTH / 2.0) / screen_width(),
        VERTICAL_ROAD_POSITIONS[1] - (ROAD_WIDTH / 2.0) / screen_width(),
        VERTICAL_ROAD_POSITIONS[1] + (ROAD_WIDTH / 2.0) / screen_width(),
        VERTICAL_ROAD_POSITIONS[2] - (ROAD_WIDTH / 2.0) / screen_width(),
        VERTICAL_ROAD_POSITIONS[2] + (ROAD_WIDTH / 2.0) / screen_width(),
        1.0,
    ];

    let y_boundaries_percent = [
        0.0,
        HORIZONTAL_ROAD_POSITIONS[0] - (ROAD_WIDTH / 2.0) / screen_height(),
        HORIZONTAL_ROAD_POSITIONS[0] + (ROAD_WIDTH / 2.0) / screen_height(),
        HORIZONTAL_ROAD_POSITIONS[1] - (ROAD_WIDTH / 2.0) / screen_height(),
        HORIZONTAL_ROAD_POSITIONS[1] + (ROAD_WIDTH / 2.0) / screen_height(),
        1.0,
    ];

    // Create blocks in grid pattern (skip road areas)
    // Block layout (0-indexed, second row = row 1, third block = column 2):
    // Row 0: blocks 1,  4,  7, 10
    // Row 1: blocks 2,  5,  8, 11  <- block 8 is second row, third column
    // Row 2: blocks 3,  6,  9, 12
    let building_color = Color::new(0.5, 0.6, 0.7, 1.0);
    for i in (0..x_boundaries_percent.len() - 1).step_by(2) {
        for j in (0..y_boundaries_percent.len() - 1).step_by(2) {
            let x_percent = x_boundaries_percent[i];
            let y_percent = y_boundaries_percent[j];
            let width_percent = x_boundaries_percent[i + 1] - x_percent;
            let height_percent = y_boundaries_percent[j + 1] - y_percent;

            // Create block
            let mut block = Block::new(
                x_percent,
                y_percent,
                width_percent,
                height_percent,
                block_id,
            );

            // Add grass to all blocks as the base
            block.add_object(Box::new(Grass::fill()));

            // Block 1 - top left corner
            if block_id == 1 {
                block.add_object(Box::new(Building::new(
                    0.20,           // x_offset: 20% from left
                    0.30,           // y_offset: 30% from top
                    0.50,           // width: 50% of block width
                    50.0,           // height: 50 pixels tall
                    0.40,           // depth: 40% of block height
                    8.0,            // corner_radius: 8 pixels
                    building_color, // Tan/beige building
                )));
            }

            // Block 2 - left side, middle row
            if block_id == 2 {
                block.add_object(Box::new(Building::new(
                    0.25,           // x_offset: 25% from left
                    0.20,           // y_offset: 20% from top
                    0.45,           // width: 45% of block width
                    35.0,           // height: 35 pixels tall
                    0.50,           // depth: 50% of block height
                    6.0,            // corner_radius: 6 pixels
                    building_color, // Reddish building
                )));
            }

            // Block 6 - Connected buildings: Large office tower with smaller annex
            if block_id == 6 {
                // Main large building (office tower)
                block.add_object(Box::new(Building::new(
                    0.30,  // x_offset: 10% from left
                    0.50,  // y_offset: 15% from top
                    0.20,  // width: 35% of block width
                    200.0, // height: 65 pixels tall (tall tower)
                    0.30,  // depth: 50% of block height
                    6.0,   // corner_radius: 6 pixels
                    building_color,
                )));

                // Smaller connected building (annex/wing)
                block.add_object(Box::new(Building::new(
                    0.50, // x_offset: 45% from left (connected to the right side)
                    0.50, // y_offset: 25% from top (slightly lower)
                    0.35, // width: 35% of block width
                    35.0, // height: 35 pixels tall (much shorter)
                    0.45, // depth: 45% of block height
                    6.0,  // corner_radius: 6 pixels
                    building_color,
                )));
            }

            // Block 10 - L-shaped building complex
            if block_id == 10 {
                // Perpendicular wing (vertical part of L) - drawn first (further back)
                block.add_object(Box::new(Building::new(
                    0.20, // x_offset: 20% from left (overlaps with main)
                    0.15, // y_offset: 15% from top (extends upward, further back)
                    0.25, // width: 25% of block width (narrow)
                    50.0, // height: 50 pixels tall (taller than main)
                    0.45, // depth: 45% of block height (deep)
                    7.0,  // corner_radius: 7 pixels
                    building_color,
                )));

                // Main building (horizontal part of L) - drawn second (closer to viewer)
                block.add_object(Box::new(Building::new(
                    0.15, // x_offset: 15% from left
                    0.30, // y_offset: 30% from top (lower, closer to viewer)
                    0.60, // width: 60% of block width (wide)
                    45.0, // height: 45 pixels tall
                    0.25, // depth: 25% of block height (shallow)
                    7.0,  // corner_radius: 7 pixels
                    building_color,
                )));
            }

            // Block 12 - Modern complex: Two towers with connecting bridge effect
            if block_id == 12 {
                // Right tower (slightly taller) - drawn first (furthest back)
                block.add_object(Box::new(Building::new(
                    0.55, // x_offset: 55% from left
                    0.15, // y_offset: 15% from top (furthest back)
                    0.30, // width: 30% of block width
                    60.0, // height: 60 pixels tall
                    0.50, // depth: 50% of block height
                    5.0,  // corner_radius: 5 pixels
                    building_color,
                )));

                // Left tower - drawn second (middle depth)
                block.add_object(Box::new(Building::new(
                    0.10, // x_offset: 10% from left
                    0.20, // y_offset: 20% from top (middle depth)
                    0.25, // width: 25% of block width
                    55.0, // height: 55 pixels tall
                    0.45, // depth: 45% of block height
                    5.0,  // corner_radius: 5 pixels
                    building_color,
                )));

                // Connecting bridge/walkway - drawn last (closest to viewer)
                block.add_object(Box::new(Building::new(
                    0.35, // x_offset: 35% from left (between towers)
                    0.35, // y_offset: 35% from top (closest to viewer)
                    0.20, // width: 20% of block width (narrow)
                    25.0, // height: 25 pixels tall (low bridge)
                    0.30, // depth: 30% of block height
                    3.0,  // corner_radius: 3 pixels
                    building_color,
                )));
            }

            // Block 5 - center of grid
            if block_id == 5 {
                block.add_object(Box::new(Building::new(
                    0.15, // x_offset: 15% from left
                    0.25, // y_offset: 25% from top
                    0.60, // width: 60% of block width
                    60.0, // height: 60 pixels tall (tallest)
                    0.45, // depth: 45% of block height
                    10.0, // corner_radius: 10 pixels
                    building_color,
                )));
            }

            // Block 8 is second row, third column - add a building in the middle
            if block_id == 8 {
                // Add a fence on the top side with offsets from edges
                block.add_object(Box::new(Fence::new(
                    0.10,                           // x_offset: 10% from left edge
                    0.10,                           // y_offset: 10% from top edge
                    0.80, // width: 80% of block width (leaves 10% at left, 10% at right)
                    0.01, // depth: 1% of block height
                    6.0,  // height: 6 pixels tall
                    Color::new(0.4, 0.3, 0.2, 1.0), // Brown fence
                )));

                // Add a fence on the left side with offsets from edges
                block.add_object(Box::new(Fence::new(
                    0.10,                           // x_offset: 10% from left edge
                    0.11, // y_offset: 11% from top edge (starts where top fence ends)
                    0.01, // width: 1% of block width
                    0.78, // depth: 78% (from 11% to 89%)
                    6.0,  // height: 6 pixels tall
                    Color::new(0.4, 0.3, 0.2, 1.0), // Brown fence
                )));

                // Add a fence on the right side with offsets from edges
                block.add_object(Box::new(Fence::new(
                    0.89, // x_offset: 89% from left edge (leaves 10% + 1% width to reach right edge)
                    0.11, // y_offset: 11% from top edge (starts where top fence ends)
                    0.01, // width: 1% of block width
                    0.78, // depth: 78% (from 11% to 89%)
                    6.0,  // height: 6 pixels tall
                    Color::new(0.4, 0.3, 0.2, 1.0), // Brown fence
                )));

                // Add a fence on the bottom side with offsets from edges
                block.add_object(Box::new(Fence::new(
                    0.10,                           // x_offset: 10% from left edge
                    0.89, // y_offset: 89% from top edge (leaves 10% + 1% height to reach bottom edge)
                    0.80, // width: 80% of block width (leaves 10% at left, 10% at right)
                    0.01, // depth: 1% of block height
                    6.0,  // height: 6 pixels tall
                    Color::new(0.4, 0.3, 0.2, 1.0), // Brown fence
                )));

                // Add building in the center of the block
                // Positioned at 25% offset, sized to 50% of block dimensions
                block.add_object(Box::new(Building::new(
                    0.25, // x_offset: 25% from left
                    0.25, // y_offset: 25% from top
                    0.4,  // width: 40% of block width
                    40.0, // height: 40 pixels tall
                    0.3,  // depth: 30% of block height
                    8.0,  // corner_radius: 8 pixels
                    building_color,
                )));
            }

            // Block 7 - top row, third column
            if block_id == 7 {
                block.add_object(Box::new(Building::new(
                    0.30, // x_offset: 30% from left
                    0.35, // y_offset: 35% from top
                    0.35, // width: 35% of block width
                    45.0, // height: 45 pixels tall
                    0.35, // depth: 35% of block height
                    7.0,  // corner_radius: 7 pixels
                    building_color,
                )));
            }

            // Block 9 - bottom row, third column
            if block_id == 9 {
                block.add_object(Box::new(Building::new(
                    0.20, // x_offset: 20% from left
                    0.25, // y_offset: 25% from top
                    0.55, // width: 55% of block width
                    40.0, // height: 40 pixels tall
                    0.50, // depth: 50% of block height
                    9.0,  // corner_radius: 9 pixels
                    building_color,
                )));
            }

            // Block 11 - middle row, far right
            if block_id == 11 {
                block.add_object(Box::new(Building::new(
                    0.25, // x_offset: 25% from left
                    0.30, // y_offset: 30% from top
                    0.40, // width: 40% of block width
                    55.0, // height: 55 pixels tall
                    0.40, // depth: 40% of block height
                    8.0,  // corner_radius: 8 pixels
                    building_color,
                )));
            }

            blocks.push(block);
            block_id += 1;
        }
    }

    blocks
}
