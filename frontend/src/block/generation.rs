//! Block generation functions
//!
//! Provides functions for generating the city grid of blocks.

use crate::block::{Block, Building, Grass};
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

            // Block 8 is second row, third column - add a building in the middle
            if block_id == 8 {
                // Add building in the center of the block
                // Positioned at 25% offset, sized to 50% of block dimensions
                block.add_object(Box::new(Building::new(
                    0.25, // x_offset: 25% from left
                    0.25, // y_offset: 25% from top
                    0.4,  // width: 40% of block width
                    40.0, // height: 40 pixels tall
                    0.3,  // depth: 30% of block height
                    8.0,  // corner_radius: 8 pixels
                    Color::new(0.5, 0.6, 0.7, 1.0), // Blue-gray building
                )));
            }

            blocks.push(block);
            block_id += 1;
        }
    }

    blocks
}
