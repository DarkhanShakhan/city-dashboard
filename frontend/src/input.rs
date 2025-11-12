//! Input handling and window management
//!
//! This module handles:
//! - Window state tracking and resize detection
//! - Keyboard input processing
//! - Traffic control modes (emergency stop, danger mode)
//!
//! The input system allows users to control various aspects of the simulation
//! through keyboard shortcuts.

use macroquad::prelude::*;

// ============================================================================
// Window State Management
// ============================================================================

/// Tracks window dimensions to detect resize events
///
/// This struct maintains the current window size and provides methods
/// to detect when the window has been resized beyond a threshold.
/// This is useful for triggering re-layouts or updates when the window
/// dimensions change.
pub struct WindowState {
    width: f32,
    height: f32,
}

impl WindowState {
    /// Creates a new WindowState initialized with current screen dimensions
    ///
    /// # Returns
    /// A new WindowState with width and height set to current screen size
    pub fn new() -> Self {
        Self {
            width: screen_width(),
            height: screen_height(),
        }
    }

    /// Checks if window was resized beyond the given threshold
    ///
    /// Compares current screen dimensions with stored dimensions.
    /// If the change in either dimension exceeds the threshold,
    /// updates the stored dimensions and returns true.
    ///
    /// # Arguments
    /// * `threshold` - Minimum pixel change to detect as a resize
    ///
    /// # Returns
    /// `true` if resize was detected, `false` otherwise
    ///
    /// # Example
    /// ```
    /// let mut window_state = WindowState::new();
    /// if window_state.check_resize(1.0) {
    ///     println!("Window was resized!");
    /// }
    /// ```
    pub fn check_resize(&mut self, threshold: f32) -> bool {
        let current_width = screen_width();
        let current_height = screen_height();

        let width_changed = (current_width - self.width).abs() > threshold;
        let height_changed = (current_height - self.height).abs() > threshold;

        if width_changed || height_changed {
            self.width = current_width;
            self.height = current_height;
            true
        } else {
            false
        }
    }
}

// ============================================================================
// Input Handling
// ============================================================================

/// Processes keyboard input for traffic control and display modes
///
/// This function handles all keyboard input for controlling the simulation:
/// - Traffic emergency stop mode
/// - Danger warning display
/// - SCADA system control
/// - Reset to normal operation
///
/// # Arguments
/// * `all_lights_red` - Current state of emergency stop mode
/// * `danger_mode` - Current state of danger warning display
///
/// # Returns
/// Tuple of (new_all_lights_red, new_danger_mode, toggle_all_scada, reset_scada, toggle_barrier) with updated states
/// toggle_all_scada is true if all SCADA systems should be toggled
/// reset_scada is true if SCADA should be reset to working state
/// toggle_barrier is true if barrier gate should be toggled
///
/// # Keyboard Controls
/// - **Enter**: Toggle all traffic lights to red (emergency stop)
/// - **Escape**: Reset all modes to normal (including SCADA)
/// - **Left Shift**: Toggle danger warning on LED display
/// - **S**: Toggle SCADA broken state for ALL buildings with SCADA
/// - **B**: Toggle barrier gate (open/close)
///
/// # Example
/// ```
/// let (all_lights_red, danger_mode, toggle_scada, reset_scada, toggle_barrier) = handle_input(false, false);
/// // User pressed 'B'
/// // toggle_barrier is true
/// ```
pub fn handle_input(all_lights_red: bool, danger_mode: bool) -> (bool, bool, bool, bool, bool) {
    let mut new_all_lights_red = all_lights_red;
    let mut new_danger_mode = danger_mode;
    let mut toggle_all_scada = false;
    let mut reset_scada = false;
    let mut toggle_barrier = false;

    // Toggle all traffic lights to red (emergency stop)
    if is_key_pressed(KeyCode::Enter) {
        new_all_lights_red = !new_all_lights_red;
    }

    // Reset all modes to normal (including SCADA)
    if is_key_pressed(KeyCode::Escape) {
        new_all_lights_red = false;
        new_danger_mode = false;
        reset_scada = true;
    }

    // Toggle danger warning on LED display
    if is_key_pressed(KeyCode::LeftShift) {
        new_danger_mode = !new_danger_mode;
    }

    // Toggle all SCADA systems
    if is_key_pressed(KeyCode::S) {
        toggle_all_scada = true;
    }

    // Toggle barrier gate
    if is_key_pressed(KeyCode::B) {
        toggle_barrier = true;
    }

    (new_all_lights_red, new_danger_mode, toggle_all_scada, reset_scada, toggle_barrier)
}
