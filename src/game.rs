pub mod states;
pub mod controls;
pub mod settings;

use bevy::prelude::*;
use controls::GameControlsPlugin;
use settings::Difficulty;
use states::GameStatesPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                GameStatesPlugin,
                GameControlsPlugin,
            ))
            .init_resource::<Difficulty>();
    }
}