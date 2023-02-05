use bevy::prelude::*;

use crate::kiddyboids::Direction;
use crate::kiddyboids::MousePosition;

pub fn sprite_movement(
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    mouse_position: Res<MousePosition>,
) {
    for (_, mut transform) in &mut sprite_position {
        transform.translation.y = mouse_position.y - 300.;
        transform.translation.x = mouse_position.x - 400.;
    }
}
