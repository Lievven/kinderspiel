use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::TAU;
mod collision;
mod goalcollision;

use super::{
    goal_setup::Goal,
    walls::{HorizontalWall, VerticalWall},
};
use crate::kiddyboids::MousePosition;

pub const VISUAL_RANGE: f32 = 80.;
pub const PROTECTED_RANGE: f32 = 40.;
pub const MOUSE_ATTRACTION: f32 = 0.08;
pub const GOAL_ATTRACTION: f32 = 0.9;
pub const SEPARATION_FACTOR: f32 = 0.8;
pub const MATCHING_FACTOR: f32 = 0.5;
pub const CENTERING_FACTOR: f32 = 0.005;
pub const MAX_SPEED: f32 = 400.;
pub const MIN_SPEED: f32 = 150.;
pub const MARGINS: f32 = 50.;
pub const TURN_FACTOR: f32 = 40.0;

pub const BOID_COUNT: i32 = 5;
pub const RARE_CHANCE: f32 = 10.0;

pub const ATLAS_RARE: &'static [&'static str] = &[
    "boid_atlas_germany.png",
    "boid_atlas_ukraine.png",
    "boid_atlas_rainbow.png",
    "boid_atlas_kiel.png",
];

pub const ATLAS_COMMON: &'static [&'static str] = &[
    "boid_atlas_tricolore.png",
    "boid_atlas_tricolore_2.png",
    "boid_atlas_flower.png",
    "boid_atlas_flower_2.png",
    "boid_atlas_plain.png",
    "boid_atlas_plain_2.png",
    "boid_atlas_plain_3.png",
    "boid_atlas_star.png",
    "boid_atlas_star_2.png",
    "boid_atlas_star_3.png",
    "boid_atlas_star_4.png",
];

pub const SPRITE_SIZE: Vec2 = Vec2::new(256.0, 256.0);
pub const SPRITE_SCALE: f32 = 1.0 / 6.0;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct BoidsList(pub Vec<Boid>);

#[derive(Component, Debug)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub is_active: bool,
}

#[derive(Component, Deref)]
pub struct BoidId(usize);

#[derive(Component, Deref)]
pub struct IsActive(bool);

pub fn boids_sprite_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut boids_list: ResMut<BoidsList>,
) {
    let mut rare_handles = Vec::with_capacity(ATLAS_RARE.len());
    let mut common_handles = Vec::with_capacity(ATLAS_COMMON.len());

    for i in 0..ATLAS_RARE.len() {
        let texture_handler: Handle<Image> = asset_server.load(ATLAS_RARE[i]);
        let texture_atlas = TextureAtlas::from_grid(texture_handler, SPRITE_SIZE, 2, 2, None, None);
        rare_handles.push(texture_atlases.add(texture_atlas));
    }

    for i in 0..ATLAS_COMMON.len() {
        let texture_handler: Handle<Image> = asset_server.load(ATLAS_COMMON[i]);
        let texture_atlas = TextureAtlas::from_grid(texture_handler, SPRITE_SIZE, 2, 2, None, None);
        common_handles.push(texture_atlases.add(texture_atlas));
    }

    let mut rng = rand::thread_rng();
    for i in 0..BOID_COUNT {
        let radians = i as f32 * TAU / BOID_COUNT as f32;
        let radius = 100.0 * rng.gen_range(0.1..4.0);
        let x = 400. + f32::sin(radians) * radius;
        let y = 300. - f32::cos(radians) * radius;
        boids_list.push(Boid {
            x,
            y,
            velocity_x: 0.,
            velocity_y: 0.,
            is_active: true,
        });
    }

    for i in 0..boids_list.len() {
        let rare_flip: f32 = rng.gen_range(0.0..100.0);
        let texture: Handle<TextureAtlas>;
        if rare_flip < RARE_CHANCE {
            let sprite_choice: usize = rng.gen_range(0..rare_handles.len());
            texture = rare_handles[sprite_choice].clone();
        } else {
            let sprite_choice: usize = rng.gen_range(0..common_handles.len());
            texture = common_handles[sprite_choice].clone();
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture,
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE)),
                ..default()
            },
            BoidId(i),
            IsActive(true),
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        ));
    }
}


pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &IsActive,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, is_active, mut sprite, texture_atlas_handle) in &mut query {
        if is_active.0 {
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        }
    }
}


pub fn boid_movement(
    time: Res<Time>,
    mut boid_position: Query<(&mut BoidId, &mut Transform, &mut IsActive)>,
    mut boids_list: ResMut<BoidsList>,
    mouse_position: Res<MousePosition>,
    windows: ResMut<Windows>,
    horizontal_walls: Query<&HorizontalWall>,
    vertical_walls: Query<&VerticalWall>,
    goals: Query<&Goal>,
) {
    let window = windows.get_primary().unwrap();
    let size = boids_list.len();
    let mut remaining_active = size;
    for i in 0..size {

        // skip boid if inactive
        if !boids_list[i].is_active {
            remaining_active -= 1;
            continue;
        }

        let mut neighbours = 0.0;
        let mut close_x = 0.0;
        let mut close_y = 0.0;
        let mut align_x = 0.0;
        let mut align_y = 0.0;
        let mut cohesion_x = 0.0;
        let mut cohesion_y = 0.0;

        for j in 0..size {

            // we do not want boids to collide with themselves now, do we?
            if i == j {
               continue;
            }

            let boid = &boids_list[i];
            let other = &boids_list[j];

            // skip collision with other boid if it is inactive
            if !other.is_active {
                continue;
            }

            let mut dist_x = boid.x - other.x;
            dist_x *= dist_x;
            let mut dist_y = boid.y - other.y;
            dist_y *= dist_y;
            let distance = dist_x + dist_y;
            // using squared distance cuz roots suck
            if distance < PROTECTED_RANGE * PROTECTED_RANGE {
                close_x += (boid.x - other.x) * PROTECTED_RANGE / f32::sqrt(distance);
                close_y += boid.y - other.y * PROTECTED_RANGE / f32::sqrt(distance);
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
        if neighbours > 0.0 {
            boid.velocity_x += (align_x / neighbours - boid.velocity_x) * MATCHING_FACTOR;
            boid.velocity_y += (align_y / neighbours - boid.velocity_y) * MATCHING_FACTOR;
            boid.velocity_x += (cohesion_x / neighbours - boid.x) * CENTERING_FACTOR;
            boid.velocity_y += (cohesion_y / neighbours - boid.y) * CENTERING_FACTOR;
        }
        boid.velocity_x += (mouse_position.x - boid.x) * MOUSE_ATTRACTION;
        boid.velocity_y += (mouse_position.y - boid.y) * MOUSE_ATTRACTION;

        let goal = goals.iter().next();
        if let Some(goal) = goal {
            if goal.position.distance(Vec2::new(boid.x, boid.y)) < (goal.radius * 2.) {
                boid.velocity_x += (goal.position.x - boid.x) * GOAL_ATTRACTION;
                boid.velocity_y += (goal.position.y - boid.y) * GOAL_ATTRACTION;
            }
        }

        collision::horizontal_collision_check(
            boid,
            &horizontal_walls,
            window.height(),
            window.width(),
        );
        collision::vertical_collision_check(
            boid,
            &vertical_walls,
            window.height(),
            window.width(),
        );
        goalcollision::goal_collisioncheck(boid, &goals);

        collision::window_collision_check(boid, &window);

        let mut speed = boid.velocity_x * boid.velocity_x + boid.velocity_y * boid.velocity_y;
        speed = f32::sqrt(speed);
        if speed > MAX_SPEED {
            boid.velocity_x *= MAX_SPEED / speed;
            boid.velocity_y *= MAX_SPEED / speed;
        } else if speed < MIN_SPEED {
            boid.velocity_x *= MIN_SPEED / speed;
            boid.velocity_y *= MIN_SPEED / speed;
        }
       
    }

    if remaining_active == 0 {
        reset(&mut boids_list);
    }

    apply_boid_position(&mut boid_position, &mut boids_list, time, window);

}


fn apply_boid_position (
    boid_position: &mut Query<(&mut BoidId, &mut Transform, &mut IsActive)>,
    boids_list: &mut ResMut<BoidsList>,
    time: Res<Time>,
    window: &Window,
) {
    for (boid_id, mut transform, mut is_active) in boid_position {
        let mut boid = &mut boids_list[boid_id.0];

        if !is_active.0 && boid.is_active {
            transform.translation.z = 1.;
        }

        is_active.0 = boid.is_active;
        if is_active.0 {
            boid.x += boid.velocity_x * time.delta_seconds();
            boid.y += boid.velocity_y * time.delta_seconds();

            let pointing: f32 = f32::atan2(-boid.velocity_x, boid.velocity_y);
            transform.rotation = Quat::from_rotation_z(pointing);

            transform.translation.x = boid.x - window.width() / 2.0;
            transform.translation.y = boid.y - window.height() / 2.0;
        } else {
            transform.translation.z = -10.;
        }
    }
}


fn reset(boids_list: &mut ResMut<BoidsList>){
    for boid in boids_list.iter_mut(){
        boid.is_active = true;
        boid.x = 0.;
        boid.y = 0.;
    }
}
