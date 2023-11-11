// Here is all code or sub modules concerning custom logging.
// This submodule is only include in debug builds.
use super::{snake::SnakeState, ui::MenuState, GameState};
use bevy::prelude::*;

pub struct MyLoggingPlugin;

impl Plugin for MyLoggingPlugin {
    fn build(&self, app: &mut App) {
        // Logging statements
        app.add_systems(Update, log_game_state_if_changed)
            .add_systems(Update, log_snake_state_if_changed)
            .add_systems(Update, log_menu_state_if_changed);
    }
}

// Logs the GameState if it has changed.
fn log_game_state_if_changed(game_state: Res<State<GameState>>) {
    if game_state.is_changed() {
        info!("Changed GameState. New state: {:?}", game_state.get());
    }
}

// Logs the SnakeState if it has changed.
fn log_snake_state_if_changed(snake_state: Res<State<SnakeState>>) {
    if snake_state.is_changed() {
        info!("Changed SnakeState. New state: {:?}", snake_state.get());
    }
}

// Logs the MenuState if it has changed.
fn log_menu_state_if_changed(menu_state: Res<State<MenuState>>) {
    if menu_state.is_changed() {
        info!("Changed MenuState. New state: {:?}", menu_state.get());
    }
}
