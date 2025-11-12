//! System logging and log window rendering
//!
//! This module provides a logging system for tracking critical system events
//! such as SCADA failures, barrier state changes, LED display modes, and
//! emergency traffic control activations.
//!
//! All logged events are marked as CRITICAL and displayed in red.

use macroquad::prelude::*;
use std::collections::VecDeque;

/// A single log entry with timestamp and message
#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: f64,
    pub message: String,
}

/// Log window for displaying critical system events
///
/// Displays recent log entries in a window overlay with timestamps.
/// All entries are critical level (red) and the window can be toggled
/// with the 'L' key.
pub struct LogWindow {
    entries: VecDeque<LogEntry>,
    max_entries: usize,
    visible: bool,
}

impl LogWindow {
    /// Creates a new log window with specified maximum entries
    ///
    /// # Arguments
    /// * `max_entries` - Maximum number of log entries to keep in memory
    ///
    /// # Returns
    /// A new LogWindow instance with empty log history
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
            visible: true,
        }
    }

    /// Logs a critical event message
    ///
    /// Adds a new log entry with current timestamp. If the number of entries
    /// exceeds max_entries, the oldest entry is removed.
    ///
    /// # Arguments
    /// * `message` - The message to log (automatically marked as CRITICAL)
    ///
    /// # Example
    /// ```
    /// log_window.log("SCADA systems toggled");
    /// log_window.log("Barrier gate opened");
    /// ```
    pub fn log(&mut self, message: impl Into<String>) {
        let entry = LogEntry {
            timestamp: get_time(),
            message: message.into(),
        };

        self.entries.push_back(entry);

        // Keep only max_entries
        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    /// Toggles log window visibility
    ///
    /// Called when the user presses the 'L' key to show/hide the log window.
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    /// Renders the log window overlay
    ///
    /// Displays a semi-transparent window in the bottom-left corner with:
    /// - Dark background with border
    /// - Title bar "CRITICAL SYSTEM LOGS"
    /// - Timestamped log entries (newest at top)
    /// - Help text for toggling visibility
    ///
    /// All log entries are displayed in red (critical level).
    pub fn render(&self) {
        if !self.visible {
            return;
        }

        let window_x = 10.0;
        let window_y = screen_height() - 310.0;
        let window_width = 450.0;
        let window_height = 300.0;

        // Draw window background (dark semi-transparent)
        draw_rectangle(
            window_x,
            window_y,
            window_width,
            window_height,
            Color::new(0.1, 0.1, 0.15, 0.95),
        );

        // Draw window border
        draw_rectangle_lines(
            window_x,
            window_y,
            window_width,
            window_height,
            2.0,
            Color::new(0.8, 0.2, 0.2, 1.0), // Red border for critical
        );

        // Draw title bar
        draw_rectangle(
            window_x,
            window_y,
            window_width,
            25.0,
            Color::new(0.2, 0.05, 0.05, 1.0), // Dark red
        );

        draw_text(
            "CRITICAL SYSTEM LOGS",
            window_x + 10.0,
            window_y + 18.0,
            20.0,
            Color::new(1.0, 0.3, 0.3, 1.0), // Light red
        );

        // Draw log entries (newest at top)
        let mut y_offset = window_y + 35.0;
        let line_height = 20.0;
        let padding = 5.0;

        for entry in self.entries.iter().rev() {
            if y_offset > window_y + window_height - 30.0 {
                break; // Don't draw beyond window (leave space for help text)
            }

            // Format timestamp (MM:SS.MS)
            let mins = (entry.timestamp / 60.0) as i32;
            let secs = (entry.timestamp % 60.0) as i32;
            let millis = ((entry.timestamp % 1.0) * 1000.0) as i32;
            let time_str = format!("{:02}:{:02}.{:03}", mins, secs, millis);

            // Draw timestamp
            draw_text(
                &time_str,
                window_x + 10.0,
                y_offset,
                14.0,
                Color::new(0.5, 0.5, 0.5, 1.0),
            );

            // Draw [CRITICAL] prefix
            draw_text(
                "[CRITICAL]",
                window_x + 95.0,
                y_offset,
                14.0,
                Color::new(1.0, 0.0, 0.0, 1.0), // Bright red
            );

            // Draw message (truncate if too long)
            let max_msg_len = 40;
            let msg = if entry.message.len() > max_msg_len {
                format!("{}...", &entry.message[..max_msg_len])
            } else {
                entry.message.clone()
            };

            draw_text(
                &msg,
                window_x + 185.0,
                y_offset,
                14.0,
                WHITE,
            );

            y_offset += line_height;
        }

        // Draw help text at bottom
        draw_text(
            "Press 'L' to toggle log window",
            window_x + 10.0,
            window_y + window_height - 10.0,
            12.0,
            Color::new(0.5, 0.5, 0.5, 1.0),
        );

        // Draw entry count
        let count_text = format!("{}/{} entries", self.entries.len(), self.max_entries);
        draw_text(
            &count_text,
            window_x + window_width - 100.0,
            window_y + window_height - 10.0,
            12.0,
            Color::new(0.5, 0.5, 0.5, 1.0),
        );
    }
}
