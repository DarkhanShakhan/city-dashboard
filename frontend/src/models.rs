use macroquad::prelude::*;

#[derive(Clone)]
pub struct Car {
    pub x_percent: f32, // x position as percentage of screen width (0.0 to 1.0)
    pub y_percent: f32, // y position as percentage of screen height (0.0 to 1.0)
    pub direction: Direction,
    pub color: Color,
    pub road_index: usize,            // which road the car is on
    pub next_turn: Option<Direction>, // planned turn at next intersection
    pub just_turned: bool,            // prevents turning multiple times at same intersection
    pub in_intersection: bool,        // true if car is currently inside an intersection
}

impl Car {
    // Get absolute x position in pixels
    pub fn x(&self) -> f32 {
        self.x_percent * screen_width()
    }

    // Get absolute y position in pixels
    pub fn y(&self) -> f32 {
        self.y_percent * screen_height()
    }

    // Set absolute x position
    pub fn set_x(&mut self, x: f32) {
        self.x_percent = x / screen_width();
    }

    // Set absolute y position
    pub fn set_y(&mut self, y: f32) {
        self.y_percent = y / screen_height();
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Down,
    Right,
    Up,
    Left,
}

#[derive(Clone)]
pub struct Intersection {
    pub x_percent: f32, // x position as percentage of screen width
    pub y_percent: f32, // y position as percentage of screen height
    pub id: usize,
    pub time_offset: f32, // For staggering traffic light cycles
}

impl Intersection {
    // Get absolute x position in pixels
    pub fn x(&self) -> f32 {
        self.x_percent * screen_width()
    }

    // Get absolute y position in pixels
    pub fn y(&self) -> f32 {
        self.y_percent * screen_height()
    }
}
