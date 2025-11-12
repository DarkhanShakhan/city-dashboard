//! Traffic light structure and management
//!
//! This module handles:
//! - Traffic light data structure
//! - Traffic light timing and state calculation
//! - Visual rendering of traffic lights at intersections
//! - Automatic cycling between green, yellow, and red
//! - Perpendicular light coordination (vertical vs horizontal)
//!
//! Each intersection has two traffic lights positioned diagonally:
//! - Top-right: Controls vertical (north-south) traffic
//! - Bottom-left: Controls horizontal (east-west) traffic

use crate::constants::traffic_light::*;
use crate::intersection::Intersection;
use crate::models::Direction;
use macroquad::prelude::*;

// ============================================================================
// Traffic Light State
// ============================================================================

/// Traffic light states with duration
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LightState {
    /// Red light - stop (duration in seconds)
    Red(f32),

    /// Yellow light - caution (duration in seconds)
    Yellow(f32),

    /// Green light - go (duration in seconds)
    Green(f32),
}

impl LightState {
    /// Gets the duration of this state
    pub fn duration(&self) -> f32 {
        match self {
            LightState::Red(d) => *d,
            LightState::Yellow(d) => *d,
            LightState::Green(d) => *d,
        }
    }

    /// Sets the duration of this state
    pub fn with_duration(self, new_duration: f32) -> Self {
        match self {
            LightState::Red(_) => LightState::Red(new_duration),
            LightState::Yellow(_) => LightState::Yellow(new_duration),
            LightState::Green(_) => LightState::Green(new_duration),
        }
    }

    /// Checks if this is a red light
    pub fn is_red(&self) -> bool {
        matches!(self, LightState::Red(_))
    }

    /// Checks if this is a yellow light
    pub fn is_yellow(&self) -> bool {
        matches!(self, LightState::Yellow(_))
    }

    /// Checks if this is a green light
    pub fn is_green(&self) -> bool {
        matches!(self, LightState::Green(_))
    }

    /// Converts to u8 for rendering compatibility
    pub fn to_u8(&self) -> u8 {
        match self {
            LightState::Red(_) => 0,
            LightState::Yellow(_) => 1,
            LightState::Green(_) => 2,
        }
    }

    /// Creates a default Red state
    pub fn default_red() -> Self {
        LightState::Red(RED_DURATION)
    }

    /// Creates a default Yellow state
    pub fn default_yellow() -> Self {
        LightState::Yellow(YELLOW_DURATION)
    }

    /// Creates a default Green state
    pub fn default_green() -> Self {
        LightState::Green(GREEN_DURATION)
    }
}

// ============================================================================
// Traffic Light Structure
// ============================================================================

/// Represents a traffic light at an intersection
///
/// Traffic lights control vehicle flow and cycle through states
/// based on internal timing.
#[derive(Clone)]
pub struct TrafficLight {
    /// Horizontal position as percentage of screen width
    pub x_percent: f32,

    /// Vertical position as percentage of screen height
    pub y_percent: f32,

    /// Whether this controls vertical (true) or horizontal (false) traffic
    pub controls_vertical: bool,

    /// Direction the traffic light is facing/controlling
    pub direction: Direction,

    /// Current light state (contains duration)
    pub state: LightState,

    /// Time remaining in current state (in seconds)
    pub time_in_state: f32,

    /// Unique identifier
    pub id: usize,
}

impl TrafficLight {
    /// Creates a new traffic light with an initial state
    ///
    /// # Arguments
    /// * `x_percent` - X position as percentage (0.0-1.0)
    /// * `y_percent` - Y position as percentage (0.0-1.0)
    /// * `controls_vertical` - True if controls vertical traffic, false for horizontal
    /// * `direction` - Direction the light is facing/controlling
    /// * `initial_state` - Initial state with duration (e.g., LightState::Green(3.0))
    /// * `id` - Unique identifier
    pub fn new(
        x_percent: f32,
        y_percent: f32,
        controls_vertical: bool,
        direction: Direction,
        initial_state: LightState,
        id: usize,
    ) -> Self {
        let time_in_state = initial_state.duration();

        Self {
            x_percent,
            y_percent,
            controls_vertical,
            direction,
            state: initial_state,
            time_in_state,
            id,
        }
    }

    /// Creates a traffic light using the builder pattern
    ///
    /// # Arguments
    /// * `id` - Unique identifier
    pub fn builder(id: usize) -> TrafficLightBuilder {
        TrafficLightBuilder::new(id)
    }

    /// Converts the percentage-based x position to absolute pixels
    pub fn x(&self) -> f32 {
        self.x_percent * screen_width()
    }

    /// Converts the percentage-based y position to absolute pixels
    pub fn y(&self) -> f32 {
        self.y_percent * screen_height()
    }

    /// Updates the traffic light state based on elapsed time
    ///
    /// This should be called each frame with the delta time to progress the light cycle.
    ///
    /// # Arguments
    /// * `dt` - Delta time (time since last frame in seconds)
    pub fn update(&mut self, dt: f32) {
        self.time_in_state -= dt;

        // Check if it's time to transition to next state
        if self.time_in_state <= 0.0 {
            self.state = self.get_next_state();
            self.time_in_state = self.state.duration();
        }
    }

    /// Gets the next state in the cycle (preserving durations)
    fn get_next_state(&self) -> LightState {
        match self.state {
            LightState::Green(_) => {
                // Get yellow duration from current state or use default
                LightState::Yellow(YELLOW_DURATION)
            }
            LightState::Yellow(_) => {
                // Get red duration from current state or use default
                LightState::Red(RED_DURATION)
            }
            LightState::Red(_) => {
                // Get green duration from current state or use default
                LightState::Green(GREEN_DURATION)
            }
        }
    }

    /// Gets the current state of this traffic light
    ///
    /// # Returns
    /// Current light state (Red, Yellow, or Green with duration)
    pub fn get_state(&self) -> LightState {
        self.state
    }

    /// Sets the traffic light state manually
    ///
    /// # Arguments
    /// * `state` - The new state to set (with duration)
    pub fn set_state(&mut self, state: LightState) {
        self.state = state;
        self.time_in_state = state.duration();
    }

    /// Gets the current state as u8 (for compatibility)
    pub fn get_state_u8(&self) -> u8 {
        self.state.to_u8()
    }

    /// Checks if the light is red
    pub fn is_red(&self) -> bool {
        self.state.is_red()
    }

    /// Checks if the light is yellow
    pub fn is_yellow(&self) -> bool {
        self.state.is_yellow()
    }

    /// Checks if the light is green
    pub fn is_green(&self) -> bool {
        self.state.is_green()
    }

    /// Gets the direction this traffic light is facing/controlling
    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    /// Sets the direction this traffic light is facing/controlling
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Checks if this light controls traffic moving in the given direction
    pub fn controls_direction(&self, direction: Direction) -> bool {
        self.direction == direction
    }

    /// Gets the time remaining in the current state
    pub fn time_remaining(&self) -> f32 {
        self.time_in_state
    }

    /// Gets the progress through the current state (0.0 to 1.0)
    pub fn state_progress(&self) -> f32 {
        let total_duration = self.state.duration();
        1.0 - (self.time_in_state / total_duration)
    }

    /// Gets the duration of the current state
    pub fn current_state_duration(&self) -> f32 {
        self.state.duration()
    }

    /// Renders this traffic light at its position
    ///
    /// # Arguments
    /// * `force_red` - If true, forces the light to show red regardless of current state
    pub fn render(&self, force_red: bool) {
        let state = if force_red { 0 } else { self.get_state_u8() };

        draw_traffic_light(self.x(), self.y(), state);
    }
}

// ============================================================================
// Intersection Traffic Light (Unified Controller)
// ============================================================================

/// Represents which direction currently has or is transitioning from green light
#[derive(Clone, Copy, PartialEq, Debug)]
enum ActiveDirection {
    Vertical,
    Horizontal,
}

/// Unified traffic light controller for an intersection
///
/// This struct manages both vertical and horizontal traffic lights at a single
/// intersection, ensuring they are always properly coordinated (when one is green,
/// the perpendicular direction is red).
#[derive(Clone)]
pub struct IntersectionTrafficLight {
    /// Horizontal position as percentage of screen width
    pub x_percent: f32,

    /// Vertical position as percentage of screen height
    pub y_percent: f32,

    /// Current state for vertical traffic (up/down)
    pub vertical_state: LightState,

    /// Current state for horizontal traffic (left/right)
    pub horizontal_state: LightState,

    /// Time remaining in current state (in seconds)
    pub time_in_state: f32,

    /// Which direction is currently active (green or transitioning)
    active_direction: ActiveDirection,

    /// Unique identifier
    pub id: usize,
}

impl IntersectionTrafficLight {
    /// Creates a new intersection traffic light
    ///
    /// # Arguments
    /// * `x_percent` - X position as percentage (0.0-1.0)
    /// * `y_percent` - Y position as percentage (0.0-1.0)
    /// * `id` - Unique identifier
    /// * `vertical_starts_green` - If true, vertical starts green (horizontal red), else opposite
    pub fn new(x_percent: f32, y_percent: f32, id: usize, vertical_starts_green: bool) -> Self {
        let (vertical_state, horizontal_state, active_direction) = if vertical_starts_green {
            (
                LightState::default_green(),
                LightState::default_red(),
                ActiveDirection::Vertical,
            )
        } else {
            (
                LightState::default_red(),
                LightState::default_green(),
                ActiveDirection::Horizontal,
            )
        };

        Self {
            x_percent,
            y_percent,
            vertical_state,
            horizontal_state,
            time_in_state: if vertical_starts_green {
                vertical_state.duration()
            } else {
                horizontal_state.duration()
            },
            active_direction,
            id,
        }
    }

    /// Converts the percentage-based x position to absolute pixels
    pub fn x(&self) -> f32 {
        self.x_percent * screen_width()
    }

    /// Converts the percentage-based y position to absolute pixels
    pub fn y(&self) -> f32 {
        self.y_percent * screen_height()
    }

    /// Updates the traffic light states based on elapsed time
    ///
    /// Automatically keeps vertical and horizontal lights coordinated.
    /// Each direction cycles through Green → Yellow → Red properly.
    ///
    /// # Arguments
    /// * `dt` - Delta time (time since last frame in seconds)
    pub fn update(&mut self, dt: f32) {
        self.time_in_state -= dt;

        // Check if it's time to transition to next state
        if self.time_in_state <= 0.0 {
            // Transition the active direction through its cycle
            match self.active_direction {
                ActiveDirection::Vertical => {
                    // Advance vertical state
                    let new_vertical_state = self.get_next_state(self.vertical_state);
                    self.vertical_state = new_vertical_state;

                    // If vertical just turned red, switch to horizontal
                    if new_vertical_state.is_red() {
                        self.active_direction = ActiveDirection::Horizontal;
                        self.horizontal_state = LightState::default_green();
                    } else {
                        // Keep horizontal red while vertical is active
                        self.horizontal_state = LightState::default_red();
                    }

                    self.time_in_state = new_vertical_state.duration();
                }
                ActiveDirection::Horizontal => {
                    // Advance horizontal state
                    let new_horizontal_state = self.get_next_state(self.horizontal_state);
                    self.horizontal_state = new_horizontal_state;

                    // If horizontal just turned red, switch to vertical
                    if new_horizontal_state.is_red() {
                        self.active_direction = ActiveDirection::Vertical;
                        self.vertical_state = LightState::default_green();
                    } else {
                        // Keep vertical red while horizontal is active
                        self.vertical_state = LightState::default_red();
                    }

                    self.time_in_state = new_horizontal_state.duration();
                }
            }
        }
    }

    /// Gets the next state in the cycle
    fn get_next_state(&self, current: LightState) -> LightState {
        match current {
            LightState::Green(_) => LightState::Yellow(YELLOW_DURATION),
            LightState::Yellow(_) => LightState::Red(RED_DURATION),
            LightState::Red(_) => LightState::Green(GREEN_DURATION),
        }
    }

    /// Gets the state for a specific direction
    ///
    /// # Arguments
    /// * `direction` - Direction of travel
    ///
    /// # Returns
    /// Light state as u8: 0=red, 1=yellow, 2=green
    pub fn get_state_for_direction(&self, direction: Direction) -> u8 {
        let is_vertical = direction == Direction::Down || direction == Direction::Up;
        let state = if is_vertical {
            self.vertical_state
        } else {
            self.horizontal_state
        };
        state.to_u8()
    }

    /// Gets the vertical light state
    pub fn get_vertical_state(&self) -> u8 {
        self.vertical_state.to_u8()
    }

    /// Gets the horizontal light state
    pub fn get_horizontal_state(&self) -> u8 {
        self.horizontal_state.to_u8()
    }

    /// Renders both traffic lights for this intersection
    ///
    /// # Arguments
    /// * `force_red` - If true, forces all lights to show red (emergency mode)
    pub fn render(&self, force_red: bool) {
        const ROAD_WIDTH: f32 = 60.0;
        let offset = ROAD_WIDTH / 2.0 + 10.0;

        let int_x = self.x();
        let int_y = self.y();

        // Vertical traffic light (top-right corner)
        // Calculate top-right grass block corner
        let top_corner_x = int_x + ROAD_WIDTH / 2.0;
        let top_corner_y = int_y - ROAD_WIDTH / 2.0;

        let v_state = if force_red {
            0
        } else {
            self.get_vertical_state()
        };

        // Position relative to corner
        let v_x = top_corner_x + 10.0;
        let v_y = top_corner_y - 70.0;
        draw_traffic_light(v_x, v_y, v_state);

        // Horizontal traffic light (bottom-left corner)
        // Calculate bottom-left grass block corner
        let bottom_corner_x = int_x - ROAD_WIDTH / 2.0;
        let bottom_corner_y = int_y + ROAD_WIDTH / 2.0;

        let h_state = if force_red {
            0
        } else {
            self.get_horizontal_state()
        };

        // Apply same offset from corner as top-right light (mirrored)
        // Top-right is +10 from corner in X, -70 in Y
        // Bottom-left should be -10 from corner in X, +0 in Y (no extra offset needed)
        let h_x = bottom_corner_x - 30.0;
        let h_y = bottom_corner_y - 35.0;

        draw_traffic_light(h_x, h_y, h_state);
    }
}

// ============================================================================
// Traffic Light Builder
// ============================================================================

/// Builder for creating TrafficLight instances
pub struct TrafficLightBuilder {
    id: usize,
    x_percent: Option<f32>,
    y_percent: Option<f32>,
    controls_vertical: Option<bool>,
    direction: Option<Direction>,
    initial_state: Option<LightState>,
}

impl TrafficLightBuilder {
    /// Creates a new TrafficLightBuilder
    fn new(id: usize) -> Self {
        Self {
            id,
            x_percent: None,
            y_percent: None,
            controls_vertical: None,
            direction: None,
            initial_state: None,
        }
    }

    /// Sets the position
    pub fn position(mut self, x_percent: f32, y_percent: f32) -> Self {
        self.x_percent = Some(x_percent);
        self.y_percent = Some(y_percent);
        self
    }

    /// Sets the x position
    pub fn x(mut self, x_percent: f32) -> Self {
        self.x_percent = Some(x_percent);
        self
    }

    /// Sets the y position
    pub fn y(mut self, y_percent: f32) -> Self {
        self.y_percent = Some(y_percent);
        self
    }

    /// Sets the initial state of the traffic light
    pub fn initial_state(mut self, state: LightState) -> Self {
        self.initial_state = Some(state);
        self
    }

    /// Sets the initial state to Green with default duration
    pub fn start_green(mut self) -> Self {
        self.initial_state = Some(LightState::default_green());
        self
    }

    /// Sets the initial state to Green with custom duration
    pub fn start_green_with(mut self, duration: f32) -> Self {
        self.initial_state = Some(LightState::Green(duration));
        self
    }

    /// Sets the initial state to Red with default duration
    pub fn start_red(mut self) -> Self {
        self.initial_state = Some(LightState::default_red());
        self
    }

    /// Sets the initial state to Red with custom duration
    pub fn start_red_with(mut self, duration: f32) -> Self {
        self.initial_state = Some(LightState::Red(duration));
        self
    }

    /// Sets the initial state to Yellow with default duration
    pub fn start_yellow(mut self) -> Self {
        self.initial_state = Some(LightState::default_yellow());
        self
    }

    /// Sets the initial state to Yellow with custom duration
    pub fn start_yellow_with(mut self, duration: f32) -> Self {
        self.initial_state = Some(LightState::Yellow(duration));
        self
    }

    /// Sets whether this light controls vertical traffic
    pub fn controls_vertical(mut self, vertical: bool) -> Self {
        self.controls_vertical = Some(vertical);
        self
    }

    /// Sets this light to control vertical traffic
    pub fn vertical(mut self) -> Self {
        self.controls_vertical = Some(true);
        self
    }

    /// Sets this light to control horizontal traffic
    pub fn horizontal(mut self) -> Self {
        self.controls_vertical = Some(false);
        self
    }

    /// Sets the direction this light is facing/controlling
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        // Auto-set controls_vertical based on direction
        match direction {
            Direction::Up | Direction::Down => {
                self.controls_vertical = Some(true);
            }
            Direction::Left | Direction::Right => {
                self.controls_vertical = Some(false);
            }
        }
        self
    }

    /// Sets this light to face/control upward traffic
    pub fn facing_up(mut self) -> Self {
        self.direction = Some(Direction::Up);
        self.controls_vertical = Some(true);
        self
    }

    /// Sets this light to face/control downward traffic
    pub fn facing_down(mut self) -> Self {
        self.direction = Some(Direction::Down);
        self.controls_vertical = Some(true);
        self
    }

    /// Sets this light to face/control left traffic
    pub fn facing_left(mut self) -> Self {
        self.direction = Some(Direction::Left);
        self.controls_vertical = Some(false);
        self
    }

    /// Sets this light to face/control right traffic
    pub fn facing_right(mut self) -> Self {
        self.direction = Some(Direction::Right);
        self.controls_vertical = Some(false);
        self
    }

    /// Builds the TrafficLight
    ///
    /// Defaults:
    /// - x_percent: 0.5
    /// - y_percent: 0.5
    /// - controls_vertical: true
    /// - direction: Direction::Down
    /// - initial_state: LightState::Red(3.0)
    pub fn build(self) -> TrafficLight {
        let controls_vertical = self.controls_vertical.unwrap_or(true);
        let direction = self.direction.unwrap_or(if controls_vertical {
            Direction::Down
        } else {
            Direction::Right
        });

        TrafficLight::new(
            self.x_percent.unwrap_or(0.5),
            self.y_percent.unwrap_or(0.5),
            controls_vertical,
            direction,
            self.initial_state.unwrap_or(LightState::default_red()),
            self.id,
        )
    }
}

// ============================================================================
// Traffic Light State Logic
// ============================================================================

/// Calculates the current traffic light state based on time
///
/// # Arguments
/// * `time_offset` - Offset in seconds for this specific light (for staggering)
///
/// # Returns
/// Light state as u8:
/// - 0 = Red (stop)
/// - 1 = Yellow (caution)
/// - 2 = Green (go)
///
/// # Timing Cycle (7 seconds total)
/// - 0-3s: Green
/// - 3-4s: Yellow
/// - 4-7s: Red
pub fn get_traffic_light_state(time_offset: f32) -> u8 {
    // Add time offset and wrap around cycle duration
    let time = (get_time() as f32 + time_offset) % CYCLE_DURATION;

    if time < GREEN_DURATION {
        2 // Green
    } else if time < GREEN_DURATION + YELLOW_DURATION {
        1 // Yellow
    } else {
        0 // Red
    }
}

// ============================================================================
// Traffic Light Rendering
// ============================================================================

/// Renders a single traffic light at the specified position
///
/// Draws a vertical traffic light with three stacked circular lights:
/// red (top), yellow (middle), green (bottom), with a dark box housing
/// and small pole underneath.
///
/// # Arguments
/// * `x` - X position for top-left corner of light box
/// * `y` - Y position for top-left corner of light box
/// * `active_light` - Which light is currently on (0=red, 1=yellow, 2=green)
pub fn draw_traffic_light(x: f32, y: f32, active_light: u8) {
    draw_traffic_light_with_pole_offset(x, y, active_light, 0.0);
}

/// Renders a traffic light with custom pole positioning
///
/// # Arguments
/// * `x` - X position for top-left corner of light box
/// * `y` - Y position for top-left corner of light box
/// * `active_light` - Which light is currently on (0=red, 1=yellow, 2=green)
/// * `pole_x_offset` - Horizontal offset for pole position relative to light box center
pub fn draw_traffic_light_with_pole_offset(x: f32, y: f32, active_light: u8, pole_x_offset: f32) {
    let box_width = TRAFFIC_LIGHT_SIZE + 6.0;
    let box_height = TRAFFIC_LIGHT_SIZE * 3.0 + TRAFFIC_LIGHT_SPACING * 4.0;

    // Draw dark housing box
    draw_rectangle(x, y, box_width, box_height, BOX_COLOR);

    // Draw 2.5D depth edges
    draw_rectangle(x + box_width, y, DEPTH_OFFSET, box_height, BOX_DEPTH_COLOR);
    draw_rectangle(x, y + box_height, box_width, DEPTH_OFFSET, BOX_DEPTH_COLOR);

    // Draw support pole underneath (with optional offset)
    let pole_x = x + box_width / 2.0 - 2.0 + pole_x_offset;
    draw_rectangle(pole_x, y + box_height, 4.0, 12.0, POLE_COLOR);
    // Pole depth edge
    draw_rectangle(
        pole_x + 4.0,
        y + box_height,
        DEPTH_OFFSET,
        12.0,
        POLE_DEPTH_COLOR,
    );

    // Calculate center x for all lights
    let light_x = x + box_width / 2.0;
    let radius = TRAFFIC_LIGHT_SIZE / 2.0;

    // RED light (top)
    let red_y = y + TRAFFIC_LIGHT_SPACING + radius;
    let red_color = if active_light == 0 {
        RED_BRIGHT
    } else {
        RED_DIM
    };
    draw_circle(light_x, red_y, radius, red_color);

    // YELLOW light (middle)
    let yellow_y = red_y + TRAFFIC_LIGHT_SIZE + TRAFFIC_LIGHT_SPACING;
    let yellow_color = if active_light == 1 {
        YELLOW_BRIGHT
    } else {
        YELLOW_DIM
    };
    draw_circle(light_x, yellow_y, radius, yellow_color);

    // GREEN light (bottom)
    let green_y = yellow_y + TRAFFIC_LIGHT_SIZE + TRAFFIC_LIGHT_SPACING;
    let green_color = if active_light == 2 {
        GREEN_BRIGHT
    } else {
        GREEN_DIM
    };
    draw_circle(light_x, green_y, radius, green_color);
}

/// Renders all traffic lights for all intersections
///
/// Places two traffic lights at each intersection:
/// - Top-right corner: Controls north-south (vertical) traffic
/// - Bottom-left corner: Controls east-west (horizontal) traffic
///
/// Lights are coordinated so when vertical is green, horizontal is red, and vice versa.
///
/// # Arguments
/// * `intersections` - All intersections to draw lights at
/// * `all_lights_red` - Emergency mode flag (forces all lights to red)
pub fn draw_traffic_lights(intersections: &[Intersection], all_lights_red: bool) {
    for intersection in intersections {
        intersection.render_lights(all_lights_red);
    }
}
