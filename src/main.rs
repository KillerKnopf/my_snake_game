use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::Cursor,
};

mod snake_game;

fn main() {
    App::new()
        // CustomDefaultPLugins is a customized initialization of DefaultPlugins
        .add_plugins(CustomDefaultPlugins {})
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
                            grab_mode: bevy::window::CursorGrabMode::Confined,
                            ..default()
                        },
                        // Vsync option, here "Fast Vsync"
                        present_mode: bevy::window::PresentMode::Mailbox,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (1280.0, 960.0).into(),
                        title: "Bevy Base Project".into(),
                        resizable: false,
                        focused: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    // Enables different logging Levels for different systems
                    filter: "wgpu=error,bevY_render=info,bevy_ecs=trace".to_string(),
                    // Default loggging level
                    level: Level::DEBUG,
                }),
        );
    }
}
