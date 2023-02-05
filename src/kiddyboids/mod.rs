use bevy::{input::mouse::MouseMotion, prelude::*, sprite::MaterialMesh2dBundle};
use std::f32::consts::TAU;

mod boids;
use boids::*;

mod sprites;
use sprites::*;

mod walls;
use walls::*;

#[derive(Resource)]
pub struct MousePosition {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub enum Direction {
    Mouse,
}

pub fn run(mut app: App) {
    app
    .insert_resource(MousePosition { x: 0., y: 0. })
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .insert_resource(BoidsList(Vec::new()))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: format!("Kinderspiel"),
            width: 800.0,
            height: 600.0,
            ..default()
        },
        ..default()
    }))
    .add_startup_system(setup_walls)
    .add_startup_system(setup)
//    .add_startup_system(boids_setup)
    .add_startup_system(boids_sprite_setup)
    .add_system(animate_sprite)
    .add_system(sprite_movement)
    .add_system(mouse_click_system)
    .add_system(boid_movement)
    .run();
}

pub fn mouse_click_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for event in cursor_moved_events.iter() {
        mouse_position.x = event.position.x;
        mouse_position.y = event.position.y;
    }
}



pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            ..default()
        }
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            ..default()
        },
        Direction::Mouse,
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}
