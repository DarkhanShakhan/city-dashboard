//! Car behavior and traffic simulation logic
//!
//! This module handles:
//! - Car movement and physics
//! - Traffic light compliance
//! - Collision avoidance
//! - Intersection navigation and turning
//!
//! Cars follow left-hand traffic rules with proper lane discipline.

use crate::constants::vehicle::*;
use crate::constants::visual::ROAD_WIDTH;
use crate::intersection::Intersection;
use crate::models::{Car, Direction};
use macroquad::prelude::*;

// ============================================================================
// Traffic Control & Collision Detection
// ============================================================================

/// Checks if a car should stop for a traffic light at an intersection
///
/// # Arguments
/// * `car` - The car to check
/// * `intersection_x` - X position of intersection center (pixels)
/// * `intersection_y` - Y position of intersection center (pixels)
/// * `light_state` - Traffic light state (0=red, 1=yellow, 2=green)
///
/// # Returns
/// `true` if car should stop, `false` if it can proceed
///
/// # Safety Rules
/// - Cars already in intersection MUST continue (never stop mid-crossing)
/// - Stop only if 30-80 pixels from intersection
/// - Stop on red or yellow lights only
fn check_traffic_light_at_intersection(
    car: &Car,
    intersection_x: f32,
    intersection_y: f32,
    light_state: u8,
) -> bool {
    // CRITICAL: Never stop a car that's already in the intersection
    if car.in_intersection {
        return false; // Cars in intersection must continue through
    }

    let stop_distance_min = STOP_DISTANCE_MIN;
    let stop_distance_max = STOP_DISTANCE_MAX;
    let lane_tolerance = LANE_TOLERANCE;

    let car_x = car.x();
    let car_y = car.y();

    match car.direction {
        Direction::Down => {
            if (car_x - intersection_x).abs() < lane_tolerance && intersection_y > car_y {
                let distance = intersection_y - car_y;
                // Only stop if far enough away and light is red/yellow
                // If too close (< stop_distance_min), continue through
                if distance > stop_distance_min && distance < stop_distance_max {
                    return light_state == 0 || light_state == 1; // Stop on red or yellow
                }
            }
        }
        Direction::Up => {
            if (car_x - intersection_x).abs() < lane_tolerance && intersection_y < car_y {
                let distance = car_y - intersection_y;
                if distance > stop_distance_min && distance < stop_distance_max {
                    return light_state == 0 || light_state == 1;
                }
            }
        }
        Direction::Right => {
            if (car_y - intersection_y).abs() < lane_tolerance && intersection_x > car_x {
                let distance = intersection_x - car_x;
                if distance > stop_distance_min && distance < stop_distance_max {
                    return light_state == 0 || light_state == 1;
                }
            }
        }
        Direction::Left => {
            if (car_y - intersection_y).abs() < lane_tolerance && intersection_x < car_x {
                let distance = car_x - intersection_x;
                if distance > stop_distance_min && distance < stop_distance_max {
                    return light_state == 0 || light_state == 1;
                }
            }
        }
    }
    false
}

/// Checks if another car is currently occupying an intersection
///
/// Prevents multiple cars from entering the same intersection simultaneously,
/// which would cause gridlock or collisions.
///
/// # Arguments
/// * `car` - The car checking to enter
/// * `intersection_x` - X position of intersection center
/// * `intersection_y` - Y position of intersection center
/// * `other_cars` - All other cars in the simulation
///
/// # Returns
/// `true` if intersection is occupied by another car
fn check_intersection_occupied(
    car: &Car,
    intersection_x: f32,
    intersection_y: f32,
    other_cars: &[Car],
) -> bool {
    // Check if another car is already in this intersection
    let intersection_radius = INTERSECTION_RADIUS;

    for other in other_cars {
        // Skip self
        if std::ptr::eq(car as *const Car, other as *const Car) {
            continue;
        }

        // Check if other car is in this intersection
        let other_x = other.x();
        let other_y = other.y();
        let dist_to_intersection =
            ((other_x - intersection_x).powi(2) + (other_y - intersection_y).powi(2)).sqrt();

        if dist_to_intersection < intersection_radius {
            return true; // Intersection is occupied
        }
    }

    false
}

/// Checks if car is too close to another vehicle (collision avoidance)
///
/// Implements basic following distance and prevents rear-end collisions.
/// Cars maintain a 50-pixel safe following distance.
///
/// # Arguments
/// * `car` - The car to check
/// * `other_cars` - All other cars to check against
///
/// # Returns
/// `true` if car should stop to avoid collision
fn check_car_collision(car: &Car, other_cars: &[Car]) -> bool {
    // Don't stop if car is in intersection - must complete crossing
    if car.in_intersection {
        return false;
    }

    // Minimum safe following distance in pixels
    let safe_distance = SAFE_FOLLOWING_DISTANCE;

    let car_x = car.x();
    let car_y = car.y();

    for other in other_cars {
        // Skip self comparison
        if std::ptr::eq(car as *const Car, other as *const Car) {
            continue;
        }

        // Skip collision check if the other car is also in an intersection
        // (they're in different intersections or will handle it themselves)
        if other.in_intersection {
            continue;
        }

        let other_x = other.x();
        let other_y = other.y();

        // Check cars going in the same direction on the same road
        if car.direction == other.direction {
            let distance = match car.direction {
                Direction::Down => {
                    if (car_x - other_x).abs() < 10.0 {
                        other_y - car_y // Distance to car ahead
                    } else {
                        f32::MAX
                    }
                }
                Direction::Up => {
                    if (car_x - other_x).abs() < 10.0 {
                        car_y - other_y // Distance to car ahead
                    } else {
                        f32::MAX
                    }
                }
                Direction::Right => {
                    if (car_y - other_y).abs() < 10.0 {
                        other_x - car_x // Distance to car ahead
                    } else {
                        f32::MAX
                    }
                }
                Direction::Left => {
                    if (car_y - other_y).abs() < 10.0 {
                        car_x - other_x // Distance to car ahead
                    } else {
                        f32::MAX
                    }
                }
            };

            if distance > 0.0 && distance < safe_distance {
                return true; // Too close to another car
            }
        }

        // Check cars going in opposite directions (avoid head-on collisions)
        let is_opposite = match car.direction {
            Direction::Down => other.direction == Direction::Up,
            Direction::Up => other.direction == Direction::Down,
            Direction::Right => other.direction == Direction::Left,
            Direction::Left => other.direction == Direction::Right,
        };

        if is_opposite {
            // Check if cars are on the same road and close to each other
            let (on_same_road, distance) = match car.direction {
                Direction::Down | Direction::Up => {
                    // Check if on same vertical road
                    let on_same = (car_x - other_x).abs() < ROAD_WIDTH / 2.0;
                    let dist = (car_y - other_y).abs();
                    (on_same, dist)
                }
                Direction::Right | Direction::Left => {
                    // Check if on same horizontal road
                    let on_same = (car_y - other_y).abs() < ROAD_WIDTH / 2.0;
                    let dist = (car_x - other_x).abs();
                    (on_same, dist)
                }
            };

            if on_same_road && distance < safe_distance {
                // Cars need to stay on their side of the road
                // Shift to the right side of the road (relative to direction)
                return false; // Don't stop, but we'll handle lane separation differently
            }
        }
    }

    false
}

// ============================================================================
// Car Movement Helpers
// ============================================================================

/// Plans the next turn for a car based on current direction
///
/// Randomly decides whether to turn at the next intersection and which
/// direction to turn based on the TURN_PROBABILITY constant.
///
/// # Arguments
/// * `current_direction` - The car's current direction of travel
///
/// # Returns
/// `Some(Direction)` if car should turn, `None` if car should go straight
fn plan_next_turn(current_direction: Direction) -> Option<Direction> {
    if rand::gen_range(0.0, 1.0) < TURN_PROBABILITY {
        match current_direction {
            Direction::Down | Direction::Up => {
                if rand::gen_range(0, 2) == 0 {
                    Some(Direction::Right)
                } else {
                    Some(Direction::Left)
                }
            }
            Direction::Right | Direction::Left => {
                if rand::gen_range(0, 2) == 0 {
                    Some(Direction::Down)
                } else {
                    Some(Direction::Up)
                }
            }
        }
    } else {
        None
    }
}

/// Handles car turning at intersection center
///
/// Executes the planned turn when the car reaches the intersection center,
/// adjusts the car's position to the correct lane for the new direction,
/// and plans the next turn.
///
/// # Arguments
/// * `car` - The car to potentially turn
/// * `intersection` - The intersection where turning might occur
/// * `at_intersection_center` - Whether the car is at the intersection center
///
/// # Returns
/// `true` if a turn was executed, `false` otherwise
fn handle_car_turn(car: &mut Car, intersection: &Intersection, at_intersection_center: bool) -> bool {
    if at_intersection_center && car.next_turn.is_some() && !car.just_turned {
        // Execute the turn
        let new_direction = car.next_turn.unwrap();
        car.direction = new_direction;

        // Adjust position to new lane (left-hand traffic)
        match new_direction {
            Direction::Down => {
                car.x_percent = intersection.x_percent - (LANE_OFFSET / screen_width());
                car.y_percent = intersection.y_percent;
            }
            Direction::Up => {
                car.x_percent = intersection.x_percent + (LANE_OFFSET / screen_width());
                car.y_percent = intersection.y_percent;
            }
            Direction::Right => {
                car.x_percent = intersection.x_percent;
                car.y_percent = intersection.y_percent + (LANE_OFFSET / screen_height());
            }
            Direction::Left => {
                car.x_percent = intersection.x_percent;
                car.y_percent = intersection.y_percent - (LANE_OFFSET / screen_height());
            }
        }

        // Plan next turn
        car.next_turn = plan_next_turn(new_direction);

        // Mark that we just turned
        car.just_turned = true;
        true
    } else {
        false
    }
}

/// Moves the car based on its direction and speed
///
/// Updates the car's position based on its current direction of travel
/// and the frame delta time. Movement is calculated as percentage of
/// screen dimensions for responsive scaling.
///
/// # Arguments
/// * `car` - The car to move
/// * `dt` - Delta time (frame duration in seconds)
fn move_car(car: &mut Car, dt: f32) {
    match car.direction {
        Direction::Down => {
            let speed_percent = CAR_SPEED * dt / screen_height();
            car.y_percent += speed_percent;
        }
        Direction::Up => {
            let speed_percent = CAR_SPEED * dt / screen_height();
            car.y_percent -= speed_percent;
        }
        Direction::Right => {
            let speed_percent = CAR_SPEED * dt / screen_width();
            car.x_percent += speed_percent;
        }
        Direction::Left => {
            let speed_percent = CAR_SPEED * dt / screen_width();
            car.x_percent -= speed_percent;
        }
    }
}

/// Checks if a car is still on screen
///
/// Cars are kept slightly off-screen (0.1 buffer) to allow smooth
/// spawning and despawning at screen edges.
///
/// # Arguments
/// * `car` - The car to check
///
/// # Returns
/// `true` if car is on or near screen, `false` if far off-screen
fn is_car_on_screen(car: &Car) -> bool {
    car.x_percent > -0.1 && car.x_percent < 1.1 && car.y_percent > -0.1 && car.y_percent < 1.1
}

/// Updates car state at intersections and handles turning
///
/// Checks all intersections to:
/// - Update car's intersection state (in_intersection flag)
/// - Check if car is approaching intersection center
/// - Handle turning if at intersection center
///
/// # Arguments
/// * `car` - The car to update
/// * `intersections` - All intersections in the simulation
///
/// # Returns
/// Tuple of (at_any_intersection, turned_at_intersection)
fn update_car_at_intersection(car: &mut Car, intersections: &[Intersection]) -> (bool, bool) {
    let mut at_any_intersection = false;
    let car_x = car.x();
    let car_y = car.y();

    for intersection in intersections {
        let int_x = intersection.x();
        let int_y = intersection.y();

        // Check if car is at this intersection
        let intersection_radius = INTERSECTION_RADIUS;
        let dist_to_intersection = ((car_x - int_x).powi(2) + (car_y - int_y).powi(2)).sqrt();
        let at_intersection = dist_to_intersection < intersection_radius;

        if at_intersection {
            at_any_intersection = true;
            car.in_intersection = true;
        }

        // Check for turning at intersection center
        let at_intersection_center = match car.direction {
            Direction::Down => (car_x - int_x).abs() < 15.0 && (car_y - int_y).abs() < 10.0,
            Direction::Up => (car_x - int_x).abs() < 15.0 && (car_y - int_y).abs() < 10.0,
            Direction::Right => (car_y - int_y).abs() < 15.0 && (car_x - int_x).abs() < 10.0,
            Direction::Left => (car_y - int_y).abs() < 15.0 && (car_x - int_x).abs() < 10.0,
        };

        if handle_car_turn(car, intersection, at_intersection_center) {
            return (at_any_intersection, true); // Turned at this intersection
        }
    }

    (at_any_intersection, false)
}

/// Determines if a car should stop based on all conditions
///
/// Checks multiple stop conditions:
/// - Traffic lights at intersections
/// - Occupied intersections (prevent gridlock)
/// - Collision avoidance with other cars
///
/// # Arguments
/// * `car` - The car to check
/// * `intersections` - All intersections with traffic lights
/// * `other_cars` - All other cars for collision checking
/// * `all_lights_red` - Emergency mode (all lights red)
///
/// # Returns
/// `true` if car should stop, `false` if car can proceed
fn should_car_stop(
    car: &Car,
    intersections: &[Intersection],
    other_cars: &[Car],
    all_lights_red: bool,
) -> bool {
    let car_x = car.x();
    let car_y = car.y();

    // Check all intersections for stop conditions
    for intersection in intersections {
        let int_x = intersection.x();
        let int_y = intersection.y();

        // Get traffic light state
        let light_state = if all_lights_red {
            0 // All lights red
        } else {
            intersection.get_light_state_for_direction(car.direction)
        };

        // Check if we should stop for traffic light
        if check_traffic_light_at_intersection(car, int_x, int_y, light_state) {
            return true;
        }

        // Check if intersection is occupied (before entering)
        if !car.in_intersection {
            let approaching_intersection = match car.direction {
                Direction::Down => {
                    (car_x - int_x).abs() < 20.0 && int_y > car_y && (int_y - car_y) < 50.0
                }
                Direction::Up => {
                    (car_x - int_x).abs() < 20.0 && int_y < car_y && (car_y - int_y) < 50.0
                }
                Direction::Right => {
                    (car_y - int_y).abs() < 20.0 && int_x > car_x && (int_x - car_x) < 50.0
                }
                Direction::Left => {
                    (car_y - int_y).abs() < 20.0 && int_x < car_x && (car_x - int_x) < 50.0
                }
            };

            if approaching_intersection && check_intersection_occupied(car, int_x, int_y, other_cars)
            {
                return true;
            }
        }
    }

    // Check for collision with other cars
    check_car_collision(car, other_cars)
}

// ============================================================================
// Main Update Loop
// ============================================================================

/// Updates all cars' positions and behaviors for one frame
///
/// This is the main simulation loop that handles:
/// - Traffic light compliance
/// - Collision avoidance
/// - Intersection navigation and turning
/// - Car removal when off-screen
///
/// # Arguments
/// * `cars` - Mutable vector of all cars
/// * `intersections` - All intersections with traffic lights
/// * `dt` - Delta time (frame duration in seconds)
/// * `all_lights_red` - Emergency mode flag (stops all traffic)
pub fn update_cars(
    cars: &mut Vec<Car>,
    intersections: &[Intersection],
    dt: f32,
    all_lights_red: bool,
) {
    // Create a snapshot for collision checking (avoid borrow issues)
    let cars_copy = cars.clone();

    // Update each car and remove those that drive off-screen
    cars.retain_mut(|car| {
        // Update intersection state and handle turning
        let (at_any_intersection, _turned) = update_car_at_intersection(car, intersections);

        // Reset flags when leaving all intersections
        if !at_any_intersection {
            car.just_turned = false;
            car.in_intersection = false;
        }

        // Check if car should stop (traffic lights, occupied intersections, collisions)
        let stop = should_car_stop(car, intersections, &cars_copy, all_lights_red);

        // Move car if not stopped
        if !stop {
            move_car(car, dt);
        }

        // Keep car only if still on screen
        is_car_on_screen(car)
    });
}
