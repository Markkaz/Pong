mod menu;
mod pong;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::menu::MenuPlugin;
use crate::pong::PongPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Settings,
    Playing,
    Paused,
    Exit,
}

#[derive(Resource)]
struct RootEntity(Entity);

fn despawn_state(mut commands: Commands, root: Res<RootEntity>) {
    commands.entity(root.0).despawn_recursive();
}

fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy_framepace::FramepacePlugin)
        .init_state::<GameState>()
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            MenuPlugin,
            PongPlugin,
        ))
        .add_systems(Startup, create_camera)
        .run();
}