use bevy::prelude::*;
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
}

impl Default for Score {
    fn default() -> Self {
        Self {
            player1: 0,
            player2: 0,
        }
    }
}