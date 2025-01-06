use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Main,
    Controls,
    Startgame,
    Playing,
    Endgame,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Playing,
    Paused,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ControlsSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StartGameSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayingSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PausedSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EndgameSet;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_state::<PausedState>()
            .configure_sets(Update, (
                MainSet.run_if(in_state(GameState::Main)),
                ControlsSet.run_if(in_state(GameState::Controls)),
                StartGameSet.run_if(in_state(GameState::Startgame)),
                PlayingSet
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PausedState::Playing)),
                PausedSet
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PausedState::Paused)),
                EndgameSet.run_if(in_state(GameState::Endgame)),
            ))
            .configure_sets(FixedUpdate,
                PlayingSet
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PausedState::Playing)),
            );
    }
}