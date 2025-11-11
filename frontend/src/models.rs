//! Core data models for the city traffic simulation
//!
//! This module defines the fundamental structures used throughout the application:
//! - Car: Represents vehicles moving through the city
//! - Intersection: Represents road crossings with traffic lights
//! - Direction: Cardinal directions for vehicle movement

use macroquad::prelude::*;

// ============================================================================
// Car Model
// ============================================================================

/// Represents a vehicle in the traffic simulation
///
/// Cars store their position as percentages (0.0-1.0) of screen dimensions
/// to support dynamic window resizing without position corruption.
#[derive(Clone)]
pub struct Car {
    /// Horizontal position as percentage of screen width (0.0 = left, 1.0 = right)
    pub x_percent: f32,

    /// Vertical position as percentage of screen height (0.0 = top, 1.0 = bottom)
    pub y_percent: f32,

    /// Current direction of travel (Down, Right, Up, or Left)
    pub direction: Direction,

    /// Visual color of the car body
    pub color: Color,

    /// Index of the road this car is currently on
    pub road_index: usize,

    /// Planned direction for the next intersection (None = go straight)
    pub next_turn: Option<Direction>,

    /// Flag to prevent multiple turns at the same intersection
    pub just_turned: bool,

    /// True when the car is currently inside an intersection
    /// (prevents stopping mid-intersection)
    pub in_intersection: bool,

    /// Logical location metadata (which road/intersection/block the car is in)
    pub location: CarLocation,
}

impl Car {
    /// Converts the percentage-based x position to absolute pixel coordinates
    ///
    /// # Returns
    /// Absolute x position in pixels
    pub fn x(&self) -> f32 {
        self.x_percent * screen_width()
    }

    /// Converts the percentage-based y position to absolute pixel coordinates
    ///
    /// # Returns
    /// Absolute y position in pixels
    pub fn y(&self) -> f32 {
        self.y_percent * screen_height()
    }

    /// Sets the car's x position using absolute pixel coordinates
    ///
    /// # Arguments
    /// * `x` - Absolute x position in pixels
    pub fn set_x(&mut self, x: f32) {
        self.x_percent = x / screen_width();
    }

    /// Sets the car's y position using absolute pixel coordinates
    ///
    /// # Arguments
    /// * `y` - Absolute y position in pixels
    pub fn set_y(&mut self, y: f32) {
        self.y_percent = y / screen_height();
    }
}

// ============================================================================
// Direction Enum
// ============================================================================

/// Cardinal directions for vehicle movement
///
/// Used to determine car orientation, turning logic, and collision detection.
/// Implements Copy for efficient passing, PartialEq for direction comparisons,
/// Hash and Eq for use as HashMap keys.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Moving downward (increasing y)
    Down,

    /// Moving right (increasing x)
    Right,

    /// Moving upward (decreasing y)
    Up,

    /// Moving left (decreasing x)
    Left,
}

impl Direction {
    /// Converts direction to a unit vector (dx, dy)
    ///
    /// # Returns
    /// Tuple of (dx, dy) representing direction as vector
    pub fn to_vector(&self) -> (f32, f32) {
        match self {
            Direction::Down => (0.0, 1.0),
            Direction::Right => (1.0, 0.0),
            Direction::Up => (0.0, -1.0),
            Direction::Left => (-1.0, 0.0),
        }
    }
}

// ============================================================================
// Car Location Enum
// ============================================================================

/// Represents the logical location of a car in the city
///
/// This is metadata about which city element the car is currently in.
/// The actual visual position is always stored in Car's x_percent/y_percent.
#[derive(Clone, Debug)]
pub enum CarLocation {
    /// Car is traveling on a road
    OnRoad { road_id: usize },

    /// Car is inside an intersection
    InIntersection { intersection_id: usize },

    /// Car is inside a block (e.g., parking lot)
    InBlock { block_id: usize },
}

