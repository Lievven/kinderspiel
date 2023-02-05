use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Goal {
    pub location: Vec2,
}

pub fn setup_goal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: ResMut<Windows>,
) {
    let window = windows.get_primary().unwrap();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(50., 50., 10.)),
            ..default()
        },
        Goal {
            location: Vec2::new(50., 50.),
        },
    ));
}
