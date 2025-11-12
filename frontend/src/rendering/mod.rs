//! Visual rendering system for the city traffic simulation
//!
//! This module handles all drawing operations for the application:
//! - Environment rendering (grass blocks, roads, intersection markings)
//! - Vehicle rendering with directional sprites
//! - LED display with scrolling text and danger warnings
//! - 2.5D depth effects for visual polish
//!
//! The rendering pipeline is organized into distinct layers:
//! 1. Background (grass blocks with depth edges)
//! 2. Road markings (center lines, crosswalks)
//! 3. Traffic elements (traffic lights, cars)
//! 4. UI overlays (LED display)

mod environment;
pub mod led_display;  // Make public for led_display_object
mod roads;
mod vehicles;
mod utils;

// Re-export public API
pub use environment::draw_intersection_markings;
pub use roads::draw_road_lines;
pub use vehicles::{draw_car, draw_guarded_building};
pub use utils::draw_rounded_rectangle;
