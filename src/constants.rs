use bevy::prelude::*;

pub const TILE_SIZE: f32 = 10.0;
pub const NUMBER_OF_TILES: f32 = 100.0;
pub const TOTAL_CELL: f32 = TILE_SIZE * NUMBER_OF_TILES;
pub const CELL_SET_COLOR: Color = Color::WHITE;
pub const CELL_UNSET_COLOR: Color = Color::BLACK;
pub const SIMULATION_TICK_TIME: f32 = 0.01;
