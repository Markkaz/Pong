mod components;
mod systems;
mod resources;
mod constants;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::states::{GameState, PausedState, PlayingSet};
use crate::pong::components::{ScorePointEvent};
use crate::pong::resources::Score;
use crate::pong::systems::*;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .configure_sets(FixedUpdate, (
                PhysicsSet::StepSimulation
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PausedState::Playing)),
            ))

            .add_event::<ScorePointEvent>()

            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())

            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), cleanup_game)
            .add_systems(FixedUpdate, move_players.in_set(PlayingSet))
            .add_systems(Update, (
                speed_up_ball,
                detect_point,
                score_point,
                update_score_display,
                reset_ball,
            ).in_set(PlayingSet));
    }
}