use bevy::{prelude::{Query}, window::Window};

use crate::kiddyboids::{walls::{HorizontalWall, VerticalWall}, boids};

use super::Boid;

pub fn vertical_collision_check(boid: &mut Boid, vertical_walls: &Query<&VerticalWall>,
    window_height: f32, window_width: f32
) {
    let y = boid.y - window_height / 2.0;
    let x = boid.x - window_width / 2.0;
    for wall in vertical_walls.iter() {
        if y > wall.startpoint.y && y < (wall.startpoint.y + wall.size) {
            if x < (wall.startpoint.x + boids::MARGINS) && x > wall.startpoint.x {
                let turn = boids::MARGINS / (x - wall.startpoint.x) - 1.0;
                boid.velocity_x += boids::TURN_FACTOR * turn;
            }

            if x > (wall.startpoint.x - boids::MARGINS) && x < wall.startpoint.x {
                let turn = boids::MARGINS / (wall.startpoint.x - x) - 1.0;
                boid.velocity_x -= boids::TURN_FACTOR * turn;
            }
        }
    }
}


pub fn horizontal_collision_check(boid: &mut Boid, horizontal_walls: &Query<&HorizontalWall>,
    window_height: f32, window_width: f32
) {
    let y = boid.y - window_height / 2.0;
    let x = boid.x - window_width / 2.0;
    for wall in horizontal_walls.iter() {
        if x > wall.startpoint.x && x < (wall.startpoint.x + wall.size) {
            if y < (wall.startpoint.y + boids::MARGINS) && y > wall.startpoint.y {
                let turn = boids::MARGINS / (y - wall.startpoint.y) - 1.0;
                boid.velocity_y += boids::TURN_FACTOR * turn;
            }

            if y > (wall.startpoint.y - boids::MARGINS) && y < wall.startpoint.y {
                let turn = boids::MARGINS / (wall.startpoint.y - y) - 1.0;
                boid.velocity_y -= boids::TURN_FACTOR * turn;
            }
        }
    }
}

pub fn window_collision_check(boid: &mut Boid, window: &Window) {
    if boid.x > window.width() - boids::MARGINS {
        boid.velocity_x -= boids::TURN_FACTOR;
    } else if boid.x < boids::MARGINS {
        boid.velocity_x += boids::TURN_FACTOR;
    }
    if boid.y > window.height() - boids::MARGINS {
        boid.velocity_y -= boids::TURN_FACTOR;
    } else if boid.y < boids::MARGINS {
        boid.velocity_y += boids::TURN_FACTOR;
    }
}