// Uncomment if unused warnings should not be raised when coding or compiling (don't do this for release).
//#![allow(unused)]

use self::logging::MyLoggingPlugin;

use self::{snake::SnakePlugin, ui::UiPlugin};
use bevy::prelude::*;

// This module manages all game specific systems.
// It is mainly collecting the othre modules and evaluating the different game states.

// Sub module containing everything about the snake.
// Snake visuals, snake movement, snake collision, snake growing (Incresing length and tracking score for victory).
mod snake;
// Submodule for drawing UI.
// Different states of main menu, current score(length).
mod ui;
// Sub module for the eaten food.
// Spawn rules, spawning, nutritional value (how much snake can grow from it), and possible other effects
mod food;
// Sub module containing my custom loggin systems.
// For example the systems logging State changes.
// Only included in debug build
mod logging;

pub struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            // Initializing sub module of SnakeGame.
            .add_plugins(SnakePlugin)
            .add_plugins(UiPlugin)
            // Startup systems.
            .add_systems(Startup, setup_game);

        // Plugins only added during debug.
        // Uncomment for release
        app.add_plugins(MyLoggingPlugin);
    }
}

// States of the SnakeGame.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    // Display menu from where the player can start a game, change settings or quit the application.
    InMenu,
    // State where a game of snake is running.
    InGame,
    // State of the displayed menu when the player paused the game.
    Paused,
}

// System to load general game stuff on startup
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load background in here because there is no plan to change the backgorund during the game.
    let texture = asset_server.load("background.png");
    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Name::new("Background"),
    ));
}
