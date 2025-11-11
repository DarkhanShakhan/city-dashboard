//! Car spawning system
//!
//! This module handles car spawning logic:
//! - CarSpawner: Manages spawning at regular intervals
//! - spawn_car: Creates new cars at random positions with random properties
//!
//! Cars are spawned off-screen at road edges and follow left-hand traffic rules.

use crate::constants::{
    road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS},
    vehicle::{LANE_OFFSET, TURN_PROBABILITY},
};
use crate::models::{Car, CarLocation, Direction};
use macroquad::prelude::*;

// ============================================================================
// CarSpawner - Interval-based spawning
// ============================================================================

/// Manages car spawning at regular intervals
///
/// This struct tracks the last spawn time and ensures cars are spawned
/// at consistent intervals rather than every frame.
pub struct CarSpawner {
    last_spawn_time: f64,
    spawn_interval: f32,
}

impl CarSpawner {
    /// Creates a new CarSpawner with a specified spawn interval
    ///
    /// # Arguments
    /// * `interval` - Time between spawns in seconds
    ///
    /// # Example
    /// ```
    /// let spawner = CarSpawner::new(1.5); // Spawn every 1.5 seconds
    /// ```
    pub fn new(interval: f32) -> Self {
        Self {
            last_spawn_time: 0.0,
            spawn_interval: interval,
        }
    }

    /// Attempts to spawn a car if enough time has elapsed
    ///
    /// Checks if the spawn interval has passed since the last spawn.
    /// If so, spawns a new car and updates the last spawn time.
    ///
    /// # Arguments
    /// * `cars` - Mutable vector to add the new car to
    pub fn try_spawn(&mut self, cars: &mut Vec<Car>) {
        let current_time = get_time();

        if current_time - self.last_spawn_time > self.spawn_interval as f64 {
            spawn_car(cars);
            self.last_spawn_time = current_time;
        }
    }
}

// ============================================================================
// Car Spawning Function
// ============================================================================

/// Spawns a new car at a random road edge
///
/// Cars are spawned just off-screen and assigned:
/// - Random road (3 vertical, 2 horizontal)
/// - Random direction (with proper lane selection)
/// - Random color
/// - Random chance of planning a turn at next intersection
///
/// # Arguments
/// * `cars` - Mutable vector to add the new car to
///
/// # Lane Discipline (Left-hand traffic)
/// - Vertical roads: Cars going down use left lane, cars going up use right lane
/// - Horizontal roads: Cars going right use bottom lane, cars going left use top lane
pub fn spawn_car(cars: &mut Vec<Car>) {
    // Road positions as percentages of screen dimensions
    let vertical_percents = VERTICAL_ROAD_POSITIONS;
    let horizontal_percents = HORIZONTAL_ROAD_POSITIONS;

    // Randomly choose vertical or horizontal road
    let is_vertical = rand::gen_range(0, 2) == 0;

    // Random car color selection
    let car_colors = [BLUE, RED, YELLOW, Color::new(1.0, 0.5, 0.0, 1.0), PURPLE];
    let color = car_colors[rand::gen_range(0, car_colors.len())];

    if is_vertical {
        // Spawn on vertical road (moving down or up)
        let road_index = rand::gen_range(0, vertical_percents.len());
        let road_center_percent = vertical_percents[road_index];
        let going_down = rand::gen_range(0, 2) == 0;

        // Cars going down use left lane (offset to the left)
        // Cars going up use right lane (offset to the right)
        let lane_offset_percent = LANE_OFFSET / screen_width(); // Offset in x direction
        let x_percent = if going_down {
            road_center_percent - lane_offset_percent
        } else {
            road_center_percent + lane_offset_percent
        };

        // Randomly decide if car will turn
        let next_turn = if rand::gen_range(0.0, 1.0) < TURN_PROBABILITY {
            // Choose a perpendicular direction for turning
            if rand::gen_range(0, 2) == 0 {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else {
            None // Go straight
        };

        cars.push(Car {
            x_percent,
            y_percent: if going_down { -0.05 } else { 1.05 }, // Spawn just off screen
            direction: if going_down {
                Direction::Down
            } else {
                Direction::Up
            },
            color,
            road_index,
            next_turn,
            just_turned: false,
            in_intersection: false,
            location: CarLocation::OnRoad {
                road_id: road_index,
            },
        });
    } else {
        // Spawn on horizontal road (moving right or left)
        let road_index = rand::gen_range(0, horizontal_percents.len());
        let road_center_percent = horizontal_percents[road_index];
        let going_right = rand::gen_range(0, 2) == 0;

        // Cars going right use bottom lane (offset down)
        // Cars going left use top lane (offset up)
        let lane_offset_percent = LANE_OFFSET / screen_height(); // Offset in y direction
        let y_percent = if going_right {
            road_center_percent + lane_offset_percent
        } else {
            road_center_percent - lane_offset_percent
        };

        // Randomly decide if car will turn
        let next_turn = if rand::gen_range(0.0, 1.0) < TURN_PROBABILITY {
            // Choose a perpendicular direction for turning
            if rand::gen_range(0, 2) == 0 {
                Some(Direction::Down)
            } else {
                Some(Direction::Up)
            }
        } else {
            None // Go straight
        };

        cars.push(Car {
            x_percent: if going_right { -0.05 } else { 1.05 }, // Spawn just off screen
            y_percent,
            direction: if going_right {
                Direction::Right
            } else {
                Direction::Left
            },
            color,
            road_index: road_index + 3, // Offset by 3 since vertical roads are 0-2
            next_turn,
            just_turned: false,
            in_intersection: false,
            location: CarLocation::OnRoad {
                road_id: road_index + 3,
            },
        });
    }
}
