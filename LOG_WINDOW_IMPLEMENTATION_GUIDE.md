# Log Window Implementation Guide for City Dashboard

## Overview

This document outlines three approaches to implement a log window system for displaying messages when components break (LED display, barriers, SCADA systems, etc.).

## Current State Analysis

### Components That Can Break/Change State
Based on the codebase exploration:

1. **SCADA Systems** (`Building.scada_broken`)
   - Toggled with 'S' key
   - All buildings with SCADA can be marked as broken
   - Currently shows visual flash effect when broken
   - Location: `frontend/src/block/building.rs`

2. **LED Display** (danger mode)
   - Toggled with 'Left Shift' key
   - Changes from "WELCOME TO CITY" to flashing red "DANGER"
   - Location: `frontend/src/led_display_object.rs`

3. **Barrier Gate** (`Fence.barrier_open`)
   - Toggled with 'B' key
   - Animates between closed (0°) and open (85°)
   - Location: `frontend/src/block/fence.rs`

4. **Traffic Emergency Stop**
   - Toggled with 'Enter' key
   - Forces all traffic lights to red
   - Location: `frontend/src/main.rs`

### Current Limitations
- **No logging system** - no structured event tracking
- **No error handling** - failures are silently ignored
- **No event history** - can't review what happened

---

## Recommended Approaches

### Option 1: Macroquad Built-in UI (Simple, Lightweight)

**Pros:**
- No additional dependencies
- Native to macroquad
- Simple API for basic use cases
- Minimal performance overhead

**Cons:**
- Limited styling options
- Less feature-rich than egui
- Requires manual scroll handling
- Basic text rendering

**Best for:** Quick implementation, minimal UI needs

#### Implementation Steps

1. **Add Log System Module** (`frontend/src/logging.rs`)

```rust
use macroquad::prelude::*;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: f64,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Info => Color::new(0.7, 0.7, 0.7, 1.0),
            LogLevel::Warning => Color::new(1.0, 0.8, 0.0, 1.0),
            LogLevel::Error => Color::new(1.0, 0.3, 0.0, 1.0),
            LogLevel::Critical => Color::new(1.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn prefix(&self) -> &str {
        match self {
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERROR]",
            LogLevel::Critical => "[CRITICAL]",
        }
    }
}

pub struct LogWindow {
    entries: VecDeque<LogEntry>,
    max_entries: usize,
    visible: bool,
    scroll_offset: f32,
}

impl LogWindow {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
            visible: true,
            scroll_offset: 0.0,
        }
    }

    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) {
        let entry = LogEntry {
            timestamp: get_time(),
            level,
            message: message.into(),
        };

        self.entries.push_back(entry);

        // Keep only max_entries
        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }

        // Auto-scroll to bottom on new entry
        self.scroll_offset = 0.0;
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn render(&self) {
        if !self.visible {
            return;
        }

        let window_x = 10.0;
        let window_y = screen_height() - 310.0;
        let window_width = 400.0;
        let window_height = 300.0;

        // Draw window background
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
            Color::new(0.3, 0.3, 0.4, 1.0),
        );

        // Draw title bar
        draw_rectangle(
            window_x,
            window_y,
            window_width,
            25.0,
            Color::new(0.15, 0.15, 0.2, 1.0),
        );

        draw_text(
            "System Logs",
            window_x + 10.0,
            window_y + 18.0,
            20.0,
            WHITE,
        );

        // Draw log entries
        let mut y_offset = window_y + 35.0;
        let line_height = 20.0;
        let padding = 5.0;

        for entry in self.entries.iter().rev() {
            if y_offset > window_y + window_height - padding {
                break; // Don't draw beyond window
            }

            // Format timestamp (MM:SS)
            let mins = (entry.timestamp / 60.0) as i32;
            let secs = (entry.timestamp % 60.0) as i32;
            let time_str = format!("{:02}:{:02}", mins, secs);

            // Draw timestamp
            draw_text(
                &time_str,
                window_x + 10.0,
                y_offset,
                14.0,
                Color::new(0.5, 0.5, 0.5, 1.0),
            );

            // Draw level prefix
            let prefix = entry.level.prefix();
            draw_text(
                prefix,
                window_x + 60.0,
                y_offset,
                14.0,
                entry.level.color(),
            );

            // Draw message (truncate if too long)
            let max_msg_len = 35;
            let msg = if entry.message.len() > max_msg_len {
                format!("{}...", &entry.message[..max_msg_len])
            } else {
                entry.message.clone()
            };

            draw_text(
                &msg,
                window_x + 120.0,
                y_offset,
                14.0,
                WHITE,
            );

            y_offset += line_height;
        }

        // Draw help text
        draw_text(
            "Press 'L' to toggle",
            window_x + 10.0,
            window_y + window_height - 10.0,
            12.0,
            Color::new(0.5, 0.5, 0.5, 1.0),
        );
    }
}
```

2. **Update main.rs** - Add log system initialization and event logging

```rust
// Add to imports
mod logging;
use logging::{LogWindow, LogLevel};

// In main() after initialization (around line 89)
let mut log_window = LogWindow::new(50); // Keep last 50 entries
log_window.log(LogLevel::Info, "City Dashboard initialized");

// After input handling (around line 115)
if toggle_scada {
    city.toggle_all_scada();
    log_window.log(LogLevel::Warning, "SCADA systems toggled");
}

if reset_scada {
    city.reset_all_scada();
    log_window.log(LogLevel::Info, "SCADA systems reset");
}

if toggle_barrier {
    barrier_open = !barrier_open;
    if barrier_open {
        log_window.log(LogLevel::Warning, "Barrier gate opened");
    } else {
        log_window.log(LogLevel::Info, "Barrier gate closed");
    }
}

// Add log window toggle key (around line 111)
if is_key_pressed(KeyCode::L) {
    log_window.toggle_visibility();
}

// In render phase (around line 168, before next_frame)
log_window.render();
```

3. **Enhanced Event Tracking** - Add more specific event logging

```rust
// Track LED display mode changes
if danger_mode && !previous_danger_mode {
    log_window.log(LogLevel::Critical, "LED Display: DANGER MODE ACTIVATED");
} else if !danger_mode && previous_danger_mode {
    log_window.log(LogLevel::Info, "LED Display: Normal operation resumed");
}

// Track emergency stop
if all_lights_red && !previous_all_lights_red {
    log_window.log(LogLevel::Critical, "EMERGENCY: All traffic lights RED");
} else if !all_lights_red && previous_all_lights_red {
    log_window.log(LogLevel::Info, "Emergency stop deactivated");
}
```

---

### Option 2: egui-macroquad Integration (Advanced, Feature-Rich)

**Pros:**
- Professional immediate-mode GUI
- Built-in scroll areas
- Rich text formatting
- Advanced widgets (collapsing headers, tabs, etc.)
- Better styling and theming
- Search/filter capabilities

**Cons:**
- Additional dependency (~100KB)
- Slightly more complex setup
- Higher learning curve
- More performance overhead

**Best for:** Production-quality UI, advanced features needed

#### Implementation Steps

1. **Add Dependency** - Update `frontend/Cargo.toml`

```toml
[dependencies]
macroquad = "0.4.14"
egui-macroquad = "0.18"  # Latest compatible version
```

2. **Create egui Log System** (`frontend/src/egui_logging.rs`)

```rust
use egui_macroquad::egui;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct LogEntry {
    pub timestamp: f64,
    pub level: LogLevel,
    pub message: String,
    pub component: String, // e.g., "SCADA", "Barrier", "LED Display"
}

#[derive(Clone, Copy, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    pub fn egui_color(&self) -> egui::Color32 {
        match self {
            LogLevel::Info => egui::Color32::from_rgb(150, 150, 150),
            LogLevel::Warning => egui::Color32::from_rgb(255, 200, 0),
            LogLevel::Error => egui::Color32::from_rgb(255, 100, 50),
            LogLevel::Critical => egui::Color32::from_rgb(255, 0, 0),
        }
    }
}

pub struct EguiLogWindow {
    entries: VecDeque<LogEntry>,
    max_entries: usize,
    filter_text: String,
    show_info: bool,
    show_warning: bool,
    show_error: bool,
    show_critical: bool,
}

impl EguiLogWindow {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
            filter_text: String::new(),
            show_info: true,
            show_warning: true,
            show_error: true,
            show_critical: true,
        }
    }

    pub fn log(&mut self, level: LogLevel, component: impl Into<String>, message: impl Into<String>) {
        let entry = LogEntry {
            timestamp: macroquad::prelude::get_time(),
            level,
            message: message.into(),
            component: component.into(),
        };

        self.entries.push_back(entry);

        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    pub fn render(&mut self, egui_ctx: &egui::Context) {
        egui::Window::new("System Logs")
            .default_pos([10.0, 500.0])
            .default_size([500.0, 300.0])
            .resizable(true)
            .show(egui_ctx, |ui| {
                // Filter controls
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    ui.text_edit_singleline(&mut self.filter_text);

                    ui.separator();

                    ui.checkbox(&mut self.show_info, "Info");
                    ui.checkbox(&mut self.show_warning, "Warn");
                    ui.checkbox(&mut self.show_error, "Error");
                    ui.checkbox(&mut self.show_critical, "Crit");

                    if ui.button("Clear").clicked() {
                        self.entries.clear();
                    }
                });

                ui.separator();

                // Scrollable log area
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for entry in self.entries.iter().rev() {
                            // Apply filters
                            let level_visible = match entry.level {
                                LogLevel::Info => self.show_info,
                                LogLevel::Warning => self.show_warning,
                                LogLevel::Error => self.show_error,
                                LogLevel::Critical => self.show_critical,
                            };

                            if !level_visible {
                                continue;
                            }

                            if !self.filter_text.is_empty() {
                                let filter_lower = self.filter_text.to_lowercase();
                                if !entry.message.to_lowercase().contains(&filter_lower)
                                    && !entry.component.to_lowercase().contains(&filter_lower) {
                                    continue;
                                }
                            }

                            // Format timestamp
                            let mins = (entry.timestamp / 60.0) as i32;
                            let secs = (entry.timestamp % 60.0) as i32;
                            let millis = ((entry.timestamp % 1.0) * 1000.0) as i32;

                            ui.horizontal(|ui| {
                                // Timestamp
                                ui.label(
                                    egui::RichText::new(format!("{:02}:{:02}.{:03}", mins, secs, millis))
                                        .color(egui::Color32::from_rgb(100, 100, 100))
                                        .monospace()
                                );

                                // Level badge
                                let level_text = match entry.level {
                                    LogLevel::Info => "INFO",
                                    LogLevel::Warning => "WARN",
                                    LogLevel::Error => "ERROR",
                                    LogLevel::Critical => "CRIT",
                                };

                                ui.label(
                                    egui::RichText::new(level_text)
                                        .color(entry.level.egui_color())
                                        .strong()
                                        .monospace()
                                );

                                // Component
                                ui.label(
                                    egui::RichText::new(&entry.component)
                                        .color(egui::Color32::from_rgb(150, 150, 255))
                                );

                                // Message
                                ui.label(&entry.message);
                            });
                        }
                    });
            });
    }
}
```

3. **Update main.rs for egui**

```rust
// Add imports
mod egui_logging;
use egui_logging::{EguiLogWindow, LogLevel};

#[macroquad::main("City Dashboard")]
async fn main() -> Result<(), macroquad::Error> {
    // After other initialization
    let mut log_window = EguiLogWindow::new(100);
    log_window.log(LogLevel::Info, "System", "City Dashboard initialized");

    loop {
        // ... existing code ...

        // Event logging
        if toggle_scada {
            city.toggle_all_scada();
            log_window.log(LogLevel::Warning, "SCADA", "All SCADA systems toggled");
        }

        if toggle_barrier {
            barrier_open = !barrier_open;
            let status = if barrier_open { "opened" } else { "closed" };
            log_window.log(
                if barrier_open { LogLevel::Warning } else { LogLevel::Info },
                "Barrier",
                format!("Barrier gate {}", status)
            );
        }

        // Render egui before next_frame
        egui_macroquad::ui(|egui_ctx| {
            log_window.render(egui_ctx);
        });

        egui_macroquad::draw();

        next_frame().await;
    }
}
```

---

### Option 3: Custom Rendering (Maximum Control)

**Pros:**
- Complete control over appearance
- No dependencies
- Can match game aesthetic perfectly
- Optimized for specific needs

**Cons:**
- More code to write and maintain
- Need to implement all features manually
- Text wrapping, scrolling all manual

**Best for:** Custom styling requirements, minimal dependencies

See Option 1 for basic implementation - can be extended with:
- Custom fonts/text rendering
- Animated log entries
- Color-coded backgrounds
- Icon system for log levels
- Sound effects on critical logs

---

## Recommended Events to Log

### Critical Events (Red)
- Emergency traffic stop activated
- Danger mode activated
- Multiple SCADA systems failing simultaneously
- Barrier malfunction

### Errors (Orange)
- Individual SCADA failure
- Barrier stuck or jammed
- LED display communication error

### Warnings (Yellow)
- Barrier gate opened
- SCADA system toggled
- Danger mode change
- Traffic pattern anomaly

### Info (Gray/White)
- System startup
- Normal operations resumed
- Settings changed
- Barrier gate closed

---

## Implementation Recommendation

**Start with Option 1** (Macroquad Built-in) because:
1. No new dependencies
2. Quick to implement (~1 hour)
3. Sufficient for basic logging needs
4. Can migrate to egui later if needed

**Upgrade to Option 2** (egui) if you need:
1. Search/filter functionality
2. Professional appearance
3. Complex UI interactions
4. Multiple log windows or tabs

---

## Additional Features to Consider

### 1. Log Persistence
```rust
// Save logs to file on exit
pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path)?;
    for entry in &self.entries {
        writeln!(
            file,
            "{:.2}, {:?}, {}",
            entry.timestamp,
            entry.level,
            entry.message
        )?;
    }
    Ok(())
}
```

### 2. Log Filtering by Component
```rust
pub fn filter_by_component(&self, component: &str) -> Vec<&LogEntry> {
    self.entries
        .iter()
        .filter(|e| e.component == component)
        .collect()
}
```

### 3. Critical Event Counter
```rust
pub fn critical_count(&self) -> usize {
    self.entries
        .iter()
        .filter(|e| matches!(e.level, LogLevel::Critical))
        .count()
}
```

### 4. Sound Alerts
```rust
// Play sound on critical events
if level == LogLevel::Critical {
    // macroquad audio support or external crate
    play_sound(critical_alert_sound);
}
```

---

## Testing Strategy

1. **Manual Testing**
   - Press 'S' → Should log "SCADA systems toggled"
   - Press 'B' → Should log barrier state change
   - Press 'Left Shift' → Should log danger mode change
   - Press 'Enter' → Should log emergency stop

2. **Visual Verification**
   - Logs appear in correct order (newest at top/bottom based on design)
   - Colors match severity levels
   - Timestamps are accurate
   - Window is readable and doesn't overlap critical UI

3. **Performance Testing**
   - Test with 100+ log entries
   - Ensure smooth scrolling
   - Check memory usage with large log buffers

---

## Next Steps

1. Choose implementation approach (recommend Option 1)
2. Create `frontend/src/logging.rs` module
3. Update `main.rs` with log system integration
4. Add event logging at key points
5. Test with all keyboard controls
6. Refine styling and positioning
7. Add persistence if needed

Would you like me to proceed with implementing Option 1 (simple built-in UI) as a starting point?
