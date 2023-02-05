use bevy::prelude::Query;

use crate::kiddyboids::{walls::{HorizontalWall, VerticalWall}, boids};

use super::Boid;

pub fn vertical_collision_check(boid: &mut Boid, vertical_walls: &Query<&VerticalWall>) {
    for wall in vertical_walls.iter() {
        if boid.y > wall.startpoint.y && boid.y < (wall.startpoint.y + wall.size) {
            if boid.y < (wall.startpoint.y + boids::MARGINS) && boid.y > wall.startpoint.y {
                boid.velocity_y += boids::TURN_FACTOR;
            }

            if boid.x > (wall.startpoint.x - boids::MARGINS) && boid.x < wall.startpoint.x {
                boid.velocity_x -= boids::TURN_FACTOR;
            }
        }
    }
}
pub fn horizontal_collision_check(boid: &mut Boid, horizontal_walls: &Query<&HorizontalWall>) {
    for wall in horizontal_walls.iter() {
        if boid.x > wall.startpoint.x && boid.x < (wall.startpoint.x + wall.size) {
            if boid.y < (wall.startpoint.y + boids::MARGINS) && boid.y > wall.startpoint.y {
                boid.velocity_y += boids::TURN_FACTOR;
            }

            if boid.y > (wall.startpoint.y - boids::MARGINS) && boid.y < wall.startpoint.y {
                boid.velocity_y -= boids::TURN_FACTOR;
            }
        }
    }
}