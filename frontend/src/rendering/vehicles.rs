//! Vehicle rendering - cars and related decorative elements

use crate::constants::{
    rendering::CAR_WINDOW_COLOR,
    vehicle::{CAR_HEIGHT, CAR_WIDTH},
    visual::DEPTH_OFFSET,
};
use crate::models::{Car, Direction};
use macroquad::prelude::*;

/// Draws a car with directional sprite and depth effect
///
/// Renders a colored car rectangle with:
/// - Orientation based on travel direction
/// - 2.5D depth edges (darker shading on right and bottom)
/// - Windshield window positioned based on direction
///
/// # Arguments
/// * `car` - The car to render
///
/// # Car Dimensions
/// - Width: 20px, Height: 35px (rotated based on direction)
/// - Window size: ~60% of car width, ~30% of car height
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
        car.color,
    );

    // Draw depth edge
    draw_rectangle(
        car_x - width / 2.0 + width,
        car_y - height / 2.0,
        DEPTH_OFFSET,
        height,
        Color::new(car.color.r * 0.5, car.color.g * 0.5, car.color.b * 0.5, 1.0),
    );
    draw_rectangle(
        car_x - width / 2.0,
        car_y - height / 2.0 + height,
        width,
        DEPTH_OFFSET,
        Color::new(car.color.r * 0.5, car.color.g * 0.5, car.color.b * 0.5, 1.0),
    );

    // Draw windows
    match car.direction {
        Direction::Down => {
            draw_rectangle(
                car_x - width / 3.0,
                car_y - height / 4.0,
                width * 0.6,
                height * 0.3,
                CAR_WINDOW_COLOR,
            );
        }
        Direction::Up => {
            draw_rectangle(
                car_x - width / 3.0,
                car_y - height / 6.0,
                width * 0.6,
                height * 0.3,
                CAR_WINDOW_COLOR,
            );
        }
        Direction::Right => {
            draw_rectangle(
                car_x - height / 6.0,
                car_y - width / 3.0,
                height * 0.3,
                width * 0.6,
                CAR_WINDOW_COLOR,
            );
        }
        Direction::Left => {
            draw_rectangle(
                car_x - height / 4.0,
                car_y - width / 3.0,
                height * 0.3,
                width * 0.6,
                CAR_WINDOW_COLOR,
            );
        }
    }
}

/// Placeholder for removed building/parking lot feature
///
/// This function was previously used to draw a guarded building with
/// parking lot, but that feature has been removed. Kept for compatibility
/// with the main rendering pipeline.
///
/// # Arguments
/// * `_time` - Unused simulation time
/// * `_cars` - Unused car list
pub fn draw_guarded_building(_time: f64, _cars: &[Car]) {
    // Function removed - no parking lot or buildings
}
