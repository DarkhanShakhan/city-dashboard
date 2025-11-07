use macroquad::prelude::*;
use crate::models::Intersection;

// Traffic light timing (in seconds)
const GREEN_DURATION: f32 = 3.0;
const YELLOW_DURATION: f32 = 1.0;
const RED_DURATION: f32 = 3.0;
const CYCLE_DURATION: f32 = GREEN_DURATION + YELLOW_DURATION + RED_DURATION;

const TRAFFIC_LIGHT_SIZE: f32 = 20.0;
const TRAFFIC_LIGHT_SPACING: f32 = 5.0;
const DEPTH_OFFSET: f32 = 5.0; // Small offset for depth effect
const ROAD_WIDTH: f32 = 60.0;

pub fn get_traffic_light_state(time_offset: f32) -> u8 {
    let time = (get_time() as f32 + time_offset) % CYCLE_DURATION;

    if time < GREEN_DURATION {
        2 // Green
    } else if time < GREEN_DURATION + YELLOW_DURATION {
        1 // Yellow
    } else {
        0 // Red
    }
}

pub fn draw_traffic_light(x: f32, y: f32, active_light: u8) {
    let box_width = TRAFFIC_LIGHT_SIZE + 10.0;
    let box_height = TRAFFIC_LIGHT_SIZE * 3.0 + TRAFFIC_LIGHT_SPACING * 4.0;

    // Draw traffic light box
    draw_rectangle(x, y, box_width, box_height, Color::new(0.2, 0.2, 0.2, 1.0));

    // Draw depth edge for 2.5D effect
    draw_rectangle(
        x + box_width,
        y,
        DEPTH_OFFSET,
        box_height,
        Color::new(0.1, 0.1, 0.1, 1.0)
    );
    draw_rectangle(
        x,
        y + box_height,
        box_width,
        DEPTH_OFFSET,
        Color::new(0.1, 0.1, 0.1, 1.0)
    );

    // Draw pole
    draw_rectangle(
        x + box_width / 2.0 - 3.0,
        y + box_height,
        6.0,
        20.0,
        Color::new(0.3, 0.3, 0.3, 1.0)
    );
    // Pole depth edge
    draw_rectangle(
        x + box_width / 2.0 + 3.0,
        y + box_height,
        DEPTH_OFFSET,
        20.0,
        Color::new(0.15, 0.15, 0.15, 1.0)
    );

    let light_x = x + box_width / 2.0;
    let radius = TRAFFIC_LIGHT_SIZE / 2.0;

    // Red light
    let red_y = y + TRAFFIC_LIGHT_SPACING + radius;
    let red_color = if active_light == 0 { RED } else { Color::new(0.3, 0.0, 0.0, 1.0) };
    draw_circle(light_x, red_y, radius, red_color);

    // Yellow light
    let yellow_y = red_y + TRAFFIC_LIGHT_SIZE + TRAFFIC_LIGHT_SPACING;
    let yellow_color = if active_light == 1 { YELLOW } else { Color::new(0.3, 0.3, 0.0, 1.0) };
    draw_circle(light_x, yellow_y, radius, yellow_color);

    // Green light
    let green_y = yellow_y + TRAFFIC_LIGHT_SIZE + TRAFFIC_LIGHT_SPACING;
    let green_color = if active_light == 2 { Color::new(0.0, 1.0, 0.0, 1.0) } else { Color::new(0.0, 0.3, 0.0, 1.0) };
    draw_circle(light_x, green_y, radius, green_color);
}

pub fn draw_traffic_lights(intersections: &[Intersection]) {
    let offset = ROAD_WIDTH / 2.0 + 15.0;

    // Draw two traffic lights per intersection (diagonally opposite)
    for intersection in intersections {
        let int_x = intersection.x();
        let int_y = intersection.y();

        let vertical_light_state = get_traffic_light_state(intersection.time_offset);
        // Horizontal lights are offset by half the cycle to alternate with vertical
        let horizontal_light_state = get_traffic_light_state(intersection.time_offset + CYCLE_DURATION / 2.0);

        // First traffic light (top-right corner, for vertical traffic)
        draw_traffic_light(
            int_x + offset,
            int_y - offset - 100.0,
            vertical_light_state
        );

        // Second traffic light (bottom-left corner, for horizontal traffic)
        draw_traffic_light(
            int_x - offset - 30.0,
            int_y + offset + 10.0,
            horizontal_light_state
        );
    }
}
