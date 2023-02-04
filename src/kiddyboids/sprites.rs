use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::f32::consts::TAU;

use crate::kiddyboids::MousePosition;
use crate::kiddyboids::Direction;

pub fn sprite_movement(
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