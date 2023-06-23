use crate::constants::{
    CELL_SET_COLOR, CELL_UNSET_COLOR, NUMBER_OF_TILES, SIMULATION_TICK_TIME, TILE_SIZE, TOTAL_CELL,
};
use crate::utils::get_cell_from_position;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use rand::prelude::*;

#[derive(Component)]
pub struct Tile {
    x: f32,
    y: f32,
    is_set: bool,
}

#[derive(Resource, Debug, PartialEq)]
pub enum CurrentGameState {
    Playing,
    Paused,
}

#[derive(Resource, Debug)]
pub struct SimulationTimer {
    timer: Timer,
}

impl SimulationTimer {
    pub fn new() -> Self {
        Self {
            timer: Timer::from_seconds(SIMULATION_TICK_TIME, TimerMode::Repeating),
        }
    }
}

impl Default for SimulationTimer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn switch_game_state(mut game_state: ResMut<CurrentGameState>, key_input: Res<Input<KeyCode>>) {
    if key_input.just_released(KeyCode::Space) {
        *game_state = match *game_state {
            CurrentGameState::Paused => CurrentGameState::Playing,
            CurrentGameState::Playing => CurrentGameState::Paused,
        };
        info!("State: {:?}", game_state);
    }
}

pub fn toggle_on_mouse_click(
    game_state: Res<CurrentGameState>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<Input<MouseButton>>,
    mut cells_query: Query<(&mut Handle<ColorMaterial>, &mut Tile)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    match *game_state {
        CurrentGameState::Paused => {
            if mouse_button.just_pressed(MouseButton::Left) {
                // Get the mouse cursor
                if let Some(position) = window_query.single().cursor_position() {
                    info!("Mouse position: {:?}", position);
                    let (row, col) = get_cell_from_position(position);
                    info!("Cell: {} {}", row, col);
                    for (mut material, mut tile) in cells_query.iter_mut() {
                        if tile.x == row && tile.y == col {
                            tile.is_set = !tile.is_set;
                            *material = materials.add(ColorMaterial::from(match tile.is_set {
                                true => CELL_SET_COLOR,
                                false => CELL_UNSET_COLOR,
                            }));
                            info!("Tile is_set: {}", tile.is_set);
                        }
                    }
                } else {
                    error!("Cursor is not in the game window.");
                }
            }
        }
        CurrentGameState::Playing => {}
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add state resource
    commands.insert_resource(CurrentGameState::Paused);
    for i in 0..(NUMBER_OF_TILES * NUMBER_OF_TILES) as usize {
        let row = (i % NUMBER_OF_TILES as usize) as f32;
        let col = (i / NUMBER_OF_TILES as usize) as f32;

        info!("Row, Col: {}, {}", row, col);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Box::new(TILE_SIZE, TILE_SIZE, 0.0).into())
                    .into(),
                material: materials.add(ColorMaterial::from(CELL_UNSET_COLOR)),
                transform: Transform::from_xyz(row * TILE_SIZE, col * TILE_SIZE, 0.0),
                ..default()
            },
            Tile {
                x: row,
                y: col,
                is_set: false,
            },
        ));
    }
}

pub fn randomize(
    mut cells_query: Query<(&mut Handle<ColorMaterial>, &mut Tile)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_game_state: Res<CurrentGameState>,
    key_input: Res<Input<KeyCode>>,
) {
    if *current_game_state == CurrentGameState::Paused {
        if key_input.just_released(KeyCode::R) {
            for (mut material, mut tile) in cells_query.iter_mut() {
                tile.is_set = rand::thread_rng().gen_bool(1.0 / 3.0);
                *material = materials.add(ColorMaterial::from(match tile.is_set {
                    true => CELL_SET_COLOR,
                    false => CELL_UNSET_COLOR,
                }));
            }
        }
    }
}

pub fn simulate(
    mut cells_query: Query<(&mut Handle<ColorMaterial>, &mut Tile)>,
    mut simulation_timer: ResMut<SimulationTimer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_game_state: Res<CurrentGameState>,
    time: Res<Time>,
) {
    if simulation_timer.timer.tick(time.delta()).just_finished()
        && *current_game_state == CurrentGameState::Playing
    {
        let mut life_grid: Vec<bool> = Vec::new();

        for (_, tile) in cells_query.iter_mut() {
            life_grid.push(tile.is_set);
        }

        for (index, (mut material, mut tile)) in cells_query.iter_mut().enumerate() {
            let mut neighbour_count = 0;
            let x = index as i32 % NUMBER_OF_TILES as i32;
            let y = index as i32 / NUMBER_OF_TILES as i32;

            for xi in (x - 1)..(x + 2) {
                for yi in (y - 1)..(y + 2) {
                    if (xi != x || yi != y)
                        && xi >= 0
                        && xi < NUMBER_OF_TILES as i32
                        && yi >= 0
                        && yi < NUMBER_OF_TILES as i32
                    {
                        let line_index = xi + yi * NUMBER_OF_TILES as i32;
                        if life_grid[line_index as usize] == true {
                            neighbour_count += 1;
                        }
                    }
                }
            }

            if neighbour_count < 2 || neighbour_count > 3 {
                if tile.is_set {
                    (*tile).is_set = false;
                    *material = materials.add(ColorMaterial::from(match tile.is_set {
                        true => CELL_SET_COLOR,
                        false => CELL_UNSET_COLOR,
                    }));
                }
            }

            if neighbour_count == 3 {
                (*tile).is_set = true;
                *material = materials.add(ColorMaterial::from(match tile.is_set {
                    true => CELL_SET_COLOR,
                    false => CELL_UNSET_COLOR,
                }));
            }
        }
    }
}
