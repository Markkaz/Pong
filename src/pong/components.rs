use bevy::prelude::*;

#[derive(Component)]
pub struct Pong;

#[derive(Component)]
pub enum Paddle {
    Player,
    Computer,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub enum ScoreField {
    Left,
    Right,
}

#[derive(Event)]
pub struct ScorePointEvent(pub Entity);