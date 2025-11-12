//! City structure and management
//!
//! This module defines the City structure that contains all city elements:
//! - Roads: The road network
//! - Blocks: Areas between roads that hold objects
//! - Intersections: Road crossings with traffic lights
//! - Cars: Vehicles moving through the city
//!
//! The City acts as the main container and coordinator for all city elements.

use crate::block::Block;
use crate::constants::visual::ROAD_WIDTH;
use crate::intersection::Intersection;
use crate::models::Car;
use crate::road::Road;
use crate::spawner::CarSpawner;
use std::collections::HashMap;

// ============================================================================
// City Model
// ============================================================================

/// Represents the entire city with its infrastructure
///
/// The City contains and manages all city elements including the road network,
/// city blocks, intersections, and cars. Uses HashMap storage for efficient
/// lookups by ID.
pub struct City {
    /// Road network indexed by road ID
    pub roads: HashMap<usize, Road>,

    /// City blocks indexed by block ID
    pub blocks: HashMap<usize, Block>,

    /// Intersections indexed by intersection ID
    pub intersections: HashMap<usize, Intersection>,

    /// All cars in the city (centralized storage)
    pub cars: Vec<Car>,

    /// Car spawner that manages spawning new cars at regular intervals
    car_spawner: CarSpawner,
}

impl City {
    /// Creates a new empty city
    ///
    /// # Returns
    /// A new City instance with no roads, blocks, intersections, or cars
    pub fn new() -> Self {
        use crate::constants::vehicle::CAR_SPAWN_INTERVAL;

        Self {
            roads: HashMap::new(),
            blocks: HashMap::new(),
            intersections: HashMap::new(),
            cars: Vec::new(),
            car_spawner: CarSpawner::new(CAR_SPAWN_INTERVAL),
        }
    }

    /// Creates a new city using the builder pattern
    ///
    /// # Example
    /// ```
    /// let city = City::builder()
    ///     .add_road(road1)
    ///     .add_road(road2)
    ///     .add_block(block1)
    ///     .build();
    /// ```
    pub fn builder() -> CityBuilder {
        CityBuilder::new()
    }

    /// Adds a road to the city
    ///
    /// # Arguments
    /// * `road` - The road to add
    pub fn add_road(&mut self, road: Road) {
        self.roads.insert(road.index, road);
    }

    /// Adds a block to the city
    ///
    /// # Arguments
    /// * `block` - The block to add
    pub fn add_block(&mut self, block: Block) {
        self.blocks.insert(block.id, block);
    }

    /// Adds an intersection to the city
    ///
    /// # Arguments
    /// * `intersection` - The intersection to add
    pub fn add_intersection(&mut self, intersection: Intersection) {
        self.intersections.insert(intersection.id, intersection);
    }

    /// Adds a car to the city
    ///
    /// # Arguments
    /// * `car` - The car to add
    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car);
    }

    /// Returns the number of roads in the city
    pub fn road_count(&self) -> usize {
        self.roads.len()
    }

    /// Returns the number of blocks in the city
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Returns the number of intersections in the city
    pub fn intersection_count(&self) -> usize {
        self.intersections.len()
    }

    /// Returns the number of cars in the city
    pub fn car_count(&self) -> usize {
        self.cars.len()
    }

    /// Renders all blocks in the city
    ///
    /// This will render all objects contained in each block.
    ///
    /// # Arguments
    /// * `context` - Rendering context with global state
    pub fn render_blocks(&self, context: &crate::block::RenderContext) {
        for block in self.blocks.values() {
            block.render(context);
        }
    }

    /// Gets a reference to a block by its ID
    ///
    /// # Arguments
    /// * `id` - The block ID to search for
    ///
    /// # Returns
    /// Optional reference to the block if found
    pub fn get_block(&self, id: usize) -> Option<&Block> {
        self.blocks.get(&id)
    }

    /// Gets a mutable reference to a block by its ID
    ///
    /// # Arguments
    /// * `id` - The block ID to search for
    ///
    /// # Returns
    /// Optional mutable reference to the block if found
    pub fn get_block_mut(&mut self, id: usize) -> Option<&mut Block> {
        self.blocks.get_mut(&id)
    }

    /// Gets a reference to a road by its ID
    ///
    /// # Arguments
    /// * `id` - The road ID to search for
    ///
    /// # Returns
    /// Optional reference to the road if found
    pub fn get_road(&self, id: usize) -> Option<&Road> {
        self.roads.get(&id)
    }

    /// Gets a mutable reference to a road by its ID
    ///
    /// # Arguments
    /// * `id` - The road ID to search for
    ///
    /// # Returns
    /// Optional mutable reference to the road if found
    pub fn get_road_mut(&mut self, id: usize) -> Option<&mut Road> {
        self.roads.get_mut(&id)
    }

    /// Gets a reference to an intersection by its ID
    ///
    /// # Arguments
    /// * `id` - The intersection ID to search for
    ///
    /// # Returns
    /// Optional reference to the intersection if found
    pub fn get_intersection(&self, id: usize) -> Option<&Intersection> {
        self.intersections.get(&id)
    }

    /// Gets a mutable reference to an intersection by its ID
    ///
    /// # Arguments
    /// * `id` - The intersection ID to search for
    ///
    /// # Returns
    /// Optional mutable reference to the intersection if found
    pub fn get_intersection_mut(&mut self, id: usize) -> Option<&mut Intersection> {
        self.intersections.get_mut(&id)
    }

    /// Clears all roads from the city
    pub fn clear_roads(&mut self) {
        self.roads.clear();
    }

    /// Clears all blocks from the city
    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    /// Clears all intersections from the city
    pub fn clear_intersections(&mut self) {
        self.intersections.clear();
    }

    /// Clears all cars from the city
    pub fn clear_cars(&mut self) {
        self.cars.clear();
    }

    /// Clears all elements from the city
    pub fn clear(&mut self) {
        self.roads.clear();
        self.blocks.clear();
        self.intersections.clear();
        self.cars.clear();
    }

    // ========================================================================
    // Car Transition Helpers
    // ========================================================================

    /// Finds which road a point is on, if any
    ///
    /// # Arguments
    /// * `x` - X coordinate in pixels
    /// * `y` - Y coordinate in pixels
    ///
    /// # Returns
    /// Optional road ID if the point is on a road
    pub fn find_road_at_position(&self, x: f32, y: f32) -> Option<usize> {
        let half_road = ROAD_WIDTH / 2.0;

        for road in self.roads.values() {
            match road.orientation {
                crate::road::Orientation::Vertical => {
                    let road_x = road.position_percent * macroquad::prelude::screen_width();
                    if (x - road_x).abs() <= half_road {
                        return Some(road.index);
                    }
                }
                crate::road::Orientation::Horizontal => {
                    let road_y = road.position_percent * macroquad::prelude::screen_height();
                    if (y - road_y).abs() <= half_road {
                        return Some(road.index);
                    }
                }
            }
        }
        None
    }

    /// Finds which intersection a point is in, if any
    ///
    /// # Arguments
    /// * `x` - X coordinate in pixels
    /// * `y` - Y coordinate in pixels
    ///
    /// # Returns
    /// Optional intersection ID if the point is inside an intersection
    pub fn find_intersection_at_position(&self, x: f32, y: f32) -> Option<usize> {
        for intersection in self.intersections.values() {
            if intersection.contains_point(x, y) {
                return Some(intersection.id);
            }
        }
        None
    }

    /// Finds which block a point is in, if any
    ///
    /// # Arguments
    /// * `x` - X coordinate in pixels
    /// * `y` - Y coordinate in pixels
    ///
    /// # Returns
    /// Optional block ID if the point is inside a block
    pub fn find_block_at_position(&self, x: f32, y: f32) -> Option<usize> {
        for block in self.blocks.values() {
            if block.contains_point(x, y) {
                return Some(block.id);
            }
        }
        None
    }

    // ========================================================================
    // Rendering Methods
    // ========================================================================

    /// Renders static environment elements (grass, roads, intersections)
    ///
    /// Draws the background environment including:
    /// - Grass blocks with 2.5D depth effect (via Block rendering)
    /// - Road center lines (dashed)
    /// - Intersection markings and crosswalks
    ///
    /// This should be called first in the rendering pipeline as it draws
    /// the background layer.
    pub fn render_environment(&self) {
        use crate::block::RenderContext;
        use crate::rendering::{draw_intersection_markings, draw_road_lines};

        // Render grass blocks (static, so time and danger_mode don't matter)
        let context = RenderContext::new(0.0, false);
        for block in self.blocks.values() {
            // Only render blocks with grass (not LED display block)
            if block.id != 0 {
                block.render(&context);
            }
        }

        draw_road_lines();

        // Convert HashMap values to Vec for rendering
        let intersections: Vec<_> = self.intersections.values().cloned().collect();
        draw_intersection_markings(&intersections);
    }

    /// Renders dynamic traffic elements (cars and traffic lights)
    ///
    /// Draws moving and interactive elements:
    /// - Traffic lights at all intersections
    /// - All cars with directional sprites
    ///
    /// Cars are drawn first (background), then traffic lights (foreground).
    ///
    /// # Arguments
    /// * `all_lights_red` - If true, forces all traffic lights to red (emergency mode)
    pub fn render_traffic(&self, all_lights_red: bool) {
        use crate::rendering::draw_car;
        use crate::traffic_light::draw_traffic_lights;

        // Convert HashMap values to Vec for rendering
        let intersections: Vec<_> = self.intersections.values().cloned().collect();

        // Draw all cars first (behind traffic lights)
        for car in &self.cars {
            draw_car(car);
        }

        // Draw traffic lights on top
        draw_traffic_lights(&intersections, all_lights_red);
    }

    /// Renders UI overlays and decorative elements
    ///
    /// Draws overlay elements that appear on top of the environment and traffic:
    /// - LED display with scrolling text or danger warning
    /// - Decorative elements (currently empty but kept for future use)
    ///
    /// This should be called last in the rendering pipeline as it draws
    /// the foreground/UI layer.
    ///
    /// # Arguments
    /// * `time` - Current simulation time for animations
    /// * `danger_mode` - If true, shows "DANGER" on LED display in red
    pub fn render_overlays(&self, time: f64, danger_mode: bool) {
        use crate::block::RenderContext;
        use crate::rendering::draw_guarded_building;

        // Note: draw_guarded_building is currently empty but kept for future use
        draw_guarded_building(time, &self.cars);

        // Create render context with current state
        let context = RenderContext::new(time, danger_mode);

        // Render only LED display blocks (id 0)
        // Grass blocks are rendered in render_environment
        for block in self.blocks.values() {
            if block.id == 0 {
                block.render(&context);
            }
        }
    }

    // ========================================================================
    // Simulation Update Methods
    // ========================================================================

    /// Spawns new cars at regular intervals
    ///
    /// Uses the internal car spawner to add new cars to the city at
    /// configured intervals. Cars spawn at random road edges with random
    /// properties (color, direction, planned turns).
    pub fn spawn_cars(&mut self) {
        self.car_spawner.try_spawn(&mut self.cars);
    }

    /// Updates all traffic lights for one frame
    ///
    /// Cycles through all intersections and updates their traffic light states
    /// based on the configured durations (green, yellow, red).
    ///
    /// # Arguments
    /// * `dt` - Delta time (frame duration in seconds)
    pub fn update_traffic_lights(&mut self, dt: f32) {
        for intersection in self.intersections.values_mut() {
            intersection.update_lights(dt);
        }
    }

    /// Updates all cars' positions and behaviors for one frame
    ///
    /// This is the main simulation loop that handles:
    /// - Traffic light compliance
    /// - Collision avoidance
    /// - Intersection navigation and turning
    /// - Car removal when off-screen
    ///
    /// # Arguments
    /// * `dt` - Delta time (frame duration in seconds)
    /// * `all_lights_red` - Emergency mode flag (stops all traffic)
    pub fn update_cars(&mut self, dt: f32, all_lights_red: bool) {
        use crate::car::update_cars;

        // Convert HashMap to Vec for the car update function
        let intersections: Vec<_> = self.intersections.values().cloned().collect();

        // Update all cars using the car module's update function
        update_cars(&mut self.cars, &intersections, dt, all_lights_red);
    }

    /// Updates the entire city simulation for one frame
    ///
    /// This is the main update method that orchestrates all simulation updates:
    /// 1. Spawns new cars at regular intervals
    /// 2. Updates all traffic light states
    /// 3. Updates all car positions and behaviors
    ///
    /// This method provides a unified interface for updating the entire city
    /// simulation in a single call.
    ///
    /// # Arguments
    /// * `dt` - Delta time (frame duration in seconds)
    /// * `all_lights_red` - Emergency mode flag (stops all traffic)
    ///
    /// # Example
    /// ```
    /// city.update(dt, false); // Normal operation
    /// city.update(dt, true);  // Emergency mode - all lights red
    /// ```
    pub fn update(&mut self, dt: f32, all_lights_red: bool) {
        self.spawn_cars();
        self.update_traffic_lights(dt);
        self.update_cars(dt, all_lights_red);
    }
}

impl Default for City {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// City Builder
// ============================================================================

/// Builder for creating City instances with a fluent API
///
/// Provides a convenient way to construct cities with roads, blocks, and intersections.
///
/// # Example
/// ```
/// let city = City::builder()
///     .add_road(road1)
///     .add_road(road2)
///     .add_block(block1)
///     .add_intersection(intersection1)
///     .build();
/// ```
pub struct CityBuilder {
    roads: HashMap<usize, Road>,
    blocks: HashMap<usize, Block>,
    intersections: HashMap<usize, Intersection>,
    cars: Vec<Car>,
}

impl CityBuilder {
    /// Creates a new CityBuilder
    fn new() -> Self {
        Self {
            roads: HashMap::new(),
            blocks: HashMap::new(),
            intersections: HashMap::new(),
            cars: Vec::new(),
        }
    }

    /// Adds a road to the city being built
    pub fn add_road(mut self, road: Road) -> Self {
        self.roads.insert(road.index, road);
        self
    }

    /// Adds multiple roads to the city being built
    pub fn add_roads(mut self, roads: Vec<Road>) -> Self {
        for road in roads {
            self.roads.insert(road.index, road);
        }
        self
    }

    /// Adds a block to the city being built
    pub fn add_block(mut self, block: Block) -> Self {
        self.blocks.insert(block.id, block);
        self
    }

    /// Adds multiple blocks to the city being built
    pub fn add_blocks(mut self, blocks: Vec<Block>) -> Self {
        for block in blocks {
            self.blocks.insert(block.id, block);
        }
        self
    }

    /// Adds an intersection to the city being built
    pub fn add_intersection(mut self, intersection: Intersection) -> Self {
        self.intersections.insert(intersection.id, intersection);
        self
    }

    /// Adds multiple intersections to the city being built
    pub fn add_intersections(mut self, intersections: Vec<Intersection>) -> Self {
        for intersection in intersections {
            self.intersections.insert(intersection.id, intersection);
        }
        self
    }

    /// Adds a car to the city being built
    pub fn add_car(mut self, car: Car) -> Self {
        self.cars.push(car);
        self
    }

    /// Adds multiple cars to the city being built
    pub fn add_cars(mut self, cars: Vec<Car>) -> Self {
        self.cars.extend(cars);
        self
    }

    /// Builds the City instance
    ///
    /// # Returns
    /// A new City instance with all added roads, blocks, intersections, and cars
    pub fn build(self) -> City {
        use crate::constants::vehicle::CAR_SPAWN_INTERVAL;

        City {
            roads: self.roads,
            blocks: self.blocks,
            intersections: self.intersections,
            cars: self.cars,
            car_spawner: CarSpawner::new(CAR_SPAWN_INTERVAL),
        }
    }
}
