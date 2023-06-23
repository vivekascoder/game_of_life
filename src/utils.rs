use bevy::prelude::*;

pub const TILE_SIZE: f32 = 5.0;
pub const NUMBER_OF_TILES: f32 = 100.0;
pub const TOTAL_CELL: f32 = TILE_SIZE * NUMBER_OF_TILES;
pub const CELL_SET_COLOR: Color = Color::RED;
pub const CELL_UNSET_COLOR: Color = Color::WHITE;

pub fn get_cell_from_position(position: Vec2) -> (f32, f32) {
    (
        (position.x / TILE_SIZE).floor() as f32,
        (position.y / TILE_SIZE).floor() as f32,
    )
}
