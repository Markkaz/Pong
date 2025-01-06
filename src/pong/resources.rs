use bevy::prelude::*;
use crate::pong::constants::game::MAX_SCORE;
use super::components::ScoreField;

#[derive(Resource)]
pub struct Score {
    player1: u32,
    player2: u32,
}

impl Score {
    pub fn reset(&mut self) {
        self.player1 = 0;
        self.player2 = 0;
    }

    pub fn add_point(&mut self, field: &ScoreField) {
        match field {
            ScoreField::Right => self.player1 += 1,
            ScoreField::Left => self.player2 += 1,
        }
    }

    pub fn display_text(&self) -> String {
        format!("{} - {}", self.player1, self.player2)
    }

    pub fn is_game_end(&self) -> bool {
        self.player1 >= MAX_SCORE || self.player2 >= MAX_SCORE
    }

    pub fn get_winner(&self) -> String {
        if self.player1 >= MAX_SCORE { "Player 1".into() }
        else { "Player 2".into() }
    }
}

impl Default for Score {
    fn default() -> Self {
        Self {
            player1: 0,
            player2: 0,
        }
    }
}