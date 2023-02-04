use bevy::{input::mouse::MouseMotion, prelude::*, sprite::MaterialMesh2dBundle};
use std::f32::consts::TAU;

const TURN_RATE: f32 = 0.2;
const VISUAL_RANGE: f32 = 400.;
const PROTECTED_RANGE: f32 = 8.;
const MOUSE_ATTRACTION: f32 = 0.03;
const SEPARATION_FACTOR: f32 = 0.05;
const CENTERING_FACTOR: f32 = 0.0005;
const MATCHING_FACTOR: f32 = 0.05;
const MAX_SPEED: f32 = 6.;
const MIN_SPEED: f32 = 3.;

fn main() {
    App::new()
        .insert_resource(MousePosition { x: 0., y: 0. })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: format!("Kinderspiel"),
                width: 800.0,
                height: 600.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(mouse_click_system)
        .add_system(boid_movement)
        .run();
}

#[derive(Resource)]
struct MousePosition {
    x: f32,
    y: f32,
}

#[derive(Resource)]
struct Boids {

}

#[derive(Component)]
struct Boid {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

#[derive(Component)]
enum Movement {
    Mouse,
}

#[derive(Component)]
enum Direction {
    Mouse,
}

fn mouse_click_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for event in cursor_moved_events.iter() {
        mouse_position.x = event.position.x;
        mouse_position.y = event.position.y;
    }
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    mouse_position: Res<MousePosition>,
) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Mouse => transform.translation.y = mouse_position.y - 300.,
        }
        transform.rotate_z(0.1 * TAU * time.delta_seconds());
    }
}

fn boid_movement (
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

fn setup(
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

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TEAL)),
            ..default()
        },
        Boid{x: 0., y: 0., velocity_x: 0., velocity_y: 0.},
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(25., 8).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TEAL)),
            ..default()
        },
        Boid{x: 0., y: 0., velocity_x: 0., velocity_y: 0.},
    ));
}
