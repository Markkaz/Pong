use bevy::prelude::*;
use super::components::ScoreField;

#[derive(Resource)]
pub struct Score {
    player: u32,
    computer: u32,
}

impl Score {
    pub fn reset(&mut self) {
        self.player = 0;
        self.computer = 0;
    }

    pub fn add_point(&mut self, field: &ScoreField) {
        match field {
            ScoreField::Left => self.player += 1,
            ScoreField::Right => self.computer += 1,
        }
    }

    pub fn display_text(&self) -> String {
        format!("{} - {}", self.player, self.computer)
    }
}

impl Default for Score {
    fn default() -> Self {
        Self {
            player: 0,
            computer: 0,
        }
    }
}