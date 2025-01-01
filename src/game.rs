pub mod states;
pub mod controls;
pub mod settings;
pub mod commands;

use bevy::prelude::*;
use controls::GameControlsPlugin;
use states::GameStatesPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                GameStatesPlugin,
                GameControlsPlugin,
            ));
    }
}