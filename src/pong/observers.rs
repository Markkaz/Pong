use bevy::prelude::*;
use crate::game::settings::PlayerType;
use crate::game::states::GameState;
use super::components::{Ball, Pong, ScoreField};
use super::resources::Score;

#[derive(Event, Debug)]
pub struct OnPointScored(pub Entity);

#[derive(Event)]
pub struct AfterPointScored;

pub fn score_point(
    trigger: Trigger<OnPointScored>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    walls: Query<&ScoreField, Without<PlayerType>>,
) {
    if let Ok(wall) = walls.get(trigger.0) {
        score.add_point(wall);
        commands.trigger(AfterPointScored);
    }
    }

pub fn reset_ball(
    _: Trigger<OnPointScored>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ball_entity: Query<Entity, With<Ball>>,
    pong_entity: Query<Entity, With<Pong>>,
) {
    commands.entity(ball_entity.single()).despawn();

    let pong = pong_entity.single();
    commands.entity(pong).with_children(|parent| {
        super::setup::spawn_ball(parent, &mut meshes, &mut materials);
    });
}

pub fn end_game(
    _: Trigger<AfterPointScored>,
    score: Res<Score>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if score.is_game_end() {
        next_state.set(GameState::Endgame);
    }
}