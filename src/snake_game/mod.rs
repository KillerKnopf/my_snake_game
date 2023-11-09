use bevy::prelude::*;

use self::snake::SnakePlugin;
// use bevy::render::camera::ScalingMode;

pub mod background;
pub mod food;
pub mod snake;

pub struct SnakeGamePlugin {}

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(SnakePlugin {})
            .add_systems(Startup, setup_game);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

impl GameState {
    fn next(&self) -> Self {
        match *self {
            // Waiting for start or restart
            GameState::MainMenu => GameState::InGame,
            // While Playing the game
            GameState::InGame => GameState::Paused,
            // If player pauses the game or the game has finished (Victory or Gameover)
            GameState::Paused => GameState::InGame,
        }
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = Camera2dBundle::default();
    // Not needed because resizeable is false
    // camera.projection.scaling_mode = ScalingMode::AutoMin {
    //     min_width: 1280.0,
    //     min_height: 1280.0,
    // };
    commands.spawn(camera);

    // Load background
    let texture = asset_server.load("background.png");
    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Name::new("Background"),
    ));
}
