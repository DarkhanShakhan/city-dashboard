//! Road structure and lane management
//!
//! This module defines the Road structure and related functionality for:
//! - Road positioning and orientation
//! - Lane calculations for left-hand traffic
//! - Car spawn position calculations

use crate::constants::vehicle::LANE_OFFSET;
use crate::models::Direction;
use macroquad::prelude::*;

// ============================================================================
// Road Orientation
// ============================================================================

/// Orientation of a road (vertical or horizontal)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Orientation {
    /// Road runs vertically (cars move up/down)
    Vertical,

    /// Road runs horizontally (cars move left/right)
    Horizontal,
}

// ============================================================================
// Road Model
// ============================================================================

/// Represents a road in the city grid
///
/// Roads are the pathways where cars travel. Each road has a fixed position
/// and orientation. The simulation uses left-hand traffic rules.
#[derive(Clone)]
pub struct Road {
    /// Position as percentage of screen dimension (0.0-1.0)
    /// For vertical roads: percentage of screen width
    /// For horizontal roads: percentage of screen height
    pub position_percent: f32,

    /// Whether this road runs vertically or horizontally
    pub orientation: Orientation,

    /// Unique identifier for this road
    pub index: usize,

    /// Intersection at the start of the road (None if road extends off-screen)
    pub start_intersection_id: Option<usize>,

    /// Intersection at the end of the road (None if road extends off-screen)
    pub end_intersection_id: Option<usize>,

    /// IDs of blocks adjacent to this road
    pub adjacent_block_ids: Vec<usize>,
}

impl Road {
    /// Creates a new road
    ///
    /// # Arguments
    /// * `position_percent` - Position as percentage (0.0-1.0)
    /// * `orientation` - Vertical or Horizontal
    /// * `index` - Unique identifier
    ///
    /// # Returns
    /// A new Road instance
    pub fn new(position_percent: f32, orientation: Orientation, index: usize) -> Self {
        Self {
            position_percent,
            orientation,
            index,
            start_intersection_id: None,
            end_intersection_id: None,
            adjacent_block_ids: Vec::new(),
        }
    }

    /// Calculates the lane position for a car based on its direction
    ///
    /// Uses left-hand traffic rules:
    /// - Vertical roads: down = left lane, up = right lane
    /// - Horizontal roads: right = bottom lane, left = top lane
    ///
    /// # Arguments
    /// * `going_positive` - True if moving in positive direction (down/right), false otherwise (up/left)
    ///
    /// # Returns
    /// Position percentage for the correct lane
    pub fn get_lane_position(&self, going_positive: bool) -> f32 {
        match self.orientation {
            Orientation::Vertical => {
                // going_positive = going down
                let offset_percent = LANE_OFFSET / screen_width();
                if going_positive {
                    self.position_percent - offset_percent // Left lane
                } else {
                    self.position_percent + offset_percent // Right lane
                }
            }
            Orientation::Horizontal => {
                // going_positive = going right
                let offset_percent = LANE_OFFSET / screen_height();
                if going_positive {
                    self.position_percent + offset_percent // Bottom lane
                } else {
                    self.position_percent - offset_percent // Top lane
                }
            }
        }
    }

    /// Calculates spawn position for a car off-screen
    ///
    /// Cars spawn just outside the visible screen area (at -0.05 or 1.05)
    /// in the appropriate lane based on their direction.
    ///
    /// # Arguments
    /// * `going_positive` - True if moving in positive direction (down/right), false otherwise (up/left)
    ///
    /// # Returns
    /// Tuple of (x_percent, y_percent) for spawning the car
    pub fn get_spawn_position(&self, going_positive: bool) -> (f32, f32) {
        match self.orientation {
            Orientation::Vertical => {
                // X position is the lane
                let x = if going_positive {
                    self.position_percent - (LANE_OFFSET / screen_width()) // Left lane (going down)
                } else {
                    self.position_percent + (LANE_OFFSET / screen_width()) // Right lane (going up)
                };

                // Y position is off-screen
                let y = if going_positive {
                    -0.05 // Top of screen (going down)
                } else {
                    1.05 // Bottom of screen (going up)
                };

                (x, y)
            }
            Orientation::Horizontal => {
                // X position is off-screen
                let x = if going_positive {
                    -0.05 // Left of screen (going right)
                } else {
                    1.05 // Right of screen (going left)
                };

                // Y position is the lane
                let y = if going_positive {
                    self.position_percent + (LANE_OFFSET / screen_height()) // Bottom lane (going right)
                } else {
                    self.position_percent - (LANE_OFFSET / screen_height()) // Top lane (going left)
                };

                (x, y)
            }
        }
    }

    /// Returns the direction a car would move in the positive direction on this road
    ///
    /// # Returns
    /// Direction::Down for vertical roads, Direction::Right for horizontal roads
    pub fn get_positive_direction(&self) -> Direction {
        match self.orientation {
            Orientation::Vertical => Direction::Down,
            Orientation::Horizontal => Direction::Right,
        }
    }

    /// Returns the direction a car would move in the negative direction on this road
    ///
    /// # Returns
    /// Direction::Up for vertical roads, Direction::Left for horizontal roads
    pub fn get_negative_direction(&self) -> Direction {
        match self.orientation {
            Orientation::Vertical => Direction::Up,
            Orientation::Horizontal => Direction::Left,
        }
    }
}
