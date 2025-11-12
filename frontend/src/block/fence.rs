//! Fence block object implementation
//!
//! Provides an isometric 3D fence that can be placed around areas in blocks.

use crate::block::{Block, BlockObject, RenderContext};
use macroquad::prelude::*;

// ============================================================================
// Fence Rendering Constants
// ============================================================================

/// Isometric projection X offset factor (cos(30°) ≈ 0.866)
const ISOMETRIC_X_FACTOR: f32 = 0.866;

/// Isometric projection Y offset factor (sin(30°) = 0.5)
const ISOMETRIC_Y_FACTOR: f32 = 0.5;

/// Amount to darken side faces for 3D effect
const FENCE_SIDE_DARKEN: f32 = 0.15;

/// Amount to lighten top face for 3D effect
const FENCE_TOP_LIGHTEN: f32 = 0.1;

/// Default fence color (brown)
const DEFAULT_FENCE_COLOR: Color = Color::new(0.4, 0.3, 0.2, 1.0);

// ============================================================================
// Color Manipulation Helpers
// ============================================================================

/// Darkens a color by a specified amount, clamping to prevent negative values
fn darken_color(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r - amount).max(0.0),
        (color.g - amount).max(0.0),
        (color.b - amount).max(0.0),
        color.a,
    )
}

/// Lightens a color by a specified amount, clamping to prevent values > 1.0
fn lighten_color(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r + amount).min(1.0),
        (color.g + amount).min(1.0),
        (color.b + amount).min(1.0),
        color.a,
    )
}

// ============================================================================
// Fence Object Implementation
// ============================================================================

/// A fence object that renders as an isometric 3D structure
///
/// Renders a fence with 2.5D depth effect showing three visible faces.
pub struct Fence {
    /// Horizontal offset as percentage of block width (0.0 = left edge, 1.0 = right edge)
    pub x_offset_percent: f32,

    /// Vertical offset as percentage of block height (0.0 = top edge, 1.0 = bottom edge)
    pub y_offset_percent: f32,

    /// Width as percentage of block width (0.0-1.0)
    pub width_percent: f32,

    /// Depth as percentage of block height (0.0-1.0)
    pub depth_percent: f32,

    /// Height in pixels (vertical dimension of the 3D fence)
    pub height_pixels: f32,

    /// Fence color
    pub color: Color,
}

impl Fence {
    /// Creates a new Fence object
    pub fn new(
        x_offset_percent: f32,
        y_offset_percent: f32,
        width_percent: f32,
        depth_percent: f32,
        height_pixels: f32,
        color: Color,
    ) -> Self {
        Self {
            x_offset_percent,
            y_offset_percent,
            width_percent,
            depth_percent,
            height_pixels,
            color,
        }
    }

    /// Creates a Fence object using the builder pattern
    pub fn builder() -> FenceBuilder {
        FenceBuilder::new()
    }

    /// Calculates the isometric projection offset for the fence top
    fn calculate_isometric_offset(&self) -> (f32, f32) {
        (
            self.height_pixels * ISOMETRIC_X_FACTOR,
            self.height_pixels * ISOMETRIC_Y_FACTOR,
        )
    }

    /// Gets the color for a specific face of the fence
    fn get_face_color(&self, face: FenceFace) -> Color {
        match face {
            FenceFace::Front => self.color,
            FenceFace::Side => darken_color(self.color, FENCE_SIDE_DARKEN),
            FenceFace::Top => lighten_color(self.color, FENCE_TOP_LIGHTEN),
        }
    }

    /// Renders the front face of the fence
    fn render_front_face(&self, params: &RenderParams) {
        let color = self.get_face_color(FenceFace::Front);

        // Lower triangle
        draw_triangle(
            Vec2 {
                x: params.x,
                y: params.y + params.depth,
            },
            Vec2 {
                x: params.x_top,
                y: params.y_top + params.depth,
            },
            Vec2 {
                x: params.x + params.width,
                y: params.y + params.depth,
            },
            color,
        );

        // Upper triangle
        draw_triangle(
            Vec2 {
                x: params.x + params.width,
                y: params.y + params.depth,
            },
            Vec2 {
                x: params.x_top + params.width,
                y: params.y_top + params.depth,
            },
            Vec2 {
                x: params.x_top,
                y: params.y_top + params.depth,
            },
            color,
        );
    }

    /// Renders the right side face of the fence
    fn render_side_face(&self, params: &RenderParams) {
        let color = self.get_face_color(FenceFace::Side);

        // Back triangle
        draw_triangle(
            Vec2 {
                x: params.x + params.width,
                y: params.y + params.depth,
            },
            Vec2 {
                x: params.x_top + params.width,
                y: params.y_top + params.depth,
            },
            Vec2 {
                x: params.x_top + params.width,
                y: params.y_top,
            },
            color,
        );

        // Front triangle
        draw_triangle(
            Vec2 {
                x: params.x + params.width,
                y: params.y + params.depth,
            },
            Vec2 {
                x: params.x + params.width,
                y: params.y,
            },
            Vec2 {
                x: params.x_top + params.width,
                y: params.y_top,
            },
            color,
        );
    }

    /// Renders the top face of the fence
    fn render_top_face(&self, params: &RenderParams) {
        let color = self.get_face_color(FenceFace::Top);
        draw_rectangle(params.x_top, params.y_top, params.width, params.depth, color);
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

        // Calculate isometric offset for fence top
        let (x_offset, y_offset) = self.calculate_isometric_offset();
        let x_top = x - x_offset;
        let y_top = y - y_offset;

        // Prepare rendering parameters
        let params = RenderParams {
            x,
            y,
            x_top,
            y_top,
            width,
            depth,
        };

        // Render all three visible faces
        self.render_front_face(&params);
        self.render_side_face(&params);
        self.render_top_face(&params);
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Represents the different faces of a 3D fence
enum FenceFace {
    /// Front face (facing camera)
    Front,
    /// Right side face
    Side,
    /// Top face
    Top,
}

/// Parameters for rendering a fence face
struct RenderParams {
    x: f32,
    y: f32,
    x_top: f32,
    y_top: f32,
    width: f32,
    depth: f32,
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
    height_pixels: Option<f32>,
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
            height_pixels: None,
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

    /// Sets the height in pixels
    pub fn height(mut self, height_pixels: f32) -> Self {
        self.height_pixels = Some(height_pixels);
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
    /// - height_pixels: 8.0 (8 pixels tall)
    /// - color: Brown (0.4, 0.3, 0.2, 1.0)
    pub fn build(self) -> Fence {
        Fence {
            x_offset_percent: self.x_offset_percent.unwrap_or(0.0),
            y_offset_percent: self.y_offset_percent.unwrap_or(0.0),
            width_percent: self.width_percent.unwrap_or(0.5),
            depth_percent: self.depth_percent.unwrap_or(0.5),
            height_pixels: self.height_pixels.unwrap_or(8.0),
            color: self.color.unwrap_or(DEFAULT_FENCE_COLOR),
        }
    }
}
