use macroquad::prelude::*;
use crate::models::{Car, Direction, Intersection};
use crate::car::{CAR_WIDTH, CAR_HEIGHT};

const ROAD_WIDTH: f32 = 60.0;
const GRASS_COLOR: Color = Color::new(0.13, 0.55, 0.13, 1.0); // Forest green grass color
const DEPTH_OFFSET: f32 = 5.0; // Small offset for depth effect
const LINE_COLOR: Color = Color::new(1.0, 1.0, 0.8, 1.0); // Yellow-white for road lines
const INTERSECTION_MARK_COLOR: Color = Color::new(1.0, 1.0, 1.0, 0.6); // Semi-transparent white

pub fn draw_road_lines() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Road positions
    let vertical_positions = [
        screen_width * 0.15,
        screen_width * 0.5,
        screen_width * 0.85,
    ];

    let horizontal_positions = [
        screen_height * 0.25,
        screen_height * 0.75,
    ];

    let line_width = 2.0;
    let dash_length = 15.0;
    let dash_gap = 10.0;

    // Draw vertical road center lines (dashed)
    for &x in &vertical_positions {
        let mut y = 0.0;
        while y < screen_height {
            draw_rectangle(
                x - line_width / 2.0,
                y,
                line_width,
                dash_length,
                LINE_COLOR
            );
            y += dash_length + dash_gap;
        }
    }

    // Draw horizontal road center lines (dashed)
    for &y in &horizontal_positions {
        let mut x = 0.0;
        while x < screen_width {
            draw_rectangle(
                x,
                y - line_width / 2.0,
                dash_length,
                line_width,
                LINE_COLOR
            );
            x += dash_length + dash_gap;
        }
    }
}

pub fn draw_intersection_markings(intersections: &[Intersection]) {
    let intersection_size = 40.0; // Size of intersection box
    let crosswalk_width = 8.0;
    let stripe_width = 4.0;
    let stripe_gap = 2.0;

    for intersection in intersections {
        let int_x = intersection.x();
        let int_y = intersection.y();

        // Draw intersection box outline
        let box_size = intersection_size * 2.0;
        draw_rectangle_lines(
            int_x - box_size / 2.0,
            int_y - box_size / 2.0,
            box_size,
            box_size,
            1.0,
            Color::new(1.0, 1.0, 1.0, 0.3)
        );

        // Draw crosswalks (zebra stripes) on all 4 sides
        let crosswalk_distance = intersection_size + 5.0;

        // Top crosswalk (horizontal stripes)
        let top_y = int_y - crosswalk_distance;
        let mut stripe_x = int_x - ROAD_WIDTH / 2.0;
        while stripe_x < int_x + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                stripe_x,
                top_y - crosswalk_width / 2.0,
                stripe_width,
                crosswalk_width,
                INTERSECTION_MARK_COLOR
            );
            stripe_x += stripe_width + stripe_gap;
        }

        // Bottom crosswalk (horizontal stripes)
        let bottom_y = int_y + crosswalk_distance;
        stripe_x = int_x - ROAD_WIDTH / 2.0;
        while stripe_x < int_x + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                stripe_x,
                bottom_y - crosswalk_width / 2.0,
                stripe_width,
                crosswalk_width,
                INTERSECTION_MARK_COLOR
            );
            stripe_x += stripe_width + stripe_gap;
        }

        // Left crosswalk (vertical stripes)
        let left_x = int_x - crosswalk_distance;
        let mut stripe_y = int_y - ROAD_WIDTH / 2.0;
        while stripe_y < int_y + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                left_x - crosswalk_width / 2.0,
                stripe_y,
                crosswalk_width,
                stripe_width,
                INTERSECTION_MARK_COLOR
            );
            stripe_y += stripe_width + stripe_gap;
        }

        // Right crosswalk (vertical stripes)
        let right_x = int_x + crosswalk_distance;
        stripe_y = int_y - ROAD_WIDTH / 2.0;
        while stripe_y < int_y + ROAD_WIDTH / 2.0 {
            draw_rectangle(
                right_x - crosswalk_width / 2.0,
                stripe_y,
                crosswalk_width,
                stripe_width,
                INTERSECTION_MARK_COLOR
            );
            stripe_y += stripe_width + stripe_gap;
        }
    }
}

pub fn draw_grass_blocks() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // 3 vertical roads (outer roads closer to edges)
    let vertical_positions = [
        screen_width * 0.15,
        screen_width * 0.5,
        screen_width * 0.85,
    ];

    // 2 horizontal roads (closer to top and bottom edges)
    let horizontal_positions = [
        screen_height * 0.25,
        screen_height * 0.75,
    ];

    // Calculate grass block boundaries
    // We need to draw grass blocks in the spaces between roads
    let x_boundaries = [
        0.0,
        vertical_positions[0] - ROAD_WIDTH / 2.0,
        vertical_positions[0] + ROAD_WIDTH / 2.0,
        vertical_positions[1] - ROAD_WIDTH / 2.0,
        vertical_positions[1] + ROAD_WIDTH / 2.0,
        vertical_positions[2] - ROAD_WIDTH / 2.0,
        vertical_positions[2] + ROAD_WIDTH / 2.0,
        screen_width,
    ];

    let y_boundaries = [
        0.0,
        horizontal_positions[0] - ROAD_WIDTH / 2.0,
        horizontal_positions[0] + ROAD_WIDTH / 2.0,
        horizontal_positions[1] - ROAD_WIDTH / 2.0,
        horizontal_positions[1] + ROAD_WIDTH / 2.0,
        screen_height,
    ];

    // Draw grass blocks in a grid (skip road areas)
    for i in (0..x_boundaries.len() - 1).step_by(2) {
        for j in (0..y_boundaries.len() - 1).step_by(2) {
            let x = x_boundaries[i];
            let y = y_boundaries[j];
            let width = x_boundaries[i + 1] - x;
            let height = y_boundaries[j + 1] - y;

            // Draw grass block
            draw_rectangle(x, y, width, height, GRASS_COLOR);

            // Add depth edge for 2.5D effect
            draw_rectangle(
                x + width,
                y,
                DEPTH_OFFSET,
                height,
                Color::new(0.1, 0.45, 0.1, 1.0) // Darker grass for depth
            );
            draw_rectangle(
                x,
                y + height,
                width + DEPTH_OFFSET,
                DEPTH_OFFSET,
                Color::new(0.1, 0.45, 0.1, 1.0)
            );
        }
    }
}

pub fn draw_car(car: &Car) {
    let car_x = car.x();
    let car_y = car.y();

    let (width, height) = match car.direction {
        Direction::Down | Direction::Up => (CAR_WIDTH, CAR_HEIGHT),
        Direction::Left | Direction::Right => (CAR_HEIGHT, CAR_WIDTH),
    };

    // Draw car body
    draw_rectangle(
        car_x - width / 2.0,
        car_y - height / 2.0,
        width,
        height,
        car.color
    );

    // Draw depth edge
    draw_rectangle(
        car_x - width / 2.0 + width,
        car_y - height / 2.0,
        DEPTH_OFFSET,
        height,
        Color::new(car.color.r * 0.5, car.color.g * 0.5, car.color.b * 0.5, 1.0)
    );
    draw_rectangle(
        car_x - width / 2.0,
        car_y - height / 2.0 + height,
        width,
        DEPTH_OFFSET,
        Color::new(car.color.r * 0.5, car.color.g * 0.5, car.color.b * 0.5, 1.0)
    );

    // Draw windows
    let window_color = Color::new(0.6, 0.8, 1.0, 1.0);
    match car.direction {
        Direction::Down => {
            draw_rectangle(
                car_x - width / 3.0,
                car_y - height / 4.0,
                width * 0.6,
                height * 0.3,
                window_color
            );
        }
        Direction::Up => {
            draw_rectangle(
                car_x - width / 3.0,
                car_y - height / 6.0,
                width * 0.6,
                height * 0.3,
                window_color
            );
        }
        Direction::Right => {
            draw_rectangle(
                car_x - height / 6.0,
                car_y - width / 3.0,
                height * 0.3,
                width * 0.6,
                window_color
            );
        }
        Direction::Left => {
            draw_rectangle(
                car_x - height / 4.0,
                car_y - width / 3.0,
                height * 0.3,
                width * 0.6,
                window_color
            );
        }
    }
}
