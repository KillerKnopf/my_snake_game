// Uncomment if unused warnings should not be raised when compiling or coding.
//#![allow(unused)]

use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use std::f32::consts::PI;

use super::GameState;

// Module to organise all code relating to the snake

pub struct SnakePlugin {}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            // Only spawn snake when coming from menus because in this case a new game starts.
            OnTransition {
                from: GameState::InMenu,
                to: GameState::InGame,
            },
            spawn_snake,
        )
        // These systems are the gameplay systems, so run during GameState::InGame.
        .add_systems(Update, move_snake.run_if(in_state(GameState::InGame)))
        .add_systems(Update, turn_snake.run_if(in_state(GameState::InGame)))
        .add_systems(Update, pause_game.run_if(in_state(GameState::InGame)))
        // The systems running while the game is paused.
        .add_systems(Update, unpause_game.run_if(in_state(GameState::Paused)))
        // Register types for reflection aka enabling the logging of the data of this data type.
        .register_type::<Snake>();
    }
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Snake {
    speed: f32,
}

// This system readies the snake so that the game can run without issues.
// In future this also spawns the tail and initializes this snakes score and max length target.
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
        Snake { speed: 400.0 },
        Name::new("Snake"),
    ));
}

// Refactor this so that snake only turns at the center of a background cell as if the snake is moving on a grid.
// Maybe implement this using SnakeState::TurningLeft, ::TurningRight and MovingForward.
// Where it only this sytem checks if the snake is in the middle of a "grid" if it is in state turning.
// And during the frame it hits a "grid" center the snake actually turns and changes state to ::MovingForward
fn turn_snake(mut snakes: Query<&mut Transform, With<Snake>>, input: Res<Input<KeyCode>>) {
    let mut snake_transform = snakes.single_mut();

    // Rotate snake by +-90 degrees on player input.
    if input.just_pressed(KeyCode::A) {
        snake_transform.rotate_z(PI * 0.5);
    }
    if input.just_pressed(KeyCode::D) {
        snake_transform.rotate_z(PI * -0.5);
    }
}

// Gets forward direction of snake from it's rotation.
// Then moves snake an appropriate amount in this direction.
fn move_snake(mut snakes: Query<(&mut Transform, &Snake)>, time: Res<Time>) {
    let (mut transform, snake) = snakes.single_mut();

    // Calculate how much to move the snake
    let movement_amount = snake.speed * time.delta_seconds();

    // Get the snakes direction
    let direction = transform.rotation * Vec3::Y;

    // Lenghten direction by movement amount
    let vector_to_new_pos = direction * movement_amount;

    // Move snake to new position
    transform.translation += vector_to_new_pos;
}

// This system checks if the snake is colliding with:
// 1 - the field edges
// 2 - itself
// 3 - food
// 1 and 2 changes GameState to ::GameOver
// 2 needs memory to store the positions of snake segements, maybe child entities with respective components?
// 3 grows snake
fn _detect_collision() {}

// Not a system (function that is placed in the Scheduler); maybe called it as an event handler?
fn _grow_snake() {}

// Waits for user input to pause the game. Runs during GameState::InGame.
// Sets GameState to ::Paused.
fn pause_game(input: Res<Input<KeyCode>>, current_state: Res<State<GameState>>) {
    // Change GameState from ::InGame to ::Paused if player presses the Escape key while playing.
    if input.just_pressed(KeyCode::Escape) {
        current_state.next();
    }
}

// Waits for user input to pause the game. Runs during GameState::Paused.
// Sets GameState to ::InGame.
fn unpause_game(input: Res<Input<KeyCode>>, current_state: Res<State<GameState>>) {
    // Change GameState from ::Paused to ::InGame if player presses the Escape key while the game is paused.
    if input.just_pressed(KeyCode::Escape) {
        current_state.next();
    }
}
