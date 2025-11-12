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

    /// Building color
    pub color: Color,

    /// Whether this building has SCADA control
    pub has_scada: bool,

    /// Whether the SCADA system is broken (only relevant if has_scada is true)
    pub scada_broken: bool,
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
    /// * `color` - Building color
    pub fn new(
        x_offset_percent: f32,
        y_offset_percent: f32,
        width_percent: f32,
        height_pixels: f32,
        depth_percent: f32,
        corner_radius: f32,
        color: Color,
    ) -> Self {
        Self {
            x_offset_percent,
            y_offset_percent,
            width_percent,
            height_pixels,
            depth_percent,
            corner_radius,
            color,
            has_scada: false,
            scada_broken: false,
        }
    }

    /// Enables SCADA control for this building
    pub fn with_scada(mut self, enabled: bool) -> Self {
        self.has_scada = enabled;
        self
    }

    /// Sets the SCADA broken state
    pub fn set_scada_broken(&mut self, broken: bool) {
        self.scada_broken = broken;
    }

    /// Gets whether SCADA is broken
    pub fn is_scada_broken(&self) -> bool {
        self.has_scada && self.scada_broken
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
    fn render_front_face(&self, params: &RenderParams, time: f64) {
        let color = self.get_face_color_with_scada(BuildingFace::Front, time);

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
    fn render_side_face(&self, params: &RenderParams, time: f64) {
        let color = self.get_face_color_with_scada(BuildingFace::Side, time);

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
    fn render_top_face(&self, params: &RenderParams, time: f64) {
        let color = self.get_face_color_with_scada(BuildingFace::Top, time);
        draw_rounded_rectangle(
            params.x_top,
            params.y_top,
            params.width,
            params.depth,
            self.corner_radius,
            color,
        );
    }

    /// Gets the color for a face when SCADA is broken (flashing between original and red)
    fn get_face_color_with_scada(&self, face: BuildingFace, time: f64) -> Color {
        // DEBUG: Check if SCADA is broken
        let is_broken = self.is_scada_broken();

        if !is_broken {
            return self.get_face_color(face);
        }

        // Flash at 1 Hz (1 flash per second) - slower and more visible
        let flash_frequency = 1.0;
        let flash_value = (time * flash_frequency * std::f64::consts::PI * 2.0).sin();

        // When flash_value > 0, show red; when < 0, show original color
        if flash_value > 0.0 {
            // Bright red color, but keep the same shading for different faces
            let base_red = Color::new(1.0, 0.0, 0.0, 1.0);
            match face {
                BuildingFace::Front => base_red,
                BuildingFace::Side => darken_color(base_red, BUILDING_SIDE_DARKEN),
                BuildingFace::Top => lighten_color(base_red, BUILDING_TOP_LIGHTEN),
            }
        } else {
            // Original color
            self.get_face_color(face)
        }
    }
}

impl BlockObject for Building {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(&self, block: &Block, context: &RenderContext) {
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

        // Render all three visible faces (with SCADA flashing if broken)
        self.render_front_face(&params, context.time);
        self.render_side_face(&params, context.time);
        self.render_top_face(&params, context.time);
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
    color: Option<Color>,
    has_scada: Option<bool>,
    scada_broken: Option<bool>,
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
            color: None,
            has_scada: None,
            scada_broken: None,
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

    /// Sets the building color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets whether the building has SCADA control
    pub fn has_scada(mut self, has_scada: bool) -> Self {
        self.has_scada = Some(has_scada);
        self
    }

    /// Sets whether the SCADA is broken
    pub fn scada_broken(mut self, broken: bool) -> Self {
        self.scada_broken = Some(broken);
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
    /// - color: Gray (0.6, 0.6, 0.6, 1.0)
    /// - has_scada: false
    /// - scada_broken: false
    pub fn build(self) -> Building {
        Building {
            x_offset_percent: self.x_offset_percent.unwrap_or(0.0),
            y_offset_percent: self.y_offset_percent.unwrap_or(0.0),
            width_percent: self.width_percent.unwrap_or(0.3),
            height_pixels: self.height_pixels.unwrap_or(50.0),
            depth_percent: self.depth_percent.unwrap_or(0.3),
            corner_radius: self.corner_radius.unwrap_or(BUILDING_CORNER_RADIUS),
            color: self.color.unwrap_or(Color::new(0.6, 0.6, 0.6, 1.0)),
            has_scada: self.has_scada.unwrap_or(false),
            scada_broken: self.scada_broken.unwrap_or(false),
        }
    }
}
