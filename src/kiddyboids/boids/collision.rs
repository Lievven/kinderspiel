use bevy::prelude::Query;

use crate::kiddyboids::{walls::HorizontalWall, boids};

use super::Boid;

pub fn collisioncheck(boid: &mut Boid, horizontal_walls: &Query<&HorizontalWall>) {
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