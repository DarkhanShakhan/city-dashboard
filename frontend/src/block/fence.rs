//! Fence block object implementation
//!
//! Provides an isometric fence that can be placed around areas in blocks.

use crate::block::{Block, BlockObject, RenderContext};
use macroquad::prelude::*;

// ============================================================================
// Fence Rendering Constants
// ============================================================================

/// Fence height in pixels
const FENCE_HEIGHT: f32 = 8.0;

/// Fence post width in pixels
const FENCE_POST_WIDTH: f32 = 2.0;

/// Spacing between fence posts in pixels
const FENCE_POST_SPACING: f32 = 12.0;

/// Default fence color (brown)
const DEFAULT_FENCE_COLOR: Color = Color::new(0.4, 0.3, 0.2, 1.0);

// ============================================================================
// Fence Object Implementation
// ============================================================================

/// A fence object that renders as an isometric perimeter
///
/// Renders a fence with vertical posts and horizontal rails.
/// Position and size are relative to the containing block.
pub struct Fence {
    /// Horizontal offset as percentage of block width (0.0 = left edge, 1.0 = right edge)
    pub x_offset_percent: f32,

    /// Vertical offset as percentage of block height (0.0 = top edge, 1.0 = bottom edge)
    pub y_offset_percent: f32,

    /// Width as percentage of block width (0.0-1.0)
    pub width_percent: f32,

    /// Depth as percentage of block height (0.0-1.0)
    pub depth_percent: f32,

    /// Fence color
    pub color: Color,
}

impl Fence {
    /// Creates a new Fence object
    ///
    /// # Arguments
    /// * `x_offset_percent` - X offset as percentage of block width (0.0-1.0)
    /// * `y_offset_percent` - Y offset as percentage of block height (0.0-1.0)
    /// * `width_percent` - Width as percentage of block width (0.0-1.0)
    /// * `depth_percent` - Depth as percentage of block height (0.0-1.0)
    /// * `color` - Fence color
    pub fn new(
        x_offset_percent: f32,
        y_offset_percent: f32,
        width_percent: f32,
        depth_percent: f32,
        color: Color,
    ) -> Self {
        Self {
            x_offset_percent,
            y_offset_percent,
            width_percent,
            depth_percent,
            color,
        }
    }

    /// Creates a Fence with default brown color
    pub fn with_size(
        x_offset_percent: f32,
        y_offset_percent: f32,
        width_percent: f32,
        depth_percent: f32,
    ) -> Self {
        Self::new(
            x_offset_percent,
            y_offset_percent,
            width_percent,
            depth_percent,
            DEFAULT_FENCE_COLOR,
        )
    }

    /// Creates a Fence object using the builder pattern
    pub fn builder() -> FenceBuilder {
        FenceBuilder::new()
    }

    /// Darkens the fence color for side faces
    fn darken_color(color: Color, amount: f32) -> Color {
        Color::new(
            (color.r - amount).max(0.0),
            (color.g - amount).max(0.0),
            (color.b - amount).max(0.0),
            color.a,
        )
    }

    /// Renders a horizontal fence segment
    fn render_horizontal_segment(&self, x: f32, y: f32, width: f32, color: Color) {
        let num_posts = (width / FENCE_POST_SPACING) as i32;

        for i in 0..num_posts {
            let post_x = x + i as f32 * FENCE_POST_SPACING;

            // Draw vertical post
            draw_rectangle(
                post_x,
                y - FENCE_HEIGHT,
                FENCE_POST_WIDTH,
                FENCE_HEIGHT,
                color,
            );

            // Draw horizontal rail connecting to next post
            if i < num_posts - 1 {
                draw_rectangle(
                    post_x + FENCE_POST_WIDTH,
                    y - FENCE_HEIGHT / 2.0,
                    FENCE_POST_SPACING - FENCE_POST_WIDTH,
                    2.0,
                    color,
                );
            }
        }
    }

    /// Renders a vertical (depth) fence segment with isometric perspective
    fn render_vertical_segment(&self, x: f32, y: f32, depth: f32, color: Color) {
        let num_posts = (depth / FENCE_POST_SPACING) as i32;

        for i in 0..num_posts {
            let offset = i as f32 * FENCE_POST_SPACING;
            let post_x = x;
            let post_y = y + offset;

            // Post (vertical line going up)
            draw_rectangle(
                post_x,
                post_y - FENCE_HEIGHT,
                FENCE_POST_WIDTH,
                FENCE_HEIGHT,
                color,
            );

            // Horizontal rail (going toward next post in depth)
            if i < num_posts - 1 {
                draw_rectangle(
                    post_x,
                    post_y - FENCE_HEIGHT / 2.0,
                    FENCE_POST_WIDTH,
                    FENCE_POST_SPACING,
                    color,
                );
            }
        }
    }
}

impl BlockObject for Fence {
    fn render(&self, block: &Block, _context: &RenderContext) {
        // Get block position and size in pixels
        let block_x = block.x();
        let block_y = block.y();
        let block_width = block.width();
        let block_height = block.height();

        // Calculate fence position relative to block
        let x = block_x + (self.x_offset_percent * block_width);
        let y = block_y + (self.y_offset_percent * block_height);
        let width = self.width_percent * block_width;
        let depth = self.depth_percent * block_height;

        let darker_color = Self::darken_color(self.color, 0.1);

        // Render fence perimeter (front and right sides visible)

        // Front fence (bottom edge) - lighter color
        self.render_horizontal_segment(x, y + depth, width, self.color);

        // Right fence (right edge) - darker for depth
        self.render_vertical_segment(x + width, y, depth, darker_color);
    }
}

// ============================================================================
// Fence Builder
// ============================================================================

/// Builder for Fence objects
pub struct FenceBuilder {
    x_offset_percent: Option<f32>,
    y_offset_percent: Option<f32>,
    width_percent: Option<f32>,
    depth_percent: Option<f32>,
    color: Option<Color>,
}

impl FenceBuilder {
    /// Creates a new FenceBuilder
    fn new() -> Self {
        Self {
            x_offset_percent: None,
            y_offset_percent: None,
            width_percent: None,
            depth_percent: None,
            color: None,
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
    pub fn size(mut self, width_percent: f32, depth_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self.depth_percent = Some(depth_percent);
        self
    }

    /// Sets the width relative to block width
    pub fn width(mut self, width_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self
    }

    /// Sets the depth relative to block height
    pub fn depth(mut self, depth_percent: f32) -> Self {
        self.depth_percent = Some(depth_percent);
        self
    }

    /// Sets the fence color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Builds the Fence object
    ///
    /// Uses default values if not set:
    /// - x_offset_percent: 0.0 (left edge of block)
    /// - y_offset_percent: 0.0 (top edge of block)
    /// - width_percent: 0.5 (50% of block width)
    /// - depth_percent: 0.5 (50% of block height)
    /// - color: Brown (0.4, 0.3, 0.2, 1.0)
    pub fn build(self) -> Fence {
        Fence {
            x_offset_percent: self.x_offset_percent.unwrap_or(0.0),
            y_offset_percent: self.y_offset_percent.unwrap_or(0.0),
            width_percent: self.width_percent.unwrap_or(0.5),
            depth_percent: self.depth_percent.unwrap_or(0.5),
            color: self.color.unwrap_or(DEFAULT_FENCE_COLOR),
        }
    }
}
