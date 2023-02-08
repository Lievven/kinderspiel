use bevy::{prelude::*};

mod boids;
use boids::*;

mod walls;
use walls::*;

mod goal_setup;
use goal_setup::*;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 800.0;

#[derive(Resource)]
pub struct MousePosition {
    x: f32,
    y: f32,
}


pub fn run(mut app: App) {
    app.insert_resource(MousePosition { x: 0., y: 0. })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(BoidsList(Vec::new()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: format!("Kinderspiel"),
                width: WIDTH,
                height: HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_walls)
        .add_startup_system(setup_goal)
        .add_startup_system(setup)
        .add_startup_system(boids_sprite_setup)
        .add_system(animate_sprite)
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
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}
