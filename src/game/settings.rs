use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    player1: PlayerType,
    player2: PlayerType,
}

impl GameSettings {
    pub fn get_player1(&self) -> &PlayerType {
        &self.player1
    }

    pub fn get_player2(&self) -> &PlayerType {
        &self.player2
    }

    pub fn update_players(&mut self, player_num: usize, player_type: PlayerType) {
        match player_num {
            1 => self.player1 = player_type,
            2 => self.player2 = player_type,
            _ => panic!("Invalid player num {}", player_num),
        }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player1: PlayerType::Human,
            player2: PlayerType::Computer(Difficulty::Easy),
        }
    }
}

#[derive(Component, PartialEq, Copy, Clone)]
pub enum PlayerType {
    Human,
    Computer(Difficulty),
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum Difficulty {
    #[default]
    Easy,
    Difficult,
    Impossible,
}

impl Difficulty {
    pub fn speed(&self) -> f32 {
        match self {
            Difficulty::Easy => 2.,
            Difficulty::Difficult => 4.,
            Difficulty::Impossible => 6.,
        }
    }
}