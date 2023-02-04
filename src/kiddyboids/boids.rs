use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::*};
use std::{f32::consts::TAU};
use rand::Rng;

use crate::kiddyboids::MousePosition;


const VISUAL_RANGE: f32 = 80.;
const PROTECTED_RANGE: f32 = 25.;
const MOUSE_ATTRACTION: f32 = 0.008;
const TURN_FACTOR: f32 = 0.2;
const SEPARATION_FACTOR: f32 = 0.2;
const MATCHING_FACTOR: f32 = 0.05;
const CENTERING_FACTOR: f32 = 0.0005;
const MAX_SPEED: f32 = 400.;
const MIN_SPEED: f32 = 150.;

const BOID_COUNT: i32 = 500;


// TODO: implement array of boids???
#[derive(Resource, Deref, DerefMut)]
pub struct BoidsList (pub Vec<Boid>);

#[derive(Component)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

#[derive(Component, Deref)]
pub struct BoidId (usize);

#[derive(Component)]
enum Movement {
    Mouse,
}

pub fn boids_setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut boids_list: ResMut<BoidsList>,
) {
    let mut rng = rand::thread_rng();
    for i in 0..BOID_COUNT {
        let radians = i as f32 * TAU / BOID_COUNT as f32;
        let radius = 100.0 * rng.gen_range(0.1 .. 4.0);
        let x = 400. + f32::sin(radians) * radius;
        let y = 300. - f32::cos(radians) * radius;
        boids_list.push(Boid{x, y, velocity_x: 0., velocity_y: 0.});
    }

    for i in 0..boids_list.len() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                ..default()
            },
            BoidId(i),
        ));
    }
}




pub fn boid_movement (
    time: Res<Time>,
    mut boid_position: Query<(&mut BoidId, &mut Transform)>,
    mut boids_list: ResMut<BoidsList>,
    mouse_position: Res<MousePosition>,
    windows: ResMut<Windows>,
) {
    let size = boids_list.len();
    for i in 0..size {

        let mut neighbours = 0.0;
        let mut close_x = 0.0;
        let mut close_y = 0.0;
        let mut align_x = 0.0;
        let mut align_y = 0.0;
        let mut cohesion_x = 0.0;
        let mut cohesion_y = 0.0;

        for j in 0..size {
            let boid = &boids_list[i];
            let other = &boids_list[j];
            
            let mut dist_x = boid.x - other.x;
            dist_x *= dist_x;
            let mut dist_y = boid.y - other.y;
            dist_y *= dist_y;
            let distance = dist_x + dist_y;
            // using squared distance cuz roots suck
            if distance < PROTECTED_RANGE * PROTECTED_RANGE {
                close_x += boid.x - other.x;
                close_y += boid.y - other.y;
            } 
            if distance < VISUAL_RANGE {
                neighbours += 1.;
                align_x += other.velocity_x;
                align_y += other.velocity_y;
                cohesion_x += other.x;
                cohesion_y += other.y;
            }
        }

        let boid = &mut boids_list[i];
        boid.velocity_x += close_x * SEPARATION_FACTOR;
        boid.velocity_y += close_y * SEPARATION_FACTOR;
        boid.velocity_x += (align_x / neighbours - boid.velocity_x) * MATCHING_FACTOR;
        boid.velocity_y += (align_y / neighbours - boid.velocity_y) * MATCHING_FACTOR;
        boid.velocity_x += (cohesion_x / neighbours - boid.x) * CENTERING_FACTOR;
        boid.velocity_y += (cohesion_y / neighbours - boid.y) * CENTERING_FACTOR;
        boid.velocity_x += (mouse_position.x - boid.x) * MOUSE_ATTRACTION;
        boid.velocity_y += (mouse_position.y - boid.y) * MOUSE_ATTRACTION;

        let mut speed = boid.velocity_x * boid.velocity_x + boid.velocity_y * boid.velocity_y;
        speed = f32::sqrt(speed);
        if speed > MAX_SPEED {
            boid.velocity_x *= MAX_SPEED / speed;
            boid.velocity_y *= MAX_SPEED / speed;
        } else if speed < MIN_SPEED {
            info!("{:?}:{:?} -> {:?}:{:?}", boid.velocity_x, boid.velocity_y, speed, MIN_SPEED);
            boid.velocity_x *= MIN_SPEED / speed;
            boid.velocity_y *= MIN_SPEED / speed;
        }
    }


    // apply boid positions to sprites
    for (boid_id, mut transform) in &mut boid_position {
        let mut boid = &mut boids_list[boid_id.0];
        boid.x += boid.velocity_x * time.delta_seconds();
        boid.y += boid.velocity_y * time.delta_seconds();
     
        let window = windows.get_primary().unwrap();
        transform.translation.x = boid.x - window.width() / 2.0;
        transform.translation.y = boid.y - window.height() / 2.0;
        
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

