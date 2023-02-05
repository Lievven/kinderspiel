use bevy::prelude::{Query};

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
                boid.velocity_x += boids::TURN_FACTOR;
            }

            if x > (wall.startpoint.x - boids::MARGINS) && x < wall.startpoint.x {
                boid.velocity_x -= boids::TURN_FACTOR;
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
                boid.velocity_y += boids::TURN_FACTOR;
            }

            if y > (wall.startpoint.y - boids::MARGINS) && y < wall.startpoint.y {
                boid.velocity_y -= boids::TURN_FACTOR;
            }
        }
    }
}