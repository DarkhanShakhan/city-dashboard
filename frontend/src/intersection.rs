//! Intersection structure and generation
//!
//! This module defines:
//! - Intersection struct: Road crossings with traffic lights
//! - City road network topology (3x2 grid)
//! - Intersection generation logic

use crate::constants::road_network::{HORIZONTAL_ROAD_POSITIONS, VERTICAL_ROAD_POSITIONS};
use crate::constants::rendering::INTERSECTION_SIZE;
use crate::models::Direction;
use crate::traffic_light::IntersectionTrafficLight;
use macroquad::prelude::*;
use std::collections::HashMap;

// ============================================================================
// Intersection Model
// ============================================================================

/// Represents a road intersection with traffic lights
///
/// Intersections are positioned at grid points where roads cross.
/// Each intersection manages its own traffic light controller and connections to roads.
#[derive(Clone)]
pub struct Intersection {
    /// Horizontal position as percentage of screen width
    pub x_percent: f32,

    /// Vertical position as percentage of screen height
    pub y_percent: f32,

    /// Unique identifier for this intersection
    pub id: usize,

    /// Unified traffic light controller for this intersection
    pub light: Option<IntersectionTrafficLight>,

    /// Roads connected to this intersection (direction -> road_id)
    pub connected_roads: HashMap<Direction, usize>,
}

impl Intersection {
    /// Creates a new intersection
    ///
    /// # Arguments
    /// * `x_percent` - X position as percentage (0.0-1.0)
    /// * `y_percent` - Y position as percentage (0.0-1.0)
    /// * `id` - Unique identifier
    pub fn new(x_percent: f32, y_percent: f32, id: usize) -> Self {
        Self {
            x_percent,
            y_percent,
            id,
            light: None,
            connected_roads: HashMap::new(),
        }
    }

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

    /// Sets the traffic light controller for this intersection
    ///
    /// # Arguments
    /// * `light` - The intersection traffic light controller
    pub fn set_light(&mut self, light: IntersectionTrafficLight) {
        self.light = Some(light);
    }

    /// Updates the traffic light at this intersection
    ///
    /// # Arguments
    /// * `dt` - Delta time in seconds
    pub fn update_lights(&mut self, dt: f32) {
        if let Some(light) = &mut self.light {
            light.update(dt);
        }
    }

    /// Renders the traffic lights at this intersection
    ///
    /// Traffic lights are positioned relative to the intersection center:
    /// - Vertical lights (up/down): top-right corner
    /// - Horizontal lights (left/right): bottom-left corner
    ///
    /// # Arguments
    /// * `force_red` - If true, forces all lights to show red (emergency mode)
    pub fn render_lights(&self, force_red: bool) {
        if let Some(light) = &self.light {
            light.render(force_red);
        }
    }

    /// Checks if this intersection has a traffic light
    pub fn has_light(&self) -> bool {
        self.light.is_some()
    }

    /// Clears the traffic light from this intersection
    pub fn clear_light(&mut self) {
        self.light = None;
    }

    /// Checks if a point (in pixels) is inside this intersection
    ///
    /// Used for detecting when cars enter/exit intersections.
    ///
    /// # Arguments
    /// * `px` - X coordinate in pixels
    /// * `py` - Y coordinate in pixels
    ///
    /// # Returns
    /// `true` if the point is inside the intersection bounds
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        let int_x = self.x();
        let int_y = self.y();

        (px - int_x).abs() <= INTERSECTION_SIZE && (py - int_y).abs() <= INTERSECTION_SIZE
    }

    /// Connects a road to this intersection in a specific direction
    ///
    /// # Arguments
    /// * `direction` - Direction from intersection to road
    /// * `road_id` - ID of the road to connect
    pub fn connect_road(&mut self, direction: Direction, road_id: usize) {
        self.connected_roads.insert(direction, road_id);
    }

    /// Gets the road ID in a specific direction from this intersection
    ///
    /// # Arguments
    /// * `direction` - Direction to look
    ///
    /// # Returns
    /// Optional road ID if a road exists in that direction
    pub fn get_road_in_direction(&self, direction: Direction) -> Option<usize> {
        self.connected_roads.get(&direction).copied()
    }

    /// Gets the traffic light state for a given direction
    ///
    /// # Arguments
    /// * `direction` - Direction of travel (Down/Up for vertical, Left/Right for horizontal)
    ///
    /// # Returns
    /// Traffic light state: 0 = red, 1 = yellow, 2 = green
    pub fn get_light_state_for_direction(&self, direction: Direction) -> u8 {
        if let Some(light) = &self.light {
            light.get_state_for_direction(direction)
        } else {
            // Default to red if no light found
            0
        }
    }
}

// ============================================================================
// Road Grid Configuration
// ============================================================================

/// Returns the absolute pixel positions of all roads
///
/// # Returns
/// A tuple of (vertical_positions, horizontal_positions) in pixels
///
/// # Note
/// This function is currently unused but kept for potential future use
pub fn get_road_positions() -> (Vec<f32>, Vec<f32>) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Three vertical roads at 15%, 50%, and 85% of screen width
    let vertical_positions = vec![screen_width * 0.15, screen_width * 0.5, screen_width * 0.85];

    // Two horizontal roads at 25% and 75% of screen height
    let horizontal_positions = vec![screen_height * 0.25, screen_height * 0.75];

    (vertical_positions, horizontal_positions)
}

// ============================================================================
// Intersection Generation
// ============================================================================

/// Generates all intersections for the city grid
///
/// Creates a 3×2 grid of intersections where vertical and horizontal roads cross.
/// Each intersection gets:
/// - Unique ID (0-5)
/// - Position as percentages (for dynamic resizing)
/// - Staggered time offset for traffic light synchronization
///
/// # Returns
/// Vector of 6 intersections
///
/// # Traffic Light Staggering
/// Each intersection has a 1-second time offset from the previous one,
/// preventing all lights from turning green simultaneously and creating
/// more realistic traffic flow patterns.
pub fn generate_intersections() -> Vec<Intersection> {
    // Store positions as percentages (0.0 to 1.0) for dynamic resizing
    let vertical_percents = VERTICAL_ROAD_POSITIONS.to_vec();
    let horizontal_percents = HORIZONTAL_ROAD_POSITIONS.to_vec();

    let mut intersections = Vec::new();
    let mut id = 0;

    // Create intersection at each grid point with unified traffic light
    for &x_percent in &vertical_percents {
        for &y_percent in &horizontal_percents {
            let mut intersection = Intersection::new(x_percent, y_percent, id);

            // Create unified traffic light controller
            // Start with vertical green for even IDs, horizontal green for odd IDs (creates staggering)
            let light = IntersectionTrafficLight::new(
                x_percent,
                y_percent,
                id,
                id % 2 == 0, // vertical_starts_green
            );

            intersection.set_light(light);

            intersections.push(intersection);
            id += 1;
        }
    }

    intersections
}
