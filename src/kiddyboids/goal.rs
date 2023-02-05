use std::cmp;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Goal {
    pub location: Vec2,
}

pub fn setup_goals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: ResMut<Windows>,
) {
    let window = windows.get_primary().unwrap();

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..default()}
        , Goal {location: Vec2::new(50.,50.)}
    ));

    let startpoint = Vec2::new(300., 50.);
    let endpoint = Vec2::new(-300., -50.);

    create_horizontal_wall(
        &mut commands,
        &mut meshes,
        &mut materials,
        startpoint,
        endpoint,
        window.height(),
        window.width(),
    );

    // create_vertical_wall(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     startpoint,
    //     endpoint,
    // );
}

fn create_horizontal_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    startpoint: Vec2,
    endpoint: Vec2,
    window_height: f32,
    window_width: f32,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new((startpoint.x - endpoint.x).abs(), 10.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(
                f32::min(startpoint.x, endpoint.x) + (startpoint.x - endpoint.x) / 2.,
                f32::min(startpoint.y, endpoint.y) + (startpoint.y - endpoint.y) / 2.,
                1.,
            )),
            ..default()
        },
        HorizontalWall {
            startpoint: Vec2::new(
                f32::min(startpoint.x, endpoint.x) + window_width / 2.,
                f32::min(startpoint.y, endpoint.y)
                    + (startpoint.y - endpoint.y) / 2.
                    + window_height / 2.,
            ),
            size: (startpoint.x - endpoint.x).abs(),
        },
    ));
}

fn create_vertical_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    startpoint: Vec2,
    endpoint: Vec2,
    window_height: f32,
    window_width: f32,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10., startpoint.y - endpoint.y)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(
                f32::min(startpoint.x, endpoint.x) + (startpoint.x - endpoint.x) / 2.,
                f32::min(startpoint.y, endpoint.y) + (startpoint.y - endpoint.y) / 2.,
                1.,
            )),
            ..default()
        },
        VerticalWall {
            startpoint: Vec2::new(
                f32::min(startpoint.x, endpoint.x) + window_width / 2.,
                f32::min(startpoint.y, endpoint.y)
                    + (startpoint.y - endpoint.y) / 2.
                    + window_height / 2.,
            ),
            size: (startpoint.y - endpoint.y).abs(),
        },
    ));
}
