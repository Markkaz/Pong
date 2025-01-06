use bevy::prelude::*;

#[derive(Component)]
pub struct Pong;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub enum ScoreField {
    Left,
    Right,
}