// Uncomment if unused warnings should not be raised when compiling or coding.
//#![allow(unused)]

use bevy::prelude::*;

use super::GameState;
// Here is all code for the different uis

pub struct UIPlugin {}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>().add_systems(
            Update,
            display_start_up_menu
                // Check if following is true:
                // Chain run_if calls to AND run conditions.
                // .run_if(condition).run_if(not(condition) if you want to AND an exclusion)
                .run_if(in_state(GameState::InMenu))
                .run_if(in_state(MenuState::Startup)),
        );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum MenuState {
    // State of the displayed menu on initial startup.
    #[default]
    Startup,
    // State of the displayed menu of main menu after the player has won.
    _Victory,
    // State of the displayed menu of main menu after the player lost.
    _GameOver,
}

// ---------- Placeholders for the systems running during the different menu states ----------

// System running while in GameState::InMenu and in MenuState::Startup.
fn display_start_up_menu(input: Res<Input<KeyCode>>, current_state: Res<State<GameState>>) {
    // Currently only here for testing and to not immediately start the snake upon opening the game
    if input.any_just_pressed([KeyCode::Space, KeyCode::A, KeyCode::D]) {
        current_state.next();
    }
}

// System running while in GameState::InMenu and in MenuState::Victory.
fn _display_victory_screen() {}

// System running while in GameState::InMenu and in MenuState::GameOver.
fn _display_gameover_screen() {}
