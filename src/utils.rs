use bevy::prelude::*;

use crate::constants::TILE_SIZE;

pub fn get_cell_from_position(position: Vec2) -> (f32, f32) {
    (
        (position.x / TILE_SIZE).floor() as f32,
        (position.y / TILE_SIZE).floor() as f32,
    )
}
