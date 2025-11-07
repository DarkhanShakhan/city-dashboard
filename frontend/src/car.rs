use crate::models::{Car, Direction, Intersection};
use crate::traffic_light::get_traffic_light_state;
use macroquad::prelude::*;

pub const CAR_WIDTH: f32 = 20.0;
pub const CAR_HEIGHT: f32 = 35.0;
pub const CAR_SPEED: f32 = 50.0; // pixels per second
const ROAD_WIDTH: f32 = 60.0;

// Traffic light timing (in seconds)
const GREEN_DURATION: f32 = 3.0;
const YELLOW_DURATION: f32 = 1.0;
const RED_DURATION: f32 = 3.0;
const CYCLE_DURATION: f32 = GREEN_DURATION + YELLOW_DURATION + RED_DURATION;

pub fn spawn_car(cars: &mut Vec<Car>) {
    // Road positions as percentages
    let vertical_percents = vec![0.15, 0.5, 0.85];
    let horizontal_percents = vec![0.25, 0.75];

    // Randomly choose vertical or horizontal road
    let is_vertical = rand::gen_range(0, 2) == 0;

    let car_colors = [BLUE, RED, YELLOW, Color::new(1.0, 0.5, 0.0, 1.0), PURPLE];
    let color = car_colors[rand::gen_range(0, car_colors.len())];

    let lane_offset = 12.0; // pixels

    if is_vertical {
        // Spawn on vertical road (moving down or up)
        let road_index = rand::gen_range(0, vertical_percents.len());
        let road_center_percent = vertical_percents[road_index];
        let going_down = rand::gen_range(0, 2) == 0;

        // Cars going down use left lane (offset to the left)
        // Cars going up use right lane (offset to the right)
        let lane_offset_percent = lane_offset / screen_width(); // Offset in x direction
        let x_percent = if going_down {
            road_center_percent - lane_offset_percent
        } else {
            road_center_percent + lane_offset_percent
        };

        // Randomly decide if car will turn (30% chance)
        let next_turn = if rand::gen_range(0, 10) < 3 {
            // Choose a perpendicular direction for turning
            if rand::gen_range(0, 2) == 0 {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else {
            None // Go straight
        };

        cars.push(Car {
            x_percent,
            y_percent: if going_down { -0.05 } else { 1.05 }, // Spawn just off screen
            direction: if going_down {
                Direction::Down
            } else {
                Direction::Up
            },
            color,
            road_index,
            next_turn,
            just_turned: false,
            in_intersection: false,
        });
    } else {
        // Spawn on horizontal road (moving right or left)
        let road_index = rand::gen_range(0, horizontal_percents.len());
        let road_center_percent = horizontal_percents[road_index];
        let going_right = rand::gen_range(0, 2) == 0;

        // Cars going right use bottom lane (offset down)
        // Cars going left use top lane (offset up)
        let lane_offset_percent = lane_offset / screen_height(); // Offset in y direction
        let y_percent = if going_right {
            road_center_percent + lane_offset_percent
        } else {
            road_center_percent - lane_offset_percent
        };

        // Randomly decide if car will turn (30% chance)
        let next_turn = if rand::gen_range(0, 10) < 3 {
            // Choose a perpendicular direction for turning
            if rand::gen_range(0, 2) == 0 {
                Some(Direction::Down)
            } else {
                Some(Direction::Up)
            }
        } else {
            None // Go straight
        };

        cars.push(Car {
            x_percent: if going_right { -0.05 } else { 1.05 }, // Spawn just off screen
            y_percent,
            direction: if going_right {
                Direction::Right
            } else {
                Direction::Left
            },
            color,
            road_index,
            next_turn,
            just_turned: false,
            in_intersection: false,
        });
    }
}

fn check_traffic_light_at_intersection(
    car: &Car,
    intersection_x: f32,
    intersection_y: f32,
    light_state: u8,
) -> bool {
    // Returns true if car should stop, false if it can go
    // IMPORTANT: Never stop a car that's already in the intersection
    if car.in_intersection {
        return false; // Cars in intersection must continue through
    }

    let stop_distance_min = 30.0; // Minimum distance before intersection to stop
    let stop_distance_max = 80.0; // Maximum distance to consider stopping
    let lane_tolerance = 20.0; // Tolerance for lane detection (wider to account for lane offsets)

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

fn check_intersection_occupied(
    car: &Car,
    intersection_x: f32,
    intersection_y: f32,
    other_cars: &[Car],
) -> bool {
    // Check if another car is already in this intersection
    let intersection_radius = 40.0; // Radius to consider as "in intersection"

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

fn check_car_collision(car: &Car, other_cars: &[Car]) -> bool {
    // Don't stop if car is in intersection - must complete crossing
    if car.in_intersection {
        return false;
    }

    let safe_distance = 50.0; // Minimum distance to maintain

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

pub fn update_cars(cars: &mut Vec<Car>, intersections: &[Intersection], dt: f32) {
    // Create a copy for collision checking
    let cars_copy = cars.clone();

    cars.retain_mut(|car| {
        let mut should_stop = false;
        let mut at_any_intersection = false;

        let car_x = car.x();
        let car_y = car.y();

        // Check all intersections for traffic lights and turning
        for intersection in intersections {
            let int_x = intersection.x();
            let int_y = intersection.y();

            // Check if car is at this intersection
            let lane_offset = 12.0;
            let intersection_radius = 40.0;
            let dist_to_intersection = ((car_x - int_x).powi(2) + (car_y - int_y).powi(2)).sqrt();
            let at_intersection = dist_to_intersection < intersection_radius;

            if at_intersection {
                at_any_intersection = true;
                car.in_intersection = true; // Mark car as in intersection
            }

            // Check appropriate light based on car direction
            let light_state = if car.direction == Direction::Down || car.direction == Direction::Up
            {
                get_traffic_light_state(intersection.time_offset) // Vertical light
            } else {
                get_traffic_light_state(intersection.time_offset + CYCLE_DURATION / 2.0) // Horizontal light
            };

            // Check traffic light (won't stop if already in intersection)
            if check_traffic_light_at_intersection(car, int_x, int_y, light_state) {
                should_stop = true;
                break;
            }

            // Check if intersection is occupied by another car (before entering)
            if !car.in_intersection && !should_stop {
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

                if approaching_intersection
                    && check_intersection_occupied(car, int_x, int_y, &cars_copy)
                {
                    should_stop = true;
                    break;
                }
            }

            // Check for turning at intersection center
            let at_intersection_center = match car.direction {
                Direction::Down => (car_x - int_x).abs() < 15.0 && (car_y - int_y).abs() < 10.0,
                Direction::Up => (car_x - int_x).abs() < 15.0 && (car_y - int_y).abs() < 10.0,
                Direction::Right => (car_y - int_y).abs() < 15.0 && (car_x - int_x).abs() < 10.0,
                Direction::Left => (car_y - int_y).abs() < 15.0 && (car_x - int_x).abs() < 10.0,
            };

            if at_intersection_center {
                if car.next_turn.is_some() && !car.just_turned {
                    // Execute the turn
                    let new_direction = car.next_turn.unwrap();
                    car.direction = new_direction.clone();

                    // Adjust position to new lane (left-hand traffic)
                    match new_direction {
                        Direction::Down => {
                            car.x_percent = intersection.x_percent - (lane_offset / screen_width());
                            car.y_percent = intersection.y_percent;
                        }
                        Direction::Up => {
                            car.x_percent = intersection.x_percent + (lane_offset / screen_width());
                            car.y_percent = intersection.y_percent;
                        }
                        Direction::Right => {
                            car.x_percent = intersection.x_percent;
                            car.y_percent =
                                intersection.y_percent + (lane_offset / screen_height());
                        }
                        Direction::Left => {
                            car.x_percent = intersection.x_percent;
                            car.y_percent =
                                intersection.y_percent - (lane_offset / screen_height());
                        }
                    }

                    // Plan next turn (30% chance)
                    car.next_turn = if rand::gen_range(0, 10) < 3 {
                        match new_direction {
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
                    };

                    // Mark that we just turned
                    car.just_turned = true;
                    break; // Only turn at one intersection
                }
            }
        }

        // Reset the flags when we leave all intersections
        if !at_any_intersection {
            car.just_turned = false;
            car.in_intersection = false;
        }

        // Check for collision with other cars
        if !should_stop && check_car_collision(car, &cars_copy) {
            should_stop = true;
        }

        // Move car if not stopped
        if !should_stop {
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

        // Remove cars that are off screen
        car.x_percent > -0.1 && car.x_percent < 1.1 && car.y_percent > -0.1 && car.y_percent < 1.1
    });
}
