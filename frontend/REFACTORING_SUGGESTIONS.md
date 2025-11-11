# Frontend Refactoring Suggestions

**Generated**: 2025-11-11
**Project**: City Dashboard - Traffic Simulation
**Current State**: 4,376 lines across 12 modules
**Architecture**: Rust + Macroquad (native application)

---

## Overview

The frontend is well-architected with excellent documentation and clean separation of concerns. These suggestions aim to improve modularity, type safety, performance, and maintainability.

---

## 🔴 HIGH PRIORITY

### 1. Split Large Modules

**Problem**: `rendering.rs` (766 lines) and `traffic_light.rs` (677 lines) are too large

**Solution**: Break into smaller, focused modules

```
frontend/src/rendering/
├── mod.rs              - Public API
├── road_renderer.rs    - Road & intersection rendering
├── vehicle_renderer.rs - Car rendering
├── led_display.rs      - LED display logic
└── environment.rs      - Grass & background

frontend/src/traffic/
├── mod.rs              - Public API
├── light.rs            - TrafficLight struct and state
├── light_renderer.rs   - Traffic light rendering
└── control.rs          - Traffic light timing logic
```

**Benefits**:
- Easier maintenance and navigation
- Better testability
- Clearer responsibilities
- Reduced cognitive load

**Files to modify**:
- `src/rendering.rs` → split into `src/rendering/*.rs`
- `src/traffic_light.rs` → split into `src/traffic/*.rs`
- `src/main.rs` → update imports

**Estimated effort**: 4-6 hours

---

### 2. Extract LED Character Patterns

**Problem**: 100+ lines of character patterns clutter `rendering.rs` (lines 52-139)

**Solution**: Move to separate const module or data structure

**Create**: `src/rendering/led_chars.rs`

```rust
pub const LED_PATTERNS: &[(char, [u8; 7])] = &[
    ('A', [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001]),
    ('B', [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110]),
    ('C', [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110]),
    // ... all other patterns
];

pub fn get_led_char_pattern(c: char) -> [u8; 7] {
    LED_PATTERNS.iter()
        .find(|(ch, _)| *ch == c.to_ascii_uppercase())
        .map(|(_, pattern)| *pattern)
        .unwrap_or([0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111])
}
```

**Benefits**:
- Cleaner rendering module
- Easier to add new characters
- Could later load from external file

**Files to modify**:
- Create `src/rendering/led_chars.rs`
- `src/rendering.rs` → remove lines 52-139, import from new module

**Estimated effort**: 1 hour

---

### 3. Reduce Car Update Complexity

**Problem**: `car.rs` update logic is complex with multiple nested functions and concerns

**Solution**: Implement behavior system with trait-based composition

**Create**: `src/simulation/` module

```rust
// src/simulation/mod.rs
pub mod behaviors;
pub mod context;

// src/simulation/context.rs
pub struct SimulationContext<'a> {
    pub intersections: &'a [Intersection],
    pub all_cars: &'a [Car],
    pub all_lights_red: bool,
    pub dt: f32,
}

// src/simulation/behaviors.rs
pub trait CarBehavior {
    fn should_stop(&self, car: &Car, context: &SimulationContext) -> bool;
    fn update(&self, car: &mut Car, context: &SimulationContext);
}

pub struct TrafficLightBehavior;
impl CarBehavior for TrafficLightBehavior {
    fn should_stop(&self, car: &Car, context: &SimulationContext) -> bool {
        // Move traffic light checking logic here
    }
    fn update(&self, car: &mut Car, context: &SimulationContext) {
        // No-op for this behavior
    }
}

pub struct CollisionAvoidanceBehavior;
impl CarBehavior for CollisionAvoidanceBehavior {
    fn should_stop(&self, car: &Car, context: &SimulationContext) -> bool {
        // Move collision checking logic here
    }
    fn update(&self, car: &mut Car, context: &SimulationContext) {
        // No-op for this behavior
    }
}

pub struct IntersectionNavigationBehavior;
impl CarBehavior for IntersectionNavigationBehavior {
    fn should_stop(&self, car: &Car, context: &SimulationContext) -> bool {
        // Check for occupied intersection
    }
    fn update(&self, car: &mut Car, context: &SimulationContext) {
        // Handle turning logic
    }
}

pub struct MovementBehavior;
impl CarBehavior for MovementBehavior {
    fn should_stop(&self, _car: &Car, _context: &SimulationContext) -> bool {
        false
    }
    fn update(&self, car: &mut Car, context: &SimulationContext) {
        // Move car based on direction and dt
    }
}

// Compose behaviors
pub struct CarBehaviorChain {
    behaviors: Vec<Box<dyn CarBehavior>>,
}

impl CarBehaviorChain {
    pub fn default_chain() -> Self {
        Self {
            behaviors: vec![
                Box::new(TrafficLightBehavior),
                Box::new(CollisionAvoidanceBehavior),
                Box::new(IntersectionNavigationBehavior),
                Box::new(MovementBehavior),
            ],
        }
    }

    pub fn should_stop(&self, car: &Car, context: &SimulationContext) -> bool {
        self.behaviors.iter().any(|b| b.should_stop(car, context))
    }

    pub fn update(&self, car: &mut Car, context: &SimulationContext) {
        for behavior in &self.behaviors {
            behavior.update(car, context);
        }
    }
}
```

**Benefits**:
- Single Responsibility Principle
- Much easier to test individual behaviors
- Can add new behaviors without modifying existing code
- Clearer separation of concerns

**Files to modify**:
- Create `src/simulation/` directory
- Refactor `src/car.rs` to use behavior system
- Update `src/city.rs` to use new system

**Estimated effort**: 8-12 hours

---

### 4. Improve Type Safety with Newtypes

**Problem**: Many raw `f32` and `usize` values that could be confused

**Solution**: Use newtype pattern for domain concepts

**Create**: `src/models/newtypes.rs`

```rust
use std::fmt;

/// Position as percentage of screen dimension (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(f32);

impl Percentage {
    pub fn new(value: f32) -> Result<Self, &'static str> {
        if (0.0..=1.0).contains(&value) {
            Ok(Percentage(value))
        } else {
            Err("Percentage must be between 0.0 and 1.0")
        }
    }

    pub fn new_unchecked(value: f32) -> Self {
        Percentage(value.clamp(0.0, 1.0))
    }

    pub fn to_pixels(&self, dimension: f32) -> f32 {
        self.0 * dimension
    }

    pub fn from_pixels(pixels: f32, dimension: f32) -> Self {
        Percentage((pixels / dimension).clamp(0.0, 1.0))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Unique identifier for a road
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoadId(pub usize);

impl fmt::Display for RoadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Road({})", self.0)
    }
}

/// Unique identifier for an intersection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntersectionId(pub usize);

impl fmt::Display for IntersectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Intersection({})", self.0)
    }
}

/// Unique identifier for a block
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block({})", self.0)
    }
}

/// Unique identifier for a car
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CarId(pub usize);

impl fmt::Display for CarId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Car({})", self.0)
    }
}
```

**Update**: `src/models.rs`

```rust
use crate::models::newtypes::*;

pub struct Car {
    pub id: CarId,                  // NEW
    pub x_percent: Percentage,      // Was f32
    pub y_percent: Percentage,      // Was f32
    pub direction: Direction,
    pub color: Color,
    pub road_index: RoadId,         // Was usize
    pub next_turn: Option<Direction>,
    pub just_turned: bool,
    pub in_intersection: bool,
    pub location: CarLocation,
}

impl Car {
    pub fn x(&self) -> f32 {
        self.x_percent.to_pixels(screen_width())
    }

    pub fn y(&self) -> f32 {
        self.y_percent.to_pixels(screen_height())
    }

    pub fn set_x(&mut self, x: f32) {
        self.x_percent = Percentage::from_pixels(x, screen_width());
    }

    pub fn set_y(&mut self, y: f32) {
        self.y_percent = Percentage::from_pixels(y, screen_height());
    }
}

pub enum CarLocation {
    OnRoad { road_id: RoadId },                          // Was usize
    InIntersection { intersection_id: IntersectionId },  // Was usize
    InBlock { block_id: BlockId },                       // Was usize
}
```

**Benefits**:
- Prevents mixing up different types of numeric values
- Compiler catches type errors at compile time
- Self-documenting code
- Easier refactoring

**Files to modify**:
- Create `src/models/newtypes.rs`
- Update `src/models.rs`, `src/car.rs`, `src/city.rs`, `src/road.rs`, `src/intersection.rs`, `src/block.rs`

**Estimated effort**: 6-8 hours

---

### 5. Avoid Cloning Entire Car Vector

**Problem**: `car.rs:519` clones entire cars vector for collision checking

```rust
let cars_copy = cars.clone();  // ❌ Expensive! Clones all cars every frame
```

**Solution Option 1**: Two-pass update (recommended)

```rust
// src/car.rs

#[derive(Clone)]
struct CarDecision {
    should_stop: bool,
    new_direction: Option<Direction>,
    new_position: Option<(f32, f32)>,
    remove: bool,
}

pub fn update_cars(
    cars: &mut Vec<Car>,
    intersections: &[Intersection],
    dt: f32,
    all_lights_red: bool,
) {
    // Pass 1: Calculate all decisions (read-only, no cloning needed)
    let decisions: Vec<CarDecision> = cars.iter()
        .map(|car| {
            let at_intersection = update_car_at_intersection_check(car, intersections);
            let should_stop = should_car_stop(car, intersections, cars, all_lights_red);
            let new_position = if !should_stop {
                Some(calculate_new_position(car, dt))
            } else {
                None
            };
            let remove = !is_car_on_screen(car);

            CarDecision {
                should_stop,
                new_direction: None, // Calculate if turning
                new_position,
                remove,
            }
        })
        .collect();

    // Pass 2: Apply decisions (write)
    let mut i = 0;
    cars.retain_mut(|car| {
        let decision = &decisions[i];
        i += 1;

        if let Some((x, y)) = decision.new_position {
            car.x_percent = x;
            car.y_percent = y;
        }

        if let Some(direction) = decision.new_direction {
            car.direction = direction;
        }

        !decision.remove
    });
}
```

**Solution Option 2**: Spatial partitioning (more complex, better for many cars)

```rust
// src/spatial_grid.rs
use std::collections::HashMap;

pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>, // cell coordinate -> car indices
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert(&mut self, car_index: usize, x: f32, y: f32) {
        let cell = self.get_cell(x, y);
        self.cells.entry(cell).or_default().push(car_index);
    }

    pub fn nearby_cars(&self, x: f32, y: f32) -> Vec<usize> {
        let cell = self.get_cell(x, y);
        let mut nearby = Vec::new();

        // Check 3x3 grid of cells around the position
        for dx in -1..=1 {
            for dy in -1..=1 {
                let check_cell = (cell.0 + dx, cell.1 + dy);
                if let Some(indices) = self.cells.get(&check_cell) {
                    nearby.extend(indices);
                }
            }
        }

        nearby
    }

    fn get_cell(&self, x: f32, y: f32) -> (i32, i32) {
        ((x / self.cell_size) as i32, (y / self.cell_size) as i32)
    }
}

// Usage in car.rs
pub fn update_cars(
    cars: &mut Vec<Car>,
    intersections: &[Intersection],
    dt: f32,
    all_lights_red: bool,
) {
    // Build spatial grid
    let mut grid = SpatialGrid::new(100.0); // 100px cells
    for (i, car) in cars.iter().enumerate() {
        grid.insert(i, car.x(), car.y());
    }

    // Update each car, only checking nearby cars
    cars.retain_mut(|car| {
        let nearby_indices = grid.nearby_cars(car.x(), car.y());
        let nearby_cars: Vec<&Car> = nearby_indices.iter()
            .filter_map(|&i| cars.get(i))
            .collect();

        let stop = should_car_stop(car, intersections, &nearby_cars, all_lights_red);
        if !stop {
            move_car(car, dt);
        }

        is_car_on_screen(car)
    });
}
```

**Benefits**:
- Eliminates expensive clone operation every frame
- Significantly better performance (especially with many cars)
- More scalable architecture

**Files to modify**:
- `src/car.rs` → refactor `update_cars` function
- Optionally create `src/spatial_grid.rs` for spatial partitioning

**Estimated effort**: 4-6 hours (Option 1), 8-10 hours (Option 2)

---

## 🟡 MEDIUM PRIORITY

### 6. Implement Error Handling

**Problem**: No error handling, potential panics with `.unwrap()` and array indexing

**Solution**: Use `Result` and `Option` properly throughout

**Create**: `src/error.rs`

```rust
use std::fmt;

#[derive(Debug, Clone)]
pub enum SimulationError {
    InvalidRoadId(usize),
    InvalidIntersectionId(usize),
    InvalidBlockId(usize),
    InvalidPosition { x: f32, y: f32 },
    InvalidPercentage { value: f32 },
    ConfigurationError(String),
    RenderError(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRoadId(id) => write!(f, "Invalid road ID: {}", id),
            Self::InvalidIntersectionId(id) => write!(f, "Invalid intersection ID: {}", id),
            Self::InvalidBlockId(id) => write!(f, "Invalid block ID: {}", id),
            Self::InvalidPosition { x, y } => write!(f, "Invalid position: ({}, {})", x, y),
            Self::InvalidPercentage { value } => {
                write!(f, "Invalid percentage: {} (must be 0.0-1.0)", value)
            }
            Self::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            Self::RenderError(msg) => write!(f, "Render error: {}", msg),
        }
    }
}

impl std::error::Error for SimulationError {}

pub type Result<T> = std::result::Result<T, SimulationError>;
```

**Update**: Methods to return `Result`

```rust
// src/city.rs
use crate::error::{Result, SimulationError};

impl City {
    pub fn get_road(&self, id: usize) -> Result<&Road> {
        self.roads.get(&id)
            .ok_or(SimulationError::InvalidRoadId(id))
    }

    pub fn get_intersection(&self, id: usize) -> Result<&Intersection> {
        self.intersections.get(&id)
            .ok_or(SimulationError::InvalidIntersectionId(id))
    }

    pub fn get_block(&self, id: usize) -> Result<&Block> {
        self.blocks.get(&id)
            .ok_or(SimulationError::InvalidBlockId(id))
    }
}
```

**Benefits**:
- Graceful error handling instead of panics
- Better error messages for debugging
- More robust application

**Files to modify**:
- Create `src/error.rs`
- Update `src/city.rs`, `src/models.rs`, and other modules to use `Result`
- `src/main.rs` → handle errors gracefully

**Estimated effort**: 6-8 hours

---

### 7. Add Configuration File Support

**Problem**: All constants are hardcoded in code, requiring recompilation for changes

**Solution**: Add runtime configuration with fallback to defaults

**Add to** `Cargo.toml`:

```toml
[dependencies]
macroquad = "0.4.14"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
```

**Create**: `src/config.rs`

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub vehicle: VehicleConfig,
    pub traffic_light: TrafficLightConfig,
    pub rendering: RenderingConfig,
    pub road_network: RoadNetworkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleConfig {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub lane_offset: f32,
    pub safe_following_distance: f32,
    pub spawn_interval: f32,
    pub turn_probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficLightConfig {
    pub green_duration: f32,
    pub yellow_duration: f32,
    pub red_duration: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingConfig {
    pub road_width: f32,
    pub depth_offset: f32,
    pub led_display_height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadNetworkConfig {
    pub vertical_road_positions: [f32; 3],
    pub horizontal_road_positions: [f32; 2],
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            vehicle: VehicleConfig {
                width: 20.0,
                height: 35.0,
                speed: 50.0,
                lane_offset: 12.0,
                safe_following_distance: 50.0,
                spawn_interval: 1.5,
                turn_probability: 0.3,
            },
            traffic_light: TrafficLightConfig {
                green_duration: 3.0,
                yellow_duration: 1.0,
                red_duration: 3.0,
            },
            rendering: RenderingConfig {
                road_width: 60.0,
                depth_offset: 5.0,
                led_display_height: 60.0,
            },
            road_network: RoadNetworkConfig {
                vertical_road_positions: [0.15, 0.5, 0.85],
                horizontal_road_positions: [0.25, 0.75],
            },
        }
    }
}

impl SimulationConfig {
    pub fn load() -> Self {
        Self::load_from_file("config.toml").unwrap_or_else(|e| {
            eprintln!("Failed to load config: {}. Using defaults.", e);
            Self::default()
        })
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
        Ok(())
    }
}
```

**Create**: `config.toml` (example)

```toml
[vehicle]
width = 20.0
height = 35.0
speed = 50.0
lane_offset = 12.0
safe_following_distance = 50.0
spawn_interval = 1.5
turn_probability = 0.3

[traffic_light]
green_duration = 3.0
yellow_duration = 1.0
red_duration = 3.0

[rendering]
road_width = 60.0
depth_offset = 5.0
led_display_height = 60.0

[road_network]
vertical_road_positions = [0.15, 0.5, 0.85]
horizontal_road_positions = [0.25, 0.75]
```

**Update**: `src/main.rs`

```rust
mod config;
use config::SimulationConfig;

#[macroquad::main("City Dashboard")]
async fn main() -> Result<(), macroquad::Error> {
    let config = SimulationConfig::load();

    // Use config values throughout
    // ...
}
```

**Benefits**:
- No recompilation needed to adjust parameters
- Easy experimentation with different values
- Can ship different configs for different scenarios
- Users can customize without touching code

**Files to modify**:
- Create `src/config.rs`
- Create `config.toml` in project root
- Update `Cargo.toml` with dependencies
- Update `src/main.rs`, `src/city.rs` to use config
- Gradually migrate from `constants.rs` to config

**Estimated effort**: 6-8 hours

---

### 8. Implement Render Caching

**Problem**: Recalculating static geometry positions every frame

**Solution**: Cache transformed coordinates for static elements

**Create**: `src/render_cache.rs`

```rust
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Line {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub color: Color,
}

#[derive(Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

pub struct RenderCache {
    pub road_lines: Vec<Line>,
    pub crosswalks: Vec<Rectangle>,
    pub grass_blocks: Vec<Rectangle>,
    pub valid: bool,
    pub screen_width: f32,
    pub screen_height: f32,
}

impl RenderCache {
    pub fn new() -> Self {
        Self {
            road_lines: Vec::new(),
            crosswalks: Vec::new(),
            grass_blocks: Vec::new(),
            valid: false,
            screen_width: 0.0,
            screen_height: 0.0,
        }
    }

    pub fn invalidate(&mut self) {
        self.valid = false;
    }

    pub fn needs_rebuild(&self) -> bool {
        !self.valid
            || (screen_width() - self.screen_width).abs() > 1.0
            || (screen_height() - self.screen_height).abs() > 1.0
    }

    pub fn rebuild(&mut self, roads: &[Road], intersections: &[Intersection]) {
        self.road_lines.clear();
        self.crosswalks.clear();
        self.grass_blocks.clear();

        // Build cached geometry
        self.build_road_lines(roads);
        self.build_crosswalks(intersections);
        self.build_grass_blocks(roads);

        self.screen_width = screen_width();
        self.screen_height = screen_height();
        self.valid = true;
    }

    fn build_road_lines(&mut self, roads: &[Road]) {
        // Calculate all road line segments once
        // ...
    }

    fn build_crosswalks(&mut self, intersections: &[Intersection]) {
        // Calculate all crosswalk rectangles once
        // ...
    }

    fn build_grass_blocks(&mut self, roads: &[Road]) {
        // Calculate all grass block rectangles once
        // ...
    }
}
```

**Update**: `src/city.rs`

```rust
use crate::render_cache::RenderCache;

pub struct City {
    // ... existing fields ...
    render_cache: RenderCache,
}

impl City {
    pub fn render_environment(&mut self) {
        if self.render_cache.needs_rebuild() {
            self.render_cache.rebuild(&self.roads, &self.intersections);
        }

        // Draw from cache (much faster)
        for line in &self.render_cache.road_lines {
            draw_line(line.x1, line.y1, line.x2, line.y2, 2.0, line.color);
        }
        for rect in &self.render_cache.crosswalks {
            draw_rectangle(rect.x, rect.y, rect.width, rect.height, rect.color);
        }
        // ... etc
    }
}
```

**Benefits**:
- Faster rendering (no recalculation each frame)
- Reduces CPU usage
- Smoother frame rates
- Only rebuilds when screen resizes

**Files to modify**:
- Create `src/render_cache.rs`
- Update `src/city.rs` to use caching
- Update `src/rendering.rs` functions to build cache

**Estimated effort**: 4-6 hours

---

### 9. Extract Input Commands Pattern

**Problem**: Input handling directly modifies state flags in `main.rs`

**Solution**: Use Command pattern for better separation and testability

**Create**: `src/commands.rs`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum InputCommand {
    ToggleEmergencyStop,
    ToggleDangerMode,
    ResetSimulation,
    SpawnCarAtRoad { road_id: usize },
    PauseSimulation,
    SpeedUpSimulation,
    SlowDownSimulation,
    ToggleDebugView,
}

impl InputCommand {
    pub fn description(&self) -> &str {
        match self {
            Self::ToggleEmergencyStop => "Toggle emergency stop mode",
            Self::ToggleDangerMode => "Toggle danger warning display",
            Self::ResetSimulation => "Reset simulation to initial state",
            Self::SpawnCarAtRoad { .. } => "Spawn car at specific road",
            Self::PauseSimulation => "Pause/resume simulation",
            Self::SpeedUpSimulation => "Increase simulation speed",
            Self::SlowDownSimulation => "Decrease simulation speed",
            Self::ToggleDebugView => "Toggle debug information display",
        }
    }
}
```

**Update**: `src/input.rs`

```rust
use crate::commands::InputCommand;
use macroquad::prelude::*;

pub fn handle_input() -> Vec<InputCommand> {
    let mut commands = Vec::new();

    if is_key_pressed(KeyCode::Enter) {
        commands.push(InputCommand::ToggleEmergencyStop);
    }

    if is_key_pressed(KeyCode::LeftShift) {
        commands.push(InputCommand::ToggleDangerMode);
    }

    if is_key_pressed(KeyCode::Escape) {
        commands.push(InputCommand::ResetSimulation);
    }

    if is_key_pressed(KeyCode::Space) {
        commands.push(InputCommand::PauseSimulation);
    }

    if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::Plus) {
        commands.push(InputCommand::SpeedUpSimulation);
    }

    if is_key_pressed(KeyCode::Minus) {
        commands.push(InputCommand::SlowDownSimulation);
    }

    if is_key_pressed(KeyCode::D) {
        commands.push(InputCommand::ToggleDebugView);
    }

    // Number keys spawn cars on specific roads
    if is_key_pressed(KeyCode::Key1) {
        commands.push(InputCommand::SpawnCarAtRoad { road_id: 0 });
    }

    commands
}
```

**Update**: `src/main.rs`

```rust
mod commands;
use commands::InputCommand;

#[macroquad::main("City Dashboard")]
async fn main() -> Result<(), macroquad::Error> {
    // ... initialization ...

    let mut all_lights_red = false;
    let mut danger_mode = false;
    let mut paused = false;
    let mut time_scale = 1.0;
    let mut debug_view = false;

    loop {
        let mut dt = get_frame_time();

        // Process input commands
        for command in handle_input() {
            match command {
                InputCommand::ToggleEmergencyStop => {
                    all_lights_red = !all_lights_red;
                    println!("Emergency stop: {}", all_lights_red);
                }
                InputCommand::ToggleDangerMode => {
                    danger_mode = !danger_mode;
                    println!("Danger mode: {}", danger_mode);
                }
                InputCommand::ResetSimulation => {
                    city.clear_cars();
                    all_lights_red = false;
                    danger_mode = false;
                    println!("Simulation reset");
                }
                InputCommand::SpawnCarAtRoad { road_id } => {
                    // Spawn car manually
                    println!("Spawning car on road {}", road_id);
                }
                InputCommand::PauseSimulation => {
                    paused = !paused;
                    println!("Paused: {}", paused);
                }
                InputCommand::SpeedUpSimulation => {
                    time_scale *= 1.5;
                    println!("Speed: {}x", time_scale);
                }
                InputCommand::SlowDownSimulation => {
                    time_scale /= 1.5;
                    println!("Speed: {}x", time_scale);
                }
                InputCommand::ToggleDebugView => {
                    debug_view = !debug_view;
                    println!("Debug view: {}", debug_view);
                }
            }
        }

        // Apply time scale
        if !paused {
            dt *= time_scale;
            city.update(dt, all_lights_red);
        }

        // Rendering...
        city.render_environment();
        city.render_traffic(all_lights_red);
        city.render_overlays(current_time, danger_mode);

        if debug_view {
            city.render_debug_info();
        }

        next_frame().await;
    }
}
```

**Benefits**:
- Cleaner separation of input from state
- Commands can be recorded/replayed for debugging
- Commands can be tested in isolation
- Easier to add new input controls
- Can add command history/undo

**Files to modify**:
- Create `src/commands.rs`
- Update `src/input.rs` to return commands
- Update `src/main.rs` to process commands
- Remove direct state manipulation from input handler

**Estimated effort**: 3-4 hours

---

## 🟢 LOW PRIORITY (Nice to Have)

### 10. Add Unit Tests

**Problem**: No visible test coverage

**Solution**: Add test modules throughout codebase

**Example**: `src/car.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersection::Intersection;
    use macroquad::prelude::*;

    fn create_test_car(x: f32, y: f32, direction: Direction) -> Car {
        Car {
            x_percent: x,
            y_percent: y,
            direction,
            color: RED,
            road_index: 0,
            next_turn: None,
            just_turned: false,
            in_intersection: false,
            location: CarLocation::OnRoad { road_id: 0 },
        }
    }

    fn create_test_intersection(x: f32, y: f32, light_state: u8) -> Intersection {
        // Create test intersection with specific light state
        // ...
    }

    #[test]
    fn test_car_stops_at_red_light() {
        let car = create_test_car(0.5, 0.2, Direction::Down);
        let intersection = create_test_intersection(0.5, 0.3, 0); // Red light

        let should_stop = check_traffic_light_at_intersection(
            &car,
            intersection.x(),
            intersection.y(),
            0
        );

        assert!(should_stop, "Car should stop at red light");
    }

    #[test]
    fn test_car_continues_through_green_light() {
        let car = create_test_car(0.5, 0.2, Direction::Down);
        let intersection = create_test_intersection(0.5, 0.3, 2); // Green light

        let should_stop = check_traffic_light_at_intersection(
            &car,
            intersection.x(),
            intersection.y(),
            2
        );

        assert!(!should_stop, "Car should not stop at green light");
    }

    #[test]
    fn test_car_in_intersection_never_stops() {
        let mut car = create_test_car(0.5, 0.3, Direction::Down);
        car.in_intersection = true;
        let intersection = create_test_intersection(0.5, 0.3, 0); // Red light

        let should_stop = check_traffic_light_at_intersection(
            &car,
            intersection.x(),
            intersection.y(),
            0
        );

        assert!(!should_stop, "Car already in intersection should continue");
    }

    #[test]
    fn test_collision_avoidance() {
        let car = create_test_car(0.5, 0.2, Direction::Down);
        let car_ahead = create_test_car(0.5, 0.25, Direction::Down);

        let should_stop = check_car_collision(&car, &[car_ahead]);

        assert!(should_stop, "Car should stop to avoid collision");
    }

    #[test]
    fn test_is_car_on_screen() {
        let on_screen = create_test_car(0.5, 0.5, Direction::Down);
        let off_screen = create_test_car(1.5, 0.5, Direction::Right);

        assert!(is_car_on_screen(&on_screen));
        assert!(!is_car_on_screen(&off_screen));
    }

    #[test]
    fn test_move_car_updates_position() {
        let mut car = create_test_car(0.5, 0.5, Direction::Down);
        let initial_y = car.y_percent;

        move_car(&mut car, 0.016); // ~1 frame at 60fps

        assert!(car.y_percent > initial_y, "Car should move down");
    }

    #[test]
    fn test_plan_next_turn_returns_perpendicular_direction() {
        // Test multiple times due to randomness
        let mut found_turn = false;
        for _ in 0..100 {
            if let Some(turn) = plan_next_turn(Direction::Down) {
                assert!(
                    turn == Direction::Left || turn == Direction::Right,
                    "Turn from Down should be Left or Right"
                );
                found_turn = true;
            }
        }
        assert!(found_turn, "Should find at least one turn in 100 attempts");
    }
}
```

**Run tests**:
```bash
cd frontend
cargo test
```

**Benefits**:
- Catch bugs early
- Prevent regressions
- Document expected behavior
- Enable confident refactoring

**Files to modify**:
- Add `#[cfg(test)] mod tests` to most modules
- Focus on `car.rs`, `traffic_light.rs`, `intersection.rs`

**Estimated effort**: 12-16 hours (comprehensive coverage)

---

### 11. Consider ECS Architecture (Long-term)

**Problem**: Current object-oriented approach may not scale well for complex simulations

**Solution**: Migrate to Entity Component System (ECS) architecture

**Add to** `Cargo.toml`:

```toml
[dependencies]
hecs = "0.10"  # Lightweight ECS, or use bevy_ecs
```

**Example ECS structure**:

```rust
// src/ecs/components.rs
use hecs::*;

#[derive(Clone, Copy)]
pub struct Position {
    pub x_percent: f32,
    pub y_percent: f32,
}

#[derive(Clone, Copy)]
pub struct Velocity {
    pub direction: Direction,
    pub speed: f32,
}

#[derive(Clone, Copy)]
pub struct CarComponent {
    pub color: Color,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy)]
pub struct TrafficState {
    pub in_intersection: bool,
    pub just_turned: bool,
    pub next_turn: Option<Direction>,
}

// src/ecs/systems.rs
pub fn movement_system(world: &mut World, dt: f32) {
    for (_, (pos, vel)) in world.query_mut::<(&mut Position, &Velocity)>() {
        let (dx, dy) = vel.direction.to_vector();
        pos.x_percent += dx * vel.speed * dt / screen_width();
        pos.y_percent += dy * vel.speed * dt / screen_height();
    }
}

pub fn traffic_light_system(
    world: &World,
    intersections: &[Intersection],
    all_lights_red: bool,
) -> Vec<(Entity, bool)> {
    let mut stop_decisions = Vec::new();

    for (entity, (pos, vel, state)) in world.query::<(&Position, &Velocity, &TrafficState)>().iter() {
        let should_stop = check_traffic_lights(pos, vel, state, intersections, all_lights_red);
        stop_decisions.push((entity, should_stop));
    }

    stop_decisions
}

pub fn collision_system(world: &World) -> Vec<(Entity, bool)> {
    // Spatial query for nearby entities
    // Return stop decisions
}

pub fn rendering_system(world: &World) {
    for (_, (pos, car)) in world.query::<(&Position, &CarComponent)>().iter() {
        draw_car_at(pos.x_percent, pos.y_percent, car);
    }
}

// src/main.rs
use hecs::World;

let mut world = World::new();

// Spawn cars as entities
let car_entity = world.spawn((
    Position { x_percent: 0.5, y_percent: 0.2 },
    Velocity { direction: Direction::Down, speed: 50.0 },
    CarComponent { color: RED, width: 20.0, height: 35.0 },
    TrafficState { in_intersection: false, just_turned: false, next_turn: None },
));

// Update loop
loop {
    let dt = get_frame_time();

    // Run systems
    let stop_decisions = traffic_light_system(&world, &intersections, all_lights_red);
    let collision_decisions = collision_system(&world);

    // Apply decisions and move
    movement_system(&mut world, dt);

    // Render
    rendering_system(&world);

    next_frame().await;
}
```

**Benefits**:
- Better performance (data-oriented design)
- More flexible and composable
- Easier to add/remove features
- Better cache locality
- Scales to thousands of entities

**Drawbacks**:
- Significant architectural change
- Learning curve for ECS
- More boilerplate initially

**Files to modify**:
- Major refactor of entire codebase
- Create `src/ecs/` module structure
- Rewrite `main.rs` game loop
- Convert all models to components

**Estimated effort**: 40-60 hours (full migration)

---

### 12. Add Debug Visualization

**Problem**: Hard to debug traffic logic, intersection states, car decisions

**Solution**: Add debug overlay with detailed information

**Create**: `src/debug.rs`

```rust
use crate::{City, Car, Intersection};
use macroquad::prelude::*;

pub struct DebugView {
    pub enabled: bool,
    pub show_car_info: bool,
    pub show_intersection_info: bool,
    pub show_collision_zones: bool,
    pub show_fps: bool,
}

impl DebugView {
    pub fn new() -> Self {
        Self {
            enabled: false,
            show_car_info: true,
            show_intersection_info: true,
            show_collision_zones: true,
            show_fps: true,
        }
    }

    pub fn render(&self, city: &City, dt: f32) {
        if !self.enabled {
            return;
        }

        // FPS counter
        if self.show_fps {
            let fps = (1.0 / dt) as i32;
            draw_text(&format!("FPS: {}", fps), 10.0, 20.0, 20.0, WHITE);
        }

        // Car count
        draw_text(
            &format!("Cars: {}", city.car_count()),
            10.0,
            40.0,
            20.0,
            WHITE,
        );

        // Car information
        if self.show_car_info {
            for (i, car) in city.cars.iter().enumerate() {
                self.render_car_debug(i, car);
            }
        }

        // Intersection information
        if self.show_intersection_info {
            for intersection in city.intersections.values() {
                self.render_intersection_debug(intersection, &city.cars);
            }
        }

        // Collision zones
        if self.show_collision_zones {
            self.render_collision_zones(city);
        }
    }

    fn render_car_debug(&self, index: usize, car: &Car) {
        let x = car.x();
        let y = car.y();

        // Car ID
        draw_text(
            &format!("{}", index),
            x - 5.0,
            y - 20.0,
            16.0,
            WHITE,
        );

        // Direction indicator
        let (dx, dy) = car.direction.to_vector();
        draw_line(
            x,
            y,
            x + dx * 30.0,
            y + dy * 30.0,
            2.0,
            YELLOW,
        );

        // State indicators
        if car.in_intersection {
            draw_circle_lines(x, y, 25.0, 2.0, GREEN);
        }

        if car.just_turned {
            draw_circle(x, y - 30.0, 5.0, ORANGE);
        }

        if car.next_turn.is_some() {
            draw_text("TURN", x + 10.0, y - 20.0, 12.0, CYAN);
        }
    }

    fn render_intersection_debug(&self, intersection: &Intersection, cars: &[Car]) {
        let x = intersection.x();
        let y = intersection.y();

        // Intersection boundary
        draw_circle_lines(x, y, 40.0, 1.0, Color::new(1.0, 1.0, 0.0, 0.5));

        // Count cars in intersection
        let cars_in_intersection = cars
            .iter()
            .filter(|car| {
                let dx = car.x() - x;
                let dy = car.y() - y;
                (dx * dx + dy * dy).sqrt() < 40.0
            })
            .count();

        // Display count
        draw_text(
            &format!("Cars: {}", cars_in_intersection),
            x - 20.0,
            y + 60.0,
            16.0,
            WHITE,
        );

        // Light states
        draw_text(
            &format!("ID: {}", intersection.id),
            x - 15.0,
            y - 50.0,
            14.0,
            YELLOW,
        );
    }

    fn render_collision_zones(&self, city: &City) {
        for car in &city.cars {
            let x = car.x();
            let y = car.y();

            // Safe following distance zone
            let (dx, dy) = car.direction.to_vector();
            let zone_x = x + dx * 50.0;
            let zone_y = y + dy * 50.0;

            draw_line(
                x,
                y,
                zone_x,
                zone_y,
                1.0,
                Color::new(1.0, 0.0, 0.0, 0.3),
            );
            draw_circle_lines(zone_x, zone_y, 25.0, 1.0, Color::new(1.0, 0.0, 0.0, 0.3));
        }
    }
}
```

**Update**: `src/main.rs`

```rust
mod debug;
use debug::DebugView;

#[macroquad::main("City Dashboard")]
async fn main() -> Result<(), macroquad::Error> {
    // ... initialization ...

    let mut debug_view = DebugView::new();

    loop {
        let dt = get_frame_time();

        // Handle debug toggle
        if is_key_pressed(KeyCode::F3) {
            debug_view.enabled = !debug_view.enabled;
        }

        // ... update logic ...

        // Render
        city.render_environment();
        city.render_traffic(all_lights_red);
        city.render_overlays(current_time, danger_mode);

        // Debug overlay
        debug_view.render(&city, dt);

        next_frame().await;
    }
}
```

**Benefits**:
- Visual debugging of simulation
- Easier to understand car behavior
- Can identify performance bottlenecks
- Helps diagnose traffic flow issues

**Files to modify**:
- Create `src/debug.rs`
- Update `src/main.rs` to toggle debug view

**Estimated effort**: 4-6 hours

---

### 13. Extract Magic Numbers to Constants

**Problem**: Some literal values remain in code instead of named constants

**Locations**:
- `car.rs:177` - `10.0` (lane tolerance for collision check)
- `car.rs:412` - `15.0` and `10.0` (intersection center thresholds)
- `rendering.rs:516` - `0.8` (display width multiplier)
- `rendering.rs:558` - `0.3` (screw offset multiplier)
- Various depth calculations and offsets

**Solution**: Add to constants module

**Update**: `src/constants.rs`

```rust
pub mod collision {
    /// Lane position tolerance for collision detection (pixels)
    pub const LANE_POSITION_TOLERANCE: f32 = 10.0;

    /// Major axis tolerance for intersection center detection (pixels)
    pub const INTERSECTION_CENTER_TOLERANCE_MAJOR: f32 = 15.0;

    /// Minor axis tolerance for intersection center detection (pixels)
    pub const INTERSECTION_CENTER_TOLERANCE_MINOR: f32 = 10.0;
}

pub mod led {
    // ... existing LED constants ...

    /// Display width as fraction of block width
    pub const DISPLAY_WIDTH_SCALE: f32 = 0.8;

    /// Screw offset from corner as fraction of frame thickness
    pub const SCREW_OFFSET_FACTOR: f32 = 0.3;

    /// Pole spacing as fraction of display width
    pub const POLE_SPACING_FACTOR: f32 = 0.25;
}

pub mod rendering {
    // ... existing rendering constants ...

    /// Depth effect size for poles (fraction of standard depth offset)
    pub const POLE_DEPTH_FACTOR: f32 = 0.6;
}
```

**Update affected files**:
- `src/car.rs` - replace magic numbers with constants
- `src/rendering.rs` - replace magic numbers with constants

**Benefits**:
- Self-documenting code
- Easier to adjust related values
- Reduces chance of typos
- Centralized configuration

**Files to modify**:
- `src/constants.rs` - add new constant modules
- `src/car.rs` - replace `10.0`, `15.0`, etc. with named constants
- `src/rendering.rs` - replace `0.8`, `0.3`, etc. with named constants

**Estimated effort**: 2-3 hours

---

## Implementation Roadmap

### Phase 1: Foundation (Week 1-2)
1. Split large modules (rendering, traffic_light)
2. Extract LED character patterns
3. Extract magic numbers to constants

**Goal**: Improve code organization and readability

### Phase 2: Type Safety (Week 3-4)
4. Implement newtype pattern for IDs and percentages
5. Add error handling with Result types
6. Update all code to use new types

**Goal**: Prevent bugs with compile-time checks

### Phase 3: Performance (Week 5-6)
7. Optimize car collision detection (eliminate clone)
8. Implement render caching for static elements
9. Profile and optimize hot paths

**Goal**: Improve frame rate and reduce CPU usage

### Phase 4: Features (Week 7-8)
10. Add configuration file support
11. Implement command pattern for input
12. Add debug visualization mode

**Goal**: Better developer experience and configurability

### Phase 5: Quality (Week 9-10)
13. Add comprehensive unit tests
14. Implement behavior system for cars
15. Document all public APIs

**Goal**: Increase code quality and maintainability

### Phase 6: Advanced (Long-term)
16. Consider ECS migration for better scalability
17. Add scripting support for scenarios
18. Implement recording/replay system

**Goal**: Enable complex simulations and testing

---

## Code Quality Metrics

| Metric                  | Current | Target  | Priority |
|-------------------------|---------|---------|----------|
| Largest file            | 766 LOC | <400    | High     |
| Module count            | 12      | ~20     | Medium   |
| Type safety             | Medium  | High    | High     |
| Performance (clone)     | Poor    | Good    | High     |
| Test coverage           | 0%      | >60%    | Low      |
| Error handling          | None    | Result  | Medium   |
| Magic numbers           | Some    | None    | Low      |
| Documentation           | Good    | Good    | ✓        |
| Configuration           | Hardcoded | File  | Medium   |

---

## Dependencies to Add

```toml
[dependencies]
macroquad = "0.4.14"

# Configuration
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# Error handling (optional but recommended)
thiserror = "1.0"

# ECS (if pursuing that path)
hecs = "0.10"  # or bevy_ecs = "0.13"

# Parallel processing (if needed later)
rayon = "1.8"

[dev-dependencies]
# Testing utilities
approx = "0.5"  # For floating-point comparisons in tests
```

---

## Testing Strategy

### Unit Tests
- Focus on `car.rs` behavior functions
- Test `traffic_light.rs` state transitions
- Test `intersection.rs` collision detection
- Test `models.rs` coordinate conversions

### Integration Tests
- Test full simulation loop
- Test car spawning and removal
- Test traffic light coordination
- Test window resize handling

### Performance Tests
- Benchmark car update loop with 100+ cars
- Profile rendering pipeline
- Test spatial grid efficiency

---

## Additional Resources

### Project Structure After Refactoring

```
frontend/
├── Cargo.toml
├── config.toml (NEW)
├── REFACTORING_SUGGESTIONS.md (this file)
├── src/
│   ├── main.rs
│   ├── lib.rs (NEW)
│   ├── error.rs (NEW)
│   ├── config.rs (NEW)
│   ├── commands.rs (NEW)
│   ├── debug.rs (NEW)
│   ├── models/
│   │   ├── mod.rs
│   │   ├── car.rs
│   │   ├── direction.rs
│   │   └── newtypes.rs (NEW)
│   ├── rendering/
│   │   ├── mod.rs
│   │   ├── road.rs
│   │   ├── vehicle.rs
│   │   ├── led_display.rs
│   │   ├── led_chars.rs (NEW)
│   │   └── environment.rs
│   ├── traffic/
│   │   ├── mod.rs
│   │   ├── light.rs
│   │   ├── control.rs
│   │   └── rendering.rs
│   ├── simulation/
│   │   ├── mod.rs
│   │   ├── behaviors.rs (NEW)
│   │   └── context.rs (NEW)
│   ├── spatial_grid.rs (NEW - optional)
│   ├── render_cache.rs (NEW)
│   ├── constants.rs
│   ├── city.rs
│   ├── car.rs
│   ├── intersection.rs
│   ├── road.rs
│   ├── block.rs
│   ├── spawner.rs
│   └── input.rs
└── tests/
    ├── integration_tests.rs (NEW)
    └── performance_tests.rs (NEW)
```

---

## Notes

- All estimates assume one developer working part-time
- Priority levels are suggestions - adjust based on your needs
- Some refactorings can be done incrementally
- Consider doing high-priority items first for maximum impact
- Keep the existing code working during refactoring (feature flags if needed)

---

## Contact & Feedback

These suggestions were generated based on analysis of your codebase on 2025-11-11. The code is already well-structured, and these improvements would enhance maintainability, performance, and developer experience.

For questions or to prioritize specific refactorings, please update this document with your decisions.
