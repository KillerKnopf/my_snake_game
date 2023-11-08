use bevy::prelude::*;

pub mod background;
pub mod food;
pub mod snake;

pub struct SnakeGamePlugin {}

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game);
    }
}

fn setup_game(mut _commands: Commands) {}
