
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Goal {
    pub position: Vec2,
    pub radius: f32
}

pub fn setup_goal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: ResMut<Windows>,
) {
    let window = windows.get_primary().unwrap();


    let radius = 50.;
    let position = Vec2::new(500., 325.);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.)),
            ..default()
        },
        Goal {
            position: Vec2::new(position.x + window.width() / 2., position.y + window.height() / 2.),
            radius: radius
        },
    ));
}