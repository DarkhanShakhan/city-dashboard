//! Centralized configuration constants for the city traffic simulation
//!
//! This module organizes all constants used throughout the application into
//! logical submodules for easy access and maintenance.

use macroquad::prelude::*;

// ============================================================================
// Visual Constants
// ============================================================================

/// Visual appearance constants for rendering
pub mod visual {
    use super::*;

    /// Width of roads in pixels
    pub const ROAD_WIDTH: f32 = 60.0;

    /// Forest green color for grass areas
    pub const GRASS_COLOR: Color = Color::new(0.13, 0.55, 0.13, 1.0);

    /// Darker grass color for 2.5D depth edges
    pub const GRASS_DEPTH_COLOR: Color = Color::new(0.1, 0.45, 0.1, 1.0);

    /// 2.5D depth effect offset in pixels
    pub const DEPTH_OFFSET: f32 = 5.0;

    /// Corner radius for rounded blocks in pixels
    pub const BLOCK_CORNER_RADIUS: f32 = 8.0;

    /// Yellow-white color for road center lines
    pub const LINE_COLOR: Color = Color::new(1.0, 1.0, 0.8, 1.0);

    /// Semi-transparent white for crosswalk stripes
    pub const INTERSECTION_MARK_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.6);

    /// Background color for road surfaces
    pub const ROAD_COLOR: Color = GRAY;
}

// ============================================================================
// Vehicle Constants
// ============================================================================

/// Constants related to car physics and appearance
pub mod vehicle {
    /// Width of car sprite in pixels
    pub const CAR_WIDTH: f32 = 20.0;

    /// Height of car sprite in pixels
    pub const CAR_HEIGHT: f32 = 35.0;

    /// Normal driving speed in pixels per second
    pub const CAR_SPEED: f32 = 50.0;

    /// Lane offset from road center in pixels (for left-hand traffic)
    pub const LANE_OFFSET: f32 = 12.0;

    /// Minimum safe following distance in pixels
    pub const SAFE_FOLLOWING_DISTANCE: f32 = 50.0;

    /// Minimum distance before intersection to stop (pixels)
    pub const STOP_DISTANCE_MIN: f32 = 30.0;

    /// Maximum distance to consider stopping before intersection (pixels)
    pub const STOP_DISTANCE_MAX: f32 = 80.0;

    /// Tolerance for lane detection (pixels)
    pub const LANE_TOLERANCE: f32 = 20.0;

    /// Radius to consider as "in intersection" (pixels)
    pub const INTERSECTION_RADIUS: f32 = 40.0;

    /// Time between car spawns (in seconds)
    pub const CAR_SPAWN_INTERVAL: f32 = 1.5;

    /// Probability of car planning a turn (0.0-1.0)
    pub const TURN_PROBABILITY: f32 = 0.3;
}

// ============================================================================
// Traffic Light Constants
// ============================================================================

/// Constants for traffic light timing and appearance
pub mod traffic_light {
    use macroquad::prelude::*;

    /// Green light duration in seconds
    pub const GREEN_DURATION: f32 = 3.0;

    /// Yellow light duration in seconds
    pub const YELLOW_DURATION: f32 = 1.0;

    /// Red light duration in seconds
    pub const RED_DURATION: f32 = 3.0;

    /// Total traffic light cycle duration in seconds
    pub const CYCLE_DURATION: f32 = GREEN_DURATION + YELLOW_DURATION + RED_DURATION;

    /// Diameter of each light circle in pixels
    pub const TRAFFIC_LIGHT_SIZE: f32 = 12.0;

    /// Space between lights in the traffic light box
    pub const TRAFFIC_LIGHT_SPACING: f32 = 3.0;

    /// 2.5D depth effect offset for traffic lights
    pub const DEPTH_OFFSET: f32 = 3.0;

    /// Bright red color for active red light
    pub const RED_BRIGHT: Color = RED;

    /// Dim red color for inactive red light
    pub const RED_DIM: Color = Color::new(0.3, 0.0, 0.0, 1.0);

    /// Bright yellow color for active yellow light
    pub const YELLOW_BRIGHT: Color = YELLOW;

    /// Dim yellow color for inactive yellow light
    pub const YELLOW_DIM: Color = Color::new(0.3, 0.3, 0.0, 1.0);

    /// Bright green color for active green light
    pub const GREEN_BRIGHT: Color = Color::new(0.0, 1.0, 0.0, 1.0);

    /// Dim green color for inactive green light
    pub const GREEN_DIM: Color = Color::new(0.0, 0.3, 0.0, 1.0);

    /// Traffic light box background color
    pub const BOX_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);

    /// Traffic light box depth edge color
    pub const BOX_DEPTH_COLOR: Color = Color::new(0.1, 0.1, 0.1, 1.0);

    /// Support pole color
    pub const POLE_COLOR: Color = Color::new(0.3, 0.3, 0.3, 1.0);

    /// Support pole depth color
    pub const POLE_DEPTH_COLOR: Color = Color::new(0.15, 0.15, 0.15, 1.0);
}

// ============================================================================
// LED Display Constants
// ============================================================================

/// Constants for LED display appearance and behavior
pub mod led {
    use macroquad::prelude::*;

    /// Very dark background for LED display
    pub const LED_BG_COLOR: Color = Color::new(0.05, 0.05, 0.08, 1.0);

    /// Bright green for active LEDs (normal mode)
    pub const LED_ON_COLOR: Color = Color::new(0.0, 1.0, 0.2, 1.0);

    /// Dim green for inactive LEDs (normal mode)
    pub const LED_OFF_COLOR: Color = Color::new(0.0, 0.15, 0.05, 0.3);

    /// Border color for LED display frame
    pub const LED_BORDER_COLOR: Color = Color::new(0.2, 0.2, 0.25, 1.0);

    /// Bright red for active LEDs (danger mode)
    pub const LED_DANGER_ON_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);

    /// Dim red for inactive LEDs (danger mode)
    pub const LED_DANGER_OFF_COLOR: Color = Color::new(0.15, 0.0, 0.0, 0.3);

    /// LED dot size in pixels
    pub const LED_DOT_SIZE: f32 = 3.0;

    /// Space between LED dots in pixels
    pub const LED_SPACING: f32 = 5.0;

    /// LED display padding in pixels
    pub const LED_PADDING: f32 = 5.0;

    /// LED display height in pixels
    pub const LED_DISPLAY_HEIGHT: f32 = 60.0;

    /// Width of each LED character in dots (5-bit patterns)
    pub const LED_CHAR_WIDTH: usize = 5;

    /// Height of each LED character in dots
    pub const LED_CHAR_HEIGHT: usize = 7;

    /// Space between LED characters in dots
    pub const LED_CHAR_SPACING: usize = 1;

    /// Scroll speed in pixels per second (normal mode)
    pub const LED_SCROLL_SPEED: f32 = 30.0;

    /// Flash speed in flashes per second (danger mode)
    pub const LED_FLASH_SPEED: f32 = 3.0;

    /// Frame thickness in pixels
    pub const FRAME_THICKNESS: f32 = 8.0;

    /// Outer frame color (darker gray)
    pub const FRAME_COLOR_OUTER: Color = Color::new(0.3, 0.3, 0.35, 1.0);

    /// Inner frame color (lighter gray)
    pub const FRAME_COLOR_INNER: Color = Color::new(0.4, 0.4, 0.45, 1.0);

    /// Screw size in pixels
    pub const SCREW_SIZE: f32 = 4.0;

    /// Screw color
    pub const SCREW_COLOR: Color = Color::new(0.2, 0.2, 0.22, 1.0);

    /// Screw center color
    pub const SCREW_CENTER_COLOR: Color = Color::new(0.15, 0.15, 0.17, 1.0);

    /// Support pole width in pixels
    pub const POLE_WIDTH: f32 = 6.0;

    /// Support pole height in pixels
    pub const POLE_HEIGHT: f32 = 25.0;

    /// Support pole color (dark gray)
    pub const POLE_COLOR: Color = Color::new(0.3, 0.3, 0.3, 1.0);

    /// Support pole depth color
    pub const POLE_DEPTH_COLOR: Color = Color::new(0.15, 0.15, 0.15, 1.0);
}

// ============================================================================
// Road Network Constants
// ============================================================================

/// Constants defining the road grid layout
pub mod road_network {
    /// Vertical road positions as percentages of screen width
    pub const VERTICAL_ROAD_POSITIONS: [f32; 3] = [0.15, 0.5, 0.85];

    /// Horizontal road positions as percentages of screen height
    pub const HORIZONTAL_ROAD_POSITIONS: [f32; 2] = [0.25, 0.75];

    /// Number of vertical roads
    pub const VERTICAL_ROAD_COUNT: usize = 3;

    /// Number of horizontal roads
    pub const HORIZONTAL_ROAD_COUNT: usize = 2;
}

// ============================================================================
// Rendering Constants
// ============================================================================

/// Constants for rendering pipeline
pub mod rendering {
    use macroquad::prelude::*;

    /// Dash length for road center lines in pixels
    pub const DASH_LENGTH: f32 = 15.0;

    /// Gap between dashes for road center lines in pixels
    pub const DASH_GAP: f32 = 10.0;

    /// Line width for road center lines in pixels
    pub const LINE_WIDTH: f32 = 2.0;

    /// Size of intersection box in pixels
    pub const INTERSECTION_SIZE: f32 = 40.0;

    /// Crosswalk width in pixels
    pub const CROSSWALK_WIDTH: f32 = 8.0;

    /// Crosswalk stripe width in pixels
    pub const CROSSWALK_STRIPE_WIDTH: f32 = 4.0;

    /// Crosswalk stripe gap in pixels
    pub const CROSSWALK_STRIPE_GAP: f32 = 2.0;

    /// Distance from intersection center for crosswalks
    pub const CROSSWALK_DISTANCE: f32 = 45.0; // INTERSECTION_SIZE + 5.0

    /// Window color for car windshields
    pub const CAR_WINDOW_COLOR: Color = Color::new(0.6, 0.8, 1.0, 1.0);
}

// ============================================================================
// Window and Input Constants
// ============================================================================

/// Constants for window management and input handling
pub mod window {
    /// Minimum pixel change to detect window resize
    pub const RESIZE_THRESHOLD: f32 = 1.0;
}
