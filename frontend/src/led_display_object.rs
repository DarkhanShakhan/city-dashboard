//! LED Display as a BlockObject
//!
//! This module defines LED displays that can be placed in any block.

use crate::block::{Block, BlockObject};
use crate::rendering::led_display::draw_led_display_at;
use macroquad::prelude::*;

/// Display mode for LED text
#[derive(Clone, Debug)]
pub enum LEDDisplayMode {
    /// Static text, centered
    Static,
    /// Scrolling text, right to left
    Scrolling,
    /// Flashing text (3 flashes per second)
    Flashing,
}

/// Color theme for LED display
#[derive(Clone, Debug)]
pub struct LEDColorTheme {
    /// Color when LED is ON (bright)
    pub on_color: Color,
    /// Color when LED is OFF (dim)
    pub off_color: Color,
}

impl LEDColorTheme {
    /// Green theme (default, normal mode)
    pub fn green() -> Self {
        Self {
            on_color: Color::new(0.0, 1.0, 0.0, 1.0),
            off_color: Color::new(0.0, 0.2, 0.0, 0.3),
        }
    }

    /// Red theme (danger/warning mode)
    pub fn red() -> Self {
        Self {
            on_color: Color::new(1.0, 0.0, 0.0, 1.0),
            off_color: Color::new(0.2, 0.0, 0.0, 0.3),
        }
    }

    /// Blue theme (information mode)
    pub fn blue() -> Self {
        Self {
            on_color: Color::new(0.0, 0.5, 1.0, 1.0),
            off_color: Color::new(0.0, 0.1, 0.2, 0.3),
        }
    }

    /// Amber/Orange theme (caution mode)
    pub fn amber() -> Self {
        Self {
            on_color: Color::new(1.0, 0.6, 0.0, 1.0),
            off_color: Color::new(0.2, 0.12, 0.0, 0.3),
        }
    }
}

/// LED Display object that can be placed in blocks
pub struct LEDDisplay {
    /// Text to display
    pub text: String,

    /// Display mode (static, scrolling, flashing)
    pub mode: LEDDisplayMode,

    /// Color theme
    pub theme: LEDColorTheme,

    /// Position within block (0.0-1.0, relative to block's top-left)
    pub x_offset_percent: f32,
    pub y_offset_percent: f32,

    /// Size as fraction of block size (0.0-1.0)
    pub width_scale: f32,
    pub height_scale: f32,
}

impl LEDDisplay {
    /// Creates a new LED display with default settings
    ///
    /// # Arguments
    /// * `text` - The text to display
    ///
    /// # Returns
    /// LEDDisplay with green scrolling text, centered in block
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            mode: LEDDisplayMode::Scrolling,
            theme: LEDColorTheme::green(),
            x_offset_percent: 0.1,  // 10% from left
            y_offset_percent: 0.3,  // 30% from top
            width_scale: 0.8,       // 80% of block width
            height_scale: 0.4,      // 40% of block height
        }
    }

    /// Creates a danger warning LED display
    ///
    /// # Returns
    /// LEDDisplay with red flashing "DANGER" text
    pub fn danger() -> Self {
        Self {
            text: "DANGER".to_string(),
            mode: LEDDisplayMode::Flashing,
            theme: LEDColorTheme::red(),
            x_offset_percent: 0.1,
            y_offset_percent: 0.3,
            width_scale: 0.8,
            height_scale: 0.4,
        }
    }

    /// Sets the text to display
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Sets the display mode
    pub fn with_mode(mut self, mode: LEDDisplayMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the color theme
    pub fn with_theme(mut self, theme: LEDColorTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the position within the block
    pub fn with_position(mut self, x_percent: f32, y_percent: f32) -> Self {
        self.x_offset_percent = x_percent;
        self.y_offset_percent = y_percent;
        self
    }

    /// Sets the size relative to block
    pub fn with_size(mut self, width_scale: f32, height_scale: f32) -> Self {
        self.width_scale = width_scale;
        self.height_scale = height_scale;
        self
    }
}

impl BlockObject for LEDDisplay {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn render(&self, block: &Block, context: &crate::block::RenderContext) {
        // Calculate absolute position and size
        let block_x = block.x();
        let block_y = block.y();
        let block_width = block.width();
        let block_height = block.height();

        let display_x = block_x + (block_width * self.x_offset_percent);
        let display_y = block_y + (block_height * self.y_offset_percent);
        let display_width = block_width * self.width_scale;
        let display_height = block_height * self.height_scale;

        // Override text, mode, and theme based on danger_mode
        let (text, mode, theme) = if context.danger_mode {
            // Danger mode: red flashing "DANGER"
            ("DANGER", LEDDisplayMode::Flashing, LEDColorTheme::red())
        } else {
            // Normal mode: use configured settings
            (self.text.as_str(), self.mode.clone(), self.theme.clone())
        };

        // Render the LED display
        draw_led_display_at(
            display_x,
            display_y,
            display_width,
            display_height,
            text,
            &mode,
            &theme,
            context.time,
        );
    }
}
