use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

pub struct SnakePlugin {}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostStartup, spawn_snake)
            .add_systems(Update, (start_snake, stop_snake, move_snake, turn_snake))
            .register_type::<Snake>();
    }
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Snake {
    is_moving: bool,
    speed: f32,
}

fn spawn_snake(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load background
    let texture = asset_server.load("snake_v1.png");
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(-192.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Snake {
            is_moving: false,
            speed: 400.0,
        },
        Name::new("Snake"),
    ));
}

fn turn_snake(mut snakes: Query<&mut Transform, With<Snake>>, input: Res<Input<KeyCode>>) {
    let mut snake_transform = snakes.single_mut();

    if input.just_pressed(KeyCode::A) {
        snake_transform.rotate_z(PI * 0.5);
    }

    if input.just_pressed(KeyCode::D) {
        snake_transform.rotate_z(PI * -0.5);
    }
}

fn move_snake(mut snakes: Query<(&mut Transform, &Snake)>, time: Res<Time>) {
    let (mut transform, snake) = snakes.single_mut();

    // Snake isn't moving if it hasn't started yet
    // so do nothing
    if !snake.is_moving {
        return;
    }

    // Calculate how much to move the snake
    let movement_amount = snake.speed * time.delta_seconds();

    // Get the snakes direction
    let direction = transform.rotation * Vec3::Y;

    // Lenghten direction by movement amount
    let vector_to_new_pos = direction * movement_amount;

    // Move snake to new position
    transform.translation += vector_to_new_pos;
}

fn start_snake(mut snake: Query<&mut Snake>, input: Res<Input<KeyCode>>) {
    let mut snake = snake.single_mut();

    // If snake is already moving then do nothing
    if snake.is_moving {
        return;
    }

    // If it isn't moving start moving it when either 'a' or 'd' is pressed
    if input.any_just_pressed([KeyCode::A, KeyCode::D]) {
        snake.is_moving = true;
    }
}

#[cfg(debug_assertions)]
fn stop_snake(mut snake: Query<&mut Snake>, input: Res<Input<KeyCode>>) {
    let mut snake = snake.single_mut();

    // If snake is stopped then do nothing
    if !snake.is_moving {
        return;
    }

    // If it is moving then stop it when space is pressed
    if input.just_pressed(KeyCode::Space) {
        snake.is_moving = false;
    }
}
