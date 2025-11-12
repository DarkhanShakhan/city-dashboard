//! Grass block object implementation
//!
//! Provides a simple grass area that can fill or partially fill city blocks.

use crate::block::{Block, BlockObject, RenderContext};
use crate::constants::visual::{BLOCK_CORNER_RADIUS, DEPTH_OFFSET, GRASS_COLOR, GRASS_DEPTH_COLOR};
use crate::rendering::draw_rounded_rectangle;
use macroquad::prelude::*;

// ============================================================================
// Grass Object Implementation
// ============================================================================

/// A grass area object that can be placed in blocks
///
/// Renders a grass area with 2.5D depth effect.
/// Position and size are relative to the containing block.
pub struct Grass {
    /// Horizontal offset as percentage of block width (0.0 = left edge, 1.0 = right edge)
    pub x_offset_percent: f32,

    /// Vertical offset as percentage of block height (0.0 = top edge, 1.0 = bottom edge)
    pub y_offset_percent: f32,

    /// Width as percentage of block width (0.0-1.0)
    pub width_percent: f32,

    /// Height as percentage of block height (0.0-1.0)
    pub height_percent: f32,
}

impl Grass {
    /// Creates a new Grass object
    ///
    /// # Arguments
    /// * `x_offset_percent` - X offset as percentage of block width (0.0-1.0)
    /// * `y_offset_percent` - Y offset as percentage of block height (0.0-1.0)
    /// * `width_percent` - Width as percentage of block width (0.0-1.0)
    /// * `height_percent` - Height as percentage of block height (0.0-1.0)
    pub fn new(
        x_offset_percent: f32,
        y_offset_percent: f32,
        width_percent: f32,
        height_percent: f32,
    ) -> Self {
        Self {
            x_offset_percent,
            y_offset_percent,
            width_percent,
            height_percent,
        }
    }

    /// Creates a Grass object that fills the entire block
    ///
    /// # Example
    /// ```
    /// let grass = Grass::fill();
    /// ```
    pub fn fill() -> Self {
        Self {
            x_offset_percent: 0.0,
            y_offset_percent: 0.0,
            width_percent: 1.0,
            height_percent: 1.0,
        }
    }

    /// Creates a Grass object using the builder pattern
    ///
    /// # Example
    /// ```
    /// let grass = Grass::builder()
    ///     .offset(0.1, 0.2)
    ///     .size(0.5, 0.6)
    ///     .build();
    /// ```
    pub fn builder() -> GrassBuilder {
        GrassBuilder::new()
    }
}

impl BlockObject for Grass {
    fn render(&self, block: &Block, _context: &RenderContext) {
        // Get block position and size in pixels
        let block_x = block.x();
        let block_y = block.y();
        let block_width = block.width();
        let block_height = block.height();

        // Calculate grass position relative to block
        let x = block_x + (self.x_offset_percent * block_width);
        let y = block_y + (self.y_offset_percent * block_height);
        let width = self.width_percent * block_width;
        let height = self.height_percent * block_height;

        // Draw main grass rectangle with rounded corners
        draw_rounded_rectangle(x, y, width, height, BLOCK_CORNER_RADIUS, GRASS_COLOR);

        // Add depth edge on right side for 2.5D effect
        draw_rectangle(x + width, y, DEPTH_OFFSET, height, GRASS_DEPTH_COLOR);

        // Add depth edge on bottom for 2.5D effect
        draw_rectangle(
            x,
            y + height,
            width + DEPTH_OFFSET,
            DEPTH_OFFSET,
            GRASS_DEPTH_COLOR,
        );
    }
}

// ============================================================================
// Grass Builder
// ============================================================================

/// Builder for Grass objects
pub struct GrassBuilder {
    x_offset_percent: Option<f32>,
    y_offset_percent: Option<f32>,
    width_percent: Option<f32>,
    height_percent: Option<f32>,
}

impl GrassBuilder {
    /// Creates a new GrassBuilder
    fn new() -> Self {
        Self {
            x_offset_percent: None,
            y_offset_percent: None,
            width_percent: None,
            height_percent: None,
        }
    }

    /// Sets the offset position within the block
    pub fn offset(mut self, x_offset_percent: f32, y_offset_percent: f32) -> Self {
        self.x_offset_percent = Some(x_offset_percent);
        self.y_offset_percent = Some(y_offset_percent);
        self
    }

    /// Sets the x offset within the block
    pub fn x_offset(mut self, x_offset_percent: f32) -> Self {
        self.x_offset_percent = Some(x_offset_percent);
        self
    }

    /// Sets the y offset within the block
    pub fn y_offset(mut self, y_offset_percent: f32) -> Self {
        self.y_offset_percent = Some(y_offset_percent);
        self
    }

    /// Sets the size relative to block size
    pub fn size(mut self, width_percent: f32, height_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self.height_percent = Some(height_percent);
        self
    }

    /// Sets the width relative to block width
    pub fn width(mut self, width_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self
    }

    /// Sets the height relative to block height
    pub fn height(mut self, height_percent: f32) -> Self {
        self.height_percent = Some(height_percent);
        self
    }

    /// Builds the Grass object
    ///
    /// Uses default values if not set:
    /// - x_offset_percent: 0.0 (left edge of block)
    /// - y_offset_percent: 0.0 (top edge of block)
    /// - width_percent: 1.0 (full block width)
    /// - height_percent: 1.0 (full block height)
    pub fn build(self) -> Grass {
        Grass {
            x_offset_percent: self.x_offset_percent.unwrap_or(0.0),
            y_offset_percent: self.y_offset_percent.unwrap_or(0.0),
            width_percent: self.width_percent.unwrap_or(1.0),
            height_percent: self.height_percent.unwrap_or(1.0),
        }
    }
}
