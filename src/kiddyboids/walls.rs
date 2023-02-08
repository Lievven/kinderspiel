
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

const MAZE_WIDTH: usize = 6;
const MAZE_HEIGHT: usize = 5;

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
    let window = windows.get_primary().unwrap();
    /*for i in 0..=MAZE_WIDTH {
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
    }); */

    generate_walls(
        0,
        0,
        &mut commands,
        &mut meshes,
        &mut materials,
        window.height(),
        window.width(),
    );

}



fn create_horizontal_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    startpoint: Vec2,
    endpoint: Vec2,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(
                Vec2::new((startpoint.x - endpoint.x).abs(), 10.))
                .into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(
                    f32::min(startpoint.x, endpoint.x) + (startpoint.x - endpoint.x).abs() / 2.0,
                    f32::min(startpoint.y, endpoint.y) + (startpoint.y - endpoint.y).abs() / 2.0,
                    1.0,
                )),
            ..default()
        },
        HorizontalWall {
            startpoint: Vec2::new(
                f32::min(startpoint.x, endpoint.x),
                f32::min(startpoint.y, endpoint.y)
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
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(
                Vec2::new(10., (startpoint.y - endpoint.y).abs()))
                .into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_translation(Vec3::new(
                f32::min(startpoint.x, endpoint.x) + (startpoint.x - endpoint.x).abs() / 2.,
                f32::min(startpoint.y, endpoint.y) + (startpoint.y - endpoint.y).abs() / 2.,
                1.,
            )),
            ..default()
        },
        VerticalWall {
            startpoint: Vec2::new(
                f32::min(startpoint.x, endpoint.x),
                f32::min(startpoint.y, endpoint.y)
            ),
            size: (startpoint.y - endpoint.y).abs(),
        },
    ));
}


fn generate_walls (
    start_x: usize,
    start_y: usize,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    window_height: f32,
    window_width: f32,
) {
    let mut visited = [[false; MAZE_HEIGHT]; MAZE_WIDTH];
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push((start_x, start_y));
    visited [start_x][start_y] = true;
    let mut horizontal_walls = [[true; MAZE_HEIGHT-1]; MAZE_WIDTH];
    let mut vertical_walls = [[true; MAZE_HEIGHT]; MAZE_WIDTH-1];

    let mut rng = rand::thread_rng();
    while !stack.is_empty() {
        let target = rng.gen_range(0..stack.len());
        let (active_x, active_y) = stack.remove(target);
        let offset = rng.gen_range(0..4);
        for i in 0..4 {
            let i = (i + offset) % 4;
            if i==0 && active_x > 0 && !visited[active_x - 1][active_y] {
                stack.push((active_x, active_y));
                stack.push((active_x -1, active_y));
                visited[active_x-1][active_y] = true;
                vertical_walls[active_x-1][active_y] = false;
            } else if i==1 && active_y > 0 && !visited[active_x][active_y - 1] {
                stack.push((active_x, active_y));
                stack.push((active_x, active_y - 1));
                visited[active_x][active_y - 1] = true;
                horizontal_walls[active_x][active_y-1] = false;
            } else if i==2 &&  active_x < MAZE_WIDTH - 1 && !visited[active_x + 1][active_y] {
                stack.push((active_x, active_y));
                stack.push((active_x + 1, active_y));
                visited[active_x + 1][active_y] = true;
                vertical_walls[active_x][active_y] = false;
            } else if i==3 && active_y < MAZE_HEIGHT - 1 && !visited[active_x][active_y + 1] {
                stack.push((active_x, active_y));
                stack.push((active_x, active_y + 1));
                visited[active_x][active_y + 1] = true;
                horizontal_walls[active_x][active_y] = false;
            }
        }
    }

    for i in 0..horizontal_walls.len() {
        for j in 0..horizontal_walls[i].len() {
            if horizontal_walls[i][j] {
                let x1 = (i) as f32 * window_width / MAZE_WIDTH as f32 - window_width / 2.0;
                let x2 = (i+1) as f32 * window_width / MAZE_WIDTH as f32 - window_width / 2.0;
                let y = (j+1) as f32 * window_height / MAZE_HEIGHT as f32 - window_height / 2.0;
                let startpoint = Vec2{x: x1, y};
                let endpoint = Vec2{x: x2, y};
                create_horizontal_wall(
                    commands,
                    meshes,
                    materials,
                    startpoint,
                    endpoint,
                );
            }
        }
    }

    for i in 0..vertical_walls.len() {
        for j in 0..vertical_walls[i].len() {
            if vertical_walls[i][j] {
                let x = (i+1) as f32 * window_width / MAZE_WIDTH as f32 - window_width / 2.0;
                let y1 = (j) as f32 * window_height / MAZE_HEIGHT as f32 - window_height / 2.0;
                let y2 = (j+1) as f32 * window_height / MAZE_HEIGHT as f32 - window_height / 2.0;
                let startpoint = Vec2{x, y: y1};
                let endpoint = Vec2{x, y: y2};
                create_vertical_wall(
                    commands,
                    meshes,
                    materials,
                    startpoint,
                    endpoint,
                );
            }
        }
    }
}