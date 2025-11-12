//! Building block object implementation
//!
//! Provides a 3D building that can be placed in city blocks with
//! isometric rendering showing front, side, and top faces.

use crate::block::{Block, BlockObject, RenderContext};
use crate::rendering::draw_rounded_rectangle;
use macroquad::prelude::*;

// ============================================================================
// Building Rendering Constants
// ============================================================================

/// Isometric projection X offset factor (cos(30°) ≈ 0.866)
/// Used to calculate horizontal offset for 3D building height
const ISOMETRIC_X_FACTOR: f32 = 0.866;

/// Isometric projection Y offset factor (sin(30°) = 0.5)
/// Used to calculate vertical offset for 3D building height
const ISOMETRIC_Y_FACTOR: f32 = 0.5;

/// Amount to darken side faces of buildings for 3D effect
const BUILDING_SIDE_DARKEN: f32 = 0.15;

/// Amount to lighten top face of buildings for 3D effect
const BUILDING_TOP_LIGHTEN: f32 = 0.1;

/// Corner radius for building top (in pixels)
pub const BUILDING_CORNER_RADIUS: f32 = 8.0;

/// Fence height in pixels
const FENCE_HEIGHT: f32 = 8.0;

/// Fence post width in pixels
const FENCE_POST_WIDTH: f32 = 2.0;

/// Spacing between fence posts in pixels
const FENCE_POST_SPACING: f32 = 12.0;

// ============================================================================
// Color Manipulation Helpers
// ============================================================================

/// Darkens a color by a specified amount, clamping to prevent negative values
///
/// # Arguments
/// * `color` - The original color
/// * `amount` - Amount to subtract from RGB channels (0.0-1.0)
///
/// # Returns
/// A new color with darkened RGB values, alpha channel unchanged
fn darken_color(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r - amount).max(0.0),
        (color.g - amount).max(0.0),
        (color.b - amount).max(0.0),
        color.a,
    )
}

/// Lightens a color by a specified amount, clamping to prevent values > 1.0
///
/// # Arguments
/// * `color` - The original color
/// * `amount` - Amount to add to RGB channels (0.0-1.0)
///
/// # Returns
/// A new color with lightened RGB values, alpha channel unchanged
fn lighten_color(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r + amount).min(1.0),
        (color.g + amount).min(1.0),
        (color.b + amount).min(1.0),
        color.a,
    )
}

// ============================================================================
// Building Object Implementation
// ============================================================================

/// A building object that renders as a 3D cube
///
/// Renders a building with 2.5D depth effect showing three visible faces:
/// front, top, and right side.
/// Position and size are relative to the containing block.
pub struct Building {
    /// Horizontal offset as percentage of block width (0.0 = left edge, 1.0 = right edge)
    pub x_offset_percent: f32,

    /// Vertical offset as percentage of block height (0.0 = top edge, 1.0 = bottom edge)
    pub y_offset_percent: f32,

    /// Width as percentage of block width (0.0-1.0)
    pub width_percent: f32,

    /// Height in pixels (vertical dimension of the 3D building)
    pub height_pixels: f32,

    /// Depth as percentage of block height (0.0-1.0)
    pub depth_percent: f32,

    /// Corner radius in pixels (for rounded top)
    pub corner_radius: f32,

    /// Whether to render a fence around the building
    pub has_fence: bool,

    /// Building color
    pub color: Color,
}

impl Building {
    /// Creates a new Building object
    ///
    /// # Arguments
    /// * `x_offset_percent` - X offset as percentage of block width (0.0-1.0)
    /// * `y_offset_percent` - Y offset as percentage of block height (0.0-1.0)
    /// * `width_percent` - Width as percentage of block width (0.0-1.0)
    /// * `height_pixels` - Height in pixels (vertical dimension of the 3D building)
    /// * `depth_percent` - Depth as percentage of block height (0.0-1.0)
    /// * `corner_radius` - Corner radius in pixels (for rounded top)
    /// * `has_fence` - Whether to render a fence around the building
    /// * `color` - Building color
    pub fn new(
        x_offset_percent: f32,
        y_offset_percent: f32,
        width_percent: f32,
        height_pixels: f32,
        depth_percent: f32,
        corner_radius: f32,
        has_fence: bool,
        color: Color,
    ) -> Self {
        Self {
            x_offset_percent,
            y_offset_percent,
            width_percent,
            height_pixels,
            depth_percent,
            corner_radius,
            has_fence,
            color,
        }
    }

    /// Creates a Building object using the builder pattern
    ///
    /// # Example
    /// ```
    /// let building = Building::builder()
    ///     .offset(0.25, 0.25)
    ///     .width(0.4)
    ///     .height(40.0)
    ///     .depth(0.3)
    ///     .corner_radius(8.0)
    ///     .with_fence(true)
    ///     .color(Color::new(0.5, 0.6, 0.7, 1.0))
    ///     .build();
    /// ```
    pub fn builder() -> BuildingBuilder {
        BuildingBuilder::new()
    }

    /// Calculates the isometric projection offset for the building top corner
    ///
    /// Returns the (x_offset, y_offset) from the base position to the top corner
    /// based on the building's height and isometric projection constants.
    ///
    /// # Returns
    /// Tuple of (x_offset, y_offset) in pixels
    fn calculate_isometric_offset(&self) -> (f32, f32) {
        (
            self.height_pixels * ISOMETRIC_X_FACTOR,
            self.height_pixels * ISOMETRIC_Y_FACTOR,
        )
    }

    /// Gets the color for a specific face of the building
    ///
    /// # Arguments
    /// * `face` - Which face to get the color for
    ///
    /// # Returns
    /// The appropriately shaded color for that face
    fn get_face_color(&self, face: BuildingFace) -> Color {
        match face {
            BuildingFace::Front => self.color,
            BuildingFace::Side => darken_color(self.color, BUILDING_SIDE_DARKEN),
            BuildingFace::Top => lighten_color(self.color, BUILDING_TOP_LIGHTEN),
        }
    }

    /// Renders the front face of the building
    fn render_front_face(&self, params: &RenderParams) {
        let color = self.get_face_color(BuildingFace::Front);

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

    /// Renders the right side face of the building
    fn render_side_face(&self, params: &RenderParams) {
        let color = self.get_face_color(BuildingFace::Side);

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

    /// Renders the top face of the building
    fn render_top_face(&self, params: &RenderParams) {
        let color = self.get_face_color(BuildingFace::Top);
        draw_rounded_rectangle(
            params.x_top,
            params.y_top,
            params.width,
            params.depth,
            self.corner_radius,
            color,
        );
    }

    /// Renders an isometric fence around the building base
    fn render_fence(&self, params: &RenderParams) {
        let fence_color = Color::new(0.4, 0.3, 0.2, 1.0); // Brown fence
        let fence_darken = darken_color(fence_color, 0.1);

        // Calculate fence perimeter with padding around building
        let padding = 8.0;
        let fence_x = params.x - padding;
        let fence_y = params.y - padding;
        let fence_width = params.width + padding * 2.0;
        let fence_depth = params.depth + padding * 2.0;

        // Front fence (bottom edge)
        self.render_fence_segment(
            fence_x,
            fence_y + fence_depth,
            fence_width,
            true,
            fence_color,
        );

        // Right fence (right edge with isometric angle)
        let num_posts = (fence_depth / FENCE_POST_SPACING) as i32;
        for i in 0..num_posts {
            let offset = i as f32 * FENCE_POST_SPACING;
            let post_x = fence_x + fence_width;
            let post_y = fence_y + offset;

            // Post (vertical line going up)
            draw_rectangle(post_x, post_y - FENCE_HEIGHT, FENCE_POST_WIDTH, FENCE_HEIGHT, fence_darken);

            // Horizontal rail
            if i < num_posts - 1 {
                draw_rectangle(
                    post_x,
                    post_y - FENCE_HEIGHT / 2.0,
                    FENCE_POST_WIDTH,
                    FENCE_POST_SPACING,
                    fence_darken,
                );
            }
        }
    }

    /// Renders a horizontal fence segment
    fn render_fence_segment(&self, x: f32, y: f32, width: f32, _is_front: bool, color: Color) {
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
}

impl BlockObject for Building {
    fn render(&self, block: &Block, _context: &RenderContext) {
        // Get block position and size in pixels
        let block_x = block.x();
        let block_y = block.y();
        let block_width = block.width();
        let block_height = block.height();

        // Calculate building position relative to block
        let x = block_x + (self.x_offset_percent * block_width);
        let y = block_y + (self.y_offset_percent * block_height);
        let width = self.width_percent * block_width;
        let depth = self.depth_percent * block_height;

        // Calculate isometric offset for building top
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

        // Render fence first (behind building)
        if self.has_fence {
            self.render_fence(&params);
        }

        // Render all three visible faces
        self.render_front_face(&params);
        self.render_side_face(&params);
        self.render_top_face(&params);
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Represents the different faces of a 3D building
enum BuildingFace {
    /// Front face (facing camera)
    Front,
    /// Right side face
    Side,
    /// Top face
    Top,
}

/// Parameters for rendering a building face
struct RenderParams {
    x: f32,
    y: f32,
    x_top: f32,
    y_top: f32,
    width: f32,
    depth: f32,
}

// ============================================================================
// Building Builder
// ============================================================================

/// Builder for Building objects
pub struct BuildingBuilder {
    x_offset_percent: Option<f32>,
    y_offset_percent: Option<f32>,
    width_percent: Option<f32>,
    height_pixels: Option<f32>,
    depth_percent: Option<f32>,
    corner_radius: Option<f32>,
    has_fence: Option<bool>,
    color: Option<Color>,
}

impl BuildingBuilder {
    /// Creates a new BuildingBuilder
    fn new() -> Self {
        Self {
            x_offset_percent: None,
            y_offset_percent: None,
            width_percent: None,
            height_pixels: None,
            depth_percent: None,
            corner_radius: None,
            has_fence: None,
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

    /// Sets the width relative to block width
    pub fn width(mut self, width_percent: f32) -> Self {
        self.width_percent = Some(width_percent);
        self
    }

    /// Sets the height in pixels
    pub fn height(mut self, height_pixels: f32) -> Self {
        self.height_pixels = Some(height_pixels);
        self
    }

    /// Sets the depth relative to block height
    pub fn depth(mut self, depth_percent: f32) -> Self {
        self.depth_percent = Some(depth_percent);
        self
    }

    /// Sets the corner radius in pixels
    pub fn corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = Some(corner_radius);
        self
    }

    /// Sets whether the building has a fence
    pub fn with_fence(mut self, has_fence: bool) -> Self {
        self.has_fence = Some(has_fence);
        self
    }

    /// Sets the building color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Builds the Building object
    ///
    /// Uses default values if not set:
    /// - x_offset_percent: 0.0 (left edge of block)
    /// - y_offset_percent: 0.0 (top edge of block)
    /// - width_percent: 0.3 (30% of block width)
    /// - height_pixels: 50.0 (50 pixels tall)
    /// - depth_percent: 0.3 (30% of block height)
    /// - corner_radius: 8.0 (8 pixel corner radius)
    /// - has_fence: false (no fence)
    /// - color: Gray (0.6, 0.6, 0.6, 1.0)
    pub fn build(self) -> Building {
        Building {
            x_offset_percent: self.x_offset_percent.unwrap_or(0.0),
            y_offset_percent: self.y_offset_percent.unwrap_or(0.0),
            width_percent: self.width_percent.unwrap_or(0.3),
            height_pixels: self.height_pixels.unwrap_or(50.0),
            depth_percent: self.depth_percent.unwrap_or(0.3),
            corner_radius: self.corner_radius.unwrap_or(BUILDING_CORNER_RADIUS),
            has_fence: self.has_fence.unwrap_or(false),
            color: self.color.unwrap_or(Color::new(0.6, 0.6, 0.6, 1.0)),
        }
    }
}
