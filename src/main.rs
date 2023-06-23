use bevy::prelude::*;
use game_of_life::{
    constants::{NUMBER_OF_TILES, TILE_SIZE},
    simulation::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                resolution: (TILE_SIZE * NUMBER_OF_TILES, TILE_SIZE * NUMBER_OF_TILES).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<SimulationTimer>()
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_system(switch_game_state)
        .add_system(toggle_on_mouse_click)
        .add_system(simulate)
        .run();
}
