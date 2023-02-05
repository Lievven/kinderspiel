
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct VerticalWall {
    pub startpoint: Vec2,
    pub size: f32,
}

#[derive(Component)]
pub struct HorizontalWall {
    pub startpoint: Vec2,
    pub size: f32,
}

pub fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: ResMut<Windows>,
) {
    const MAZE_WIDTH: i32 = 6;
    const MAZE_HEIGHT: i32 = 5;
    let window = windows.get_primary().unwrap();
    for i in 0..=MAZE_WIDTH {
        let x = window.width() / MAZE_WIDTH as f32 * i as f32 - (window.width() / 2.);
        for j in 0..=MAZE_HEIGHT {
            let y = window.height() / MAZE_HEIGHT as f32 * j as f32 - window.height() / 2.;

            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(5., 6).into()).into(),
                material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            });
        }
    }

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(5., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..default()
    });

    // for x in 0..30 {
    //     let x = 50. * x as f32;
    //     for y in 0..15 {
    //         let y = 50. * y as f32;
    //         {
    //         create_horizontal_wall(
    //             &mut commands,
    //             &mut meshes,
    //             &mut materials,
    //             Vec2::new(x, y),
    //             Vec2::new(x + 20., y + 20.),
    //         );
    //     }
    //         info!("{:?}, {:?}", x, y);
    //     }
    // }

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
