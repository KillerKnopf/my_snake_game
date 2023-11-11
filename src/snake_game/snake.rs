// Uncomment if unused warnings should not be raised when compiling or coding.
//#![allow(unused)]

use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use std::f32::consts::PI;

use super::GameState;

// Module to organise all code relating to the snake.

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<SnakeState>()
            .add_systems(
                // Only spawn snake when coming from menus because in this case a new game starts.
                OnTransition {
                    from: GameState::InMenu,
                    to: GameState::InGame,
                },
                spawn_snake,
            )
            // These systems are the gameplay systems, so run during GameState::InGame.
            .add_systems(Update, move_snake.run_if(in_state(GameState::InGame)))
            .add_systems(Update, pause_game.run_if(in_state(GameState::InGame)))
            // Additionaly don't detect turning input when a turning command is already queued.
            // A queued turning command is signified by SnakeState not being ::Normal.
            .add_systems(
                Update,
                detect_turn_input
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(SnakeState::Normal)),
            )
            // The turning systems are running during GameState::InGame and their respective SnakeStates (::TurningLeft or ::TurningRight).
            .add_systems(Update, turn_snake_left.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                turn_snake_left.run_if(in_state(SnakeState::TurningLeft)),
            )
            .add_systems(Update, turn_snake_right.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                turn_snake_right.run_if(in_state(SnakeState::TurningRight)),
            )
            // The systems running while the game is paused.
            .add_systems(Update, unpause_game.run_if(in_state(GameState::Paused)))
            // Register types for reflection aka enabling the logging of the data of this data type.
            .register_type::<Snake>();
    }
}

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
struct Snake {
    speed: f32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum SnakeState {
    // Normal state where the game is awaiting turn input.
    #[default]
    Normal,
    // The state in which the game is looking for a suitable place (if snake is in grid center) to turn the snake left.
    TurningLeft,
    // The state in which the game is looking for a suitable place (if snake is in grid center) to turn the snake right.
    TurningRight,
}

// Only runs when transitioning from GameState::InMenu to ::InGame because at that point a new game starts.
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

// Runs during GameState::InGame and SnakeState::Normal.
// Checks whether the user has pressed 'A' or 'D' and if so set the SnakeState to TurningLeft or TurningRight.
fn detect_turn_input(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<SnakeState>>) {
    if input.just_pressed(KeyCode::A) {
        next_state.set(SnakeState::TurningLeft);
    }
    if input.just_pressed(KeyCode::D) {
        next_state.set(SnakeState::TurningRight);
    }
}

// Runs during GameState::InGame and SnakeState::TurningLeft and turns the snake to the left if it is at a grid center.
fn turn_snake_left(
    mut snakes: Query<&mut Transform, With<Snake>>,
    mut next_state: ResMut<NextState<SnakeState>>,
) {
    let mut snake_transform = snakes.single_mut();

    // Check if snake is at "grid" center. If not leave early.
    if !snake_is_at_grid_center(snake_transform.translation) {
        return;
    }

    // Rotate snake to the left by 90 degrees.
    snake_transform.rotate_z(PI * 0.5);

    // Also snap snake to grid center coordinates
    info!("Snake turned at: {:?}", snake_transform.translation);

    // Reset SnakeState to ::Normal.
    next_state.set(SnakeState::Normal);
}

// Runs during GameState::InGame and SnakeState::TurningRight and turns the snake to the right if it is at a grid center.
fn turn_snake_right(
    mut snakes: Query<&mut Transform, With<Snake>>,
    mut next_state: ResMut<NextState<SnakeState>>,
) {
    let mut snake_transform = snakes.single_mut();

    // Check if snake is at "grid" center. If not leave early.
    if !snake_is_at_grid_center(snake_transform.translation) {
        return;
    }

    // Rotate snake to the right by 90 degrees.
    snake_transform.rotate_z(PI * -0.5);

    // Also snap snake to grid center coordinates
    info!("Snake turned at: {:?}", snake_transform.translation);

    // Reset SnakeState to ::Normal.
    next_state.set(SnakeState::Normal);
}

// This function determines if the snakes position aligns with a "grid" (actually cell of a grid) center.
// The grid doesn't actually exist but is visually present on the background.
// Grid centers have x- and y-coordinates equal to a multiple of 64.
fn snake_is_at_grid_center(snake_translation: Vec3) -> bool {
    // It's really really rare for the snake to be exactly at a*64.0 coordinates so a bit of inprecision is needed.
    let x_check = (snake_translation.x % 64.0) < 0.1;
    let y_check = (snake_translation.y % 64.0) < 0.1;

    x_check && y_check
}

// Runs during GameState::InGame.
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
fn pause_game(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    // Change GameState from ::InGame to ::Paused if player presses the Escape key while playing.
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

// Waits for user input to pause the game. Runs during GameState::Paused.
// Sets GameState to ::InGame.
fn unpause_game(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    // Change GameState from ::Paused to ::InGame if player presses the Escape key while the game is paused.
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame);
    }
}
