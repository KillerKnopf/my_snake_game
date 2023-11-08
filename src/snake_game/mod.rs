use bevy::{prelude::*, render::camera::ScalingMode};

pub mod background;
pub mod food;
pub mod snake;

pub struct SnakeGamePlugin {}

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);

    commands.spawn(camera);

    let texture = asset_server.load("background.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Name::new("Background"),
    ));
}
