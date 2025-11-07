use crate::models::Intersection;
use macroquad::prelude::*;

pub fn get_road_positions() -> (Vec<f32>, Vec<f32>) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let vertical_positions = vec![screen_width * 0.15, screen_width * 0.5, screen_width * 0.85];

    let horizontal_positions = vec![screen_height * 0.25, screen_height * 0.75];

    (vertical_positions, horizontal_positions)
}

pub fn generate_intersections() -> Vec<Intersection> {
    // Store positions as percentages (0.0 to 1.0)
    let vertical_percents = vec![0.15, 0.5, 0.85];
    let horizontal_percents = vec![0.25, 0.75];

    let mut intersections = Vec::new();
    let mut id = 0;

    for &x_percent in &vertical_percents {
        for &y_percent in &horizontal_percents {
            intersections.push(Intersection {
                x_percent,
                y_percent,
                id,
                time_offset: id as f32 * 1.0, // Stagger traffic lights
            });
            id += 1;
        }
    }

    intersections
}
