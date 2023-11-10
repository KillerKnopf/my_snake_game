// Uncomment if unused warnings should not be raised when coding or compiling (don't do this for release).
//#![allow(unused)]

use self::snake::SnakePlugin;
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

pub struct SnakeGamePlugin {}

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(SnakePlugin {})
            .add_systems(Startup, setup_game);
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

impl GameState {
    fn next(&self) -> Self {
        match *self {
            // Any MainMenu leads to InGame because any of them either starts or restarts the game or quits the application.
            GameState::InMenu => GameState::InGame,
            // GameState while game is being played.
            GameState::InGame => GameState::Paused,
            // Gamestate if user paused the game.
            GameState::Paused => GameState::InGame,
        }
    }
}

// System to load general game stuff on startup
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Split into two to prepare space for possible, future alterations.
    let camera = Camera2dBundle::default();

    commands.spawn(camera);

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
