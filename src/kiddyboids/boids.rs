use bevy::{prelude::*};
use std::f32::consts::TAU;

use crate::kiddyboids::MousePosition;


const TURN_RATE: f32 = 0.2;
const VISUAL_RANGE: f32 = 400.;
const PROTECTED_RANGE: f32 = 8.;
const MOUSE_ATTRACTION: f32 = 0.03;
const SEPARATION_FACTOR: f32 = 0.05;
const CENTERING_FACTOR: f32 = 0.0005;
const MATCHING_FACTOR: f32 = 0.05;
const MAX_SPEED: f32 = 6.;
const MIN_SPEED: f32 = 3.;


// TODO: implement array of boids???
#[derive(Resource)]
pub struct Boids {

}

#[derive(Component)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

#[derive(Component)]
enum Movement {
    Mouse,
}

pub fn boid_movement (
    time: Res<Time>,
    mut boid_position: Query<(&mut Boid, &mut Transform)>,
    mouse_position: Res<MousePosition>,
) {

    for (mut boid, _) in &mut boid_position {
        
        boid.velocity_x += (mouse_position.x - boid.x) * MOUSE_ATTRACTION;
        boid.velocity_y += (mouse_position.y - boid.y) * MOUSE_ATTRACTION;
    }
    
    for (mut boid, mut transform) in &mut boid_position {
        boid.x += boid.velocity_x * time.delta_seconds();
        boid.y += boid.velocity_y * time.delta_seconds();
     
        transform.translation.x = boid.x - 400.;
        transform.translation.y = boid.y - 300.;
    }
}

fn compute_boid_behavior (
    mouse_position: Res<MousePosition>,
    mut boid: Boid,
    mut transform: Transform,
    boids: Query<(&mut Boid, &mut Transform)>,
) {
    for (other, other_transform) in &boids {
        let distance_x = transform.translation.x - other_transform.translation.x;
        let distance_y = transform.translation.y - other_transform.translation.y;
        let distance = distance_x * distance_x + distance_y * distance_y;
        if distance <= VISUAL_RANGE * VISUAL_RANGE {
            info!("{:?}:{:?}", other.velocity_x, other.velocity_y);
        }
    }
}

