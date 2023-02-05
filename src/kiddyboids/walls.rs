use bevy::{prelude::*, sprite::MaterialMesh2dBundle};



#[derive(Component)]
pub struct Wall {
    point_1: Vec2,
    point_2: Vec2
}

pub fn setup_walls (
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
}