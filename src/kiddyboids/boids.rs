use bevy::{prelude::*, math::Vec3Swizzles};
use rand::Rng;
use std::f32::consts::TAU;
mod collision;
mod goalcollision;

use super::{goal_setup::Goal, walls::{HorizontalWall, VerticalWall}};
use crate::kiddyboids::MousePosition;

pub const VISUAL_RANGE: f32 = 80.;
pub const PROTECTED_RANGE: f32 = 40.;
pub const MOUSE_ATTRACTION: f32 = 0.08;
pub const TURN_FACTOR: f32 = 300.0;
pub const SEPARATION_FACTOR: f32 = 0.8;
pub const MATCHING_FACTOR: f32 = 0.5;
pub const CENTERING_FACTOR: f32 = 0.005;
pub const MAX_SPEED: f32 = 400.;
pub const MIN_SPEED: f32 = 150.;
pub const MARGINS: f32 = 60.;

pub const BOID_COUNT: i32 = 50;
pub const RARE_CHANCE: f32 = 80.0;

pub const ATLAS_RARE: &'static [&'static str] = &[
    "boid_atlas_germany.png",
    "boid_atlas_ukraine.png",
    "boid_atlas_rainbow.png",
    "boid_atlas_kiel.png",
];

pub const ATLAS_COMMON: &'static [&'static str] = &[
    "boid_atlas_tricolore.png",
    "boid_atlas_flower.png",
    "boid_atlas_plain.png",
    "boid_atlas_star.png",
];

pub const SPRITE_SIZE: Vec2 = Vec2::new(256.0, 256.0);
pub const SPRITE_SCALE: f32 = 1.0 / 6.0;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct BoidsList(pub Vec<Boid>);

#[derive(Component)]
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
    let goal = goals.iter().next();

    let size = boids_list.len();
    for i in 0..size {
        let mut neighbours = 0.0;
        let mut close_x = 0.0;
        let mut close_y = 0.0;
        let mut align_x = 0.0;
        let mut align_y = 0.0;
        let mut cohesion_x = 0.0;
        let mut cohesion_y = 0.0;
        let mut this_boid_is_active = true;

        for j in 0..size {
            let boid = &boids_list[i];
            this_boid_is_active = boid.is_active;
            if boid.is_active {
                let other = &boids_list[j];
                if other.is_active {
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
            }
        }
        if this_boid_is_active {
            let boid = &mut boids_list[i];
            boid.velocity_x += close_x * SEPARATION_FACTOR;
            boid.velocity_y += close_y * SEPARATION_FACTOR;
            boid.velocity_x += (align_x / neighbours - boid.velocity_x) * MATCHING_FACTOR;
            boid.velocity_y += (align_y / neighbours - boid.velocity_y) * MATCHING_FACTOR;
            boid.velocity_x += (cohesion_x / neighbours - boid.x) * CENTERING_FACTOR;
            boid.velocity_y += (cohesion_y / neighbours - boid.y) * CENTERING_FACTOR;
            boid.velocity_x += (mouse_position.x - boid.x) * MOUSE_ATTRACTION;
            boid.velocity_y += (mouse_position.y - boid.y) * MOUSE_ATTRACTION;

            collision::horizontal_collision_check(boid, &horizontal_walls);
            collision::vertical_collision_check(boid, &vertical_walls);
            goalcollision::goal_collisioncheck(boid, &goals);

            let window = windows.get_primary().unwrap();
            if boid.x > window.width() - MARGINS {
                boid.velocity_x -= TURN_FACTOR;
            } else if boid.x < MARGINS {
                boid.velocity_x += TURN_FACTOR;
            }
            if boid.y > window.height() - MARGINS {
                boid.velocity_y -= TURN_FACTOR;
            } else if boid.y < MARGINS {
                boid.velocity_y += TURN_FACTOR;
            }

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
    }

    // apply boid positions to sprites
    for (boid_id, mut transform, mut is_active) in &mut boid_position {
        let mut boid = &mut boids_list[boid_id.0];
        is_active.0 = boid.is_active;
        if is_active.0 {
            boid.x += boid.velocity_x * time.delta_seconds();
            boid.y += boid.velocity_y * time.delta_seconds();

            let pointing: f32 = f32::atan2(-boid.velocity_x, boid.velocity_y);
            transform.rotation = Quat::from_rotation_z(pointing);

            let window = windows.get_primary().unwrap();
            transform.translation.x = boid.x - window.width() / 2.0;
            transform.translation.y = boid.y - window.height() / 2.0;
        }
        else{
            transform.translation.z = -10.;
        }
    }
}
