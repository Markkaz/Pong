mod pong;
mod game;
mod ui;

use bevy::prelude::*;
use bevy::window::PresentMode;

use game::GamePlugin;
use pong::PongPlugin;
use ui::MenuSystemsPlugin;

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            present_mode: PresentMode::AutoNoVsync,
                            ..default()
                        }
                    ),
                    ..default()
                }
            )
        )
        .add_plugins((
            GamePlugin,
            MenuSystemsPlugin,
            PongPlugin,
        ))
        .add_systems(Startup, create_camera)
        .run();
}