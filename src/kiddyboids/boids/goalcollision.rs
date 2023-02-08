use bevy::{prelude::*};

use crate::kiddyboids::goal_setup::Goal;

use super::Boid;


pub fn goal_collisioncheck(boid: &mut Boid, goals: &Query<&Goal>)
{
    for goal in goals.iter() {
        if Vec2::new(boid.x, boid.y).distance(goal.position) < goal.radius{
            boid.is_active = false;
        }
    }
}
