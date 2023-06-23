use crate::constants::{CELL_SET_COLOR, CELL_UNSET_COLOR, NUMBER_OF_TILES, TILE_SIZE, TOTAL_CELL};
use crate::utils::get_cell_from_position;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

#[derive(Component)]
pub struct Tile {
    x: f32,
    y: f32,
    is_set: bool,
}

#[derive(Resource, Debug)]
pub enum CurrentGameState {
    Playing,
    Paused,
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
                    for (mut material, mut tile) in cells_query.iter_mut() {
                        if tile.x == row && tile.y == col {
                            tile.is_set = !tile.is_set;
                            *material = materials.add(ColorMaterial::from(match tile.is_set {
                                true => CELL_SET_COLOR,
                                false => CELL_UNSET_COLOR,
                            }));
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
        let mut row = ((i + 1) % NUMBER_OF_TILES as usize) as f32;
        if i + 1 == TOTAL_CELL as usize {
            row = 100.0;
        }
        let mut col = 1.0;
        if i + 1 > NUMBER_OF_TILES as usize {
            col = ((i + 1) as f32 / NUMBER_OF_TILES).ceil();
        }

        println!("Row, Col: {}, {}", row, col);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Box::new(TILE_SIZE, TILE_SIZE, 0.0).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
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
