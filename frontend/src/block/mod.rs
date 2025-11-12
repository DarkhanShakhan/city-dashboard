//! Block structure and rendering
//!
//! This module defines city blocks - the areas between roads.
//! Blocks are containers that can hold multiple objects (grass, buildings, parking, etc.).
//!
//! Architecture:
//! - Block: A positioned container that holds BlockObjects
//! - BlockObject: Trait for things that can be rendered (Grass, Building, etc.)
//! - Grass, Building, etc.: Concrete implementations of BlockObject

mod building;
mod generation;
mod grass;

pub use building::{Building, BuildingBuilder, BUILDING_CORNER_RADIUS};
pub use generation::generate_grass_blocks;
pub use grass::{Grass, GrassBuilder};

use crate::models::Direction;
use macroquad::prelude::*;
use std::collections::HashMap;

// ============================================================================
// Render Context
// ============================================================================

/// Context passed to block objects during rendering
///
/// Contains global state that objects may need to render differently
#[derive(Clone, Debug)]
pub struct RenderContext {
    /// Current simulation time
    pub time: f64,

    /// Danger mode active (emergency warning state)
    pub danger_mode: bool,
}

impl RenderContext {
    pub fn new(time: f64, danger_mode: bool) -> Self {
        Self { time, danger_mode }
    }
}

// ============================================================================
// Block Object Trait
// ============================================================================

/// Trait for objects that can be placed in blocks and rendered
///
/// Implement this trait for any object that should be drawable in the city
/// (buildings, parking lots, parks, grass areas, etc.)
pub trait BlockObject {
    /// Renders the object to the screen
    ///
    /// # Arguments
    /// * `block` - Reference to the block this object is being rendered in
    /// * `context` - Rendering context with global state
    fn render(&self, block: &Block, context: &RenderContext);
}

// ============================================================================
// Block Model (Container)
// ============================================================================

/// Represents a city block in the grid between roads
///
/// Blocks are rectangular container areas that can hold multiple objects.
/// Position and size are stored as percentages (0.0-1.0) to support
/// dynamic window resizing.
pub struct Block {
    /// Horizontal position as percentage of screen width (0.0 = left, 1.0 = right)
    pub x_percent: f32,

    /// Vertical position as percentage of screen height (0.0 = top, 1.0 = bottom)
    pub y_percent: f32,

    /// Width as percentage of screen width
    pub width_percent: f32,

    /// Height as percentage of screen height
    pub height_percent: f32,

    /// Unique identifier for this block
    pub id: usize,

    /// Objects contained in this block
    pub objects: Vec<Box<dyn BlockObject>>,

    /// Roads adjacent to this block (direction -> road_id)
    pub adjacent_roads: HashMap<Direction, usize>,
}

impl Block {
    /// Creates a new BlockBuilder for constructing blocks with the builder pattern
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the block
    ///
    /// # Returns
    /// A BlockBuilder instance
    ///
    /// # Example
    /// ```
    /// let block = Block::builder(0)
    ///     .position(0.1, 0.2)
    ///     .size(0.15, 0.2)
    ///     .build();
    /// ```
    pub fn builder(id: usize) -> BlockBuilder {
        BlockBuilder::new(id)
    }

    /// Creates a new block directly
    ///
    /// Consider using `Block::builder(id)` for more flexible construction.
    ///
    /// # Arguments
    /// * `x_percent` - X position as percentage (0.0-1.0)
    /// * `y_percent` - Y position as percentage (0.0-1.0)
    /// * `width_percent` - Width as percentage (0.0-1.0)
    /// * `height_percent` - Height as percentage (0.0-1.0)
    /// * `id` - Unique identifier
    ///
    /// # Returns
    /// A new Block instance with no objects
    pub fn new(
        x_percent: f32,
        y_percent: f32,
        width_percent: f32,
        height_percent: f32,
        id: usize,
    ) -> Self {
        Self {
            x_percent,
            y_percent,
            width_percent,
            height_percent,
            id,
            objects: Vec::new(),
            adjacent_roads: HashMap::new(),
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

    /// Converts the percentage-based width to absolute pixels
    ///
    /// # Returns
    /// Absolute width in pixels
    pub fn width(&self) -> f32 {
        self.width_percent * screen_width()
    }

    /// Converts the percentage-based height to absolute pixels
    ///
    /// # Returns
    /// Absolute height in pixels
    pub fn height(&self) -> f32 {
        self.height_percent * screen_height()
    }

    /// Renders all objects contained in this block
    ///
    /// # Arguments
    /// * `context` - Rendering context with global state (time, danger_mode, etc.)
    pub fn render(&self, context: &RenderContext) {
        for obj in &self.objects {
            obj.render(self, context);
        }
    }

    /// Adds an object to this block
    ///
    /// # Arguments
    /// * `obj` - The object to add (must implement BlockObject)
    pub fn add_object(&mut self, obj: Box<dyn BlockObject>) {
        self.objects.push(obj);
    }

    /// Removes all objects from this block
    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    /// Returns the number of objects in this block
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Checks if a point (in pixels) is inside this block
    ///
    /// Useful for click detection and interaction.
    ///
    /// # Arguments
    /// * `px` - X coordinate in pixels
    /// * `py` - Y coordinate in pixels
    ///
    /// # Returns
    /// `true` if the point is inside the block
    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        let x = self.x();
        let y = self.y();
        let width = self.width();
        let height = self.height();

        px >= x && px <= x + width && py >= y && py <= y + height
    }

    /// Gets the center position of the block in pixels
    ///
    /// # Returns
    /// Tuple of (center_x, center_y) in pixels
    pub fn center(&self) -> (f32, f32) {
        let x = self.x();
        let y = self.y();
        let width = self.width();
        let height = self.height();

        (x + width / 2.0, y + height / 2.0)
    }

    /// Connects a road to this block in a specific direction
    ///
    /// # Arguments
    /// * `direction` - Which side of the block the road is on
    /// * `road_id` - ID of the road to connect
    pub fn connect_road(&mut self, direction: Direction, road_id: usize) {
        self.adjacent_roads.insert(direction, road_id);
    }

    /// Gets the road ID adjacent to this block in a specific direction
    ///
    /// # Arguments
    /// * `direction` - Direction to look
    ///
    /// # Returns
    /// Optional road ID if a road exists in that direction
    pub fn get_adjacent_road(&self, direction: Direction) -> Option<usize> {
        self.adjacent_roads.get(&direction).copied()
    }
}

// ============================================================================
// Block Builder
// ============================================================================

/// Builder for creating Block instances with a fluent API
///
/// Provides a convenient way to construct blocks with optional parameters.
///
/// # Example
/// ```
/// let block = Block::builder(0)
///     .position(0.1, 0.2)
///     .size(0.15, 0.2)
///     .build();
/// ```
pub struct BlockBuilder {
    id: usize,
    x_percent: Option<f32>,
    y_percent: Option<f32>,
    width_percent: Option<f32>,
    height_percent: Option<f32>,
}

impl BlockBuilder {
    /// Creates a new BlockBuilder
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the block
    fn new(id: usize) -> Self {
        Self {
            id,
            x_percent: None,
            y_percent: None,
            width_percent: None,
            height_percent: None,
        }
    }

    /// Sets the position of the block
    ///
    /// # Arguments
    /// * `x_percent` - X position as percentage (0.0-1.0)
    /// * `y_percent` - Y position as percentage (0.0-1.0)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn position(mut self, x_percent: f32, y_percent: f32) -> Self {
        self.x_percent = Some(x_percent);
        self.y_percent = Some(y_percent);
        self
    }

    /// Sets the x position of the block
    ///
    /// # Arguments
    /// * `x_percent` - X position as percentage (0.0-1.0)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn x(mut self, x_percent: f32) -> Self {
        self.x_percent = Some(x_percent);
        self
    }

    /// Sets the y position of the block
    ///
    /// # Arguments
    /// * `y_percent` - Y position as percentage (0.0-1.0)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn y(mut self, y_percent: f32) -> Self {
        self.y_percent = Some(y_percent);
        self
    }

    /// Sets the size of the block
    ///
    /// # Arguments
    /// * `width_percent` - Width as percentage (0.0-1.0)
    /// * `height_percent` - Height as percentage (0.0-1.0)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn size(mut self, width_percent: f32, height_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self.height_percent = Some(height_percent);
        self
    }

    /// Sets the width of the block
    ///
    /// # Arguments
    /// * `width_percent` - Width as percentage (0.0-1.0)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn width(mut self, width_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self
    }

    /// Sets the height of the block
    ///
    /// # Arguments
    /// * `height_percent` - Height as percentage (0.0-1.0)
    ///
    /// # Returns
    /// Self for method chaining
    pub fn height(mut self, height_percent: f32) -> Self {
        self.height_percent = Some(height_percent);
        self
    }

    /// Builds the Block instance
    ///
    /// Uses default values if parameters were not explicitly set:
    /// - x_percent: 0.0 (left edge)
    /// - y_percent: 0.0 (top edge)
    /// - width_percent: 0.1 (10% of screen width)
    /// - height_percent: 0.1 (10% of screen height)
    ///
    /// # Returns
    /// A new Block instance with no objects
    pub fn build(self) -> Block {
        Block {
            x_percent: self.x_percent.unwrap_or(0.0),
            y_percent: self.y_percent.unwrap_or(0.0),
            width_percent: self.width_percent.unwrap_or(0.1),
            height_percent: self.height_percent.unwrap_or(0.1),
            id: self.id,
            objects: Vec::new(),
            adjacent_roads: HashMap::new(),
        }
    }
}
