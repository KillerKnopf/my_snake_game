use bevy::{
    input::common_conditions::input_toggle_active,
    log::{Level, LogPlugin},
    prelude::*,
    window::Cursor,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use snake_game::SnakeGamePlugin;

mod snake_game;

fn main() {
    App::new()
        // CustomDefaultPLugins is a customized initialization of DefaultPlugins
        .add_plugins(CustomDefaultPlugins {})
        .add_plugins(CameraPlugin {})
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::F3)))
        .add_plugins(SnakeGamePlugin {})
        .run();
}

struct CustomDefaultPlugins {}

impl Plugin for CustomDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        cursor: Cursor {
                            icon: CursorIcon::Crosshair,
                            visible: true,
                            // Lock cursor in window
                            // grab_mode: bevy::window::CursorGrabMode::Confined,
                            ..default()
                        },
                        // Vsync option, here "Fast Vsync"
                        present_mode: bevy::window::PresentMode::Mailbox,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (1216.0, 1216.0).into(),
                        title: "Snake".into(),
                        resizable: false,
                        focused: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    // Enables different logging Levels for different systems
                    filter: "wgpu=error,bevy_render=error,bevy_ecs=trace".to_string(),
                    // Default loggging level
                    level: Level::INFO,
                }),
        );
    }
}

#[derive(Component)]
pub struct MainCamera {}

fn initialize_camera(mut commands: Commands) {
    // Split into two to prepare space for possible, future alterations.
    let camera = Camera2dBundle::default();

    // Creates a main camera for the case that there may be mor cameras used
    commands.spawn((camera, MainCamera {}));
}

pub struct CameraPlugin {}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
    }
}
