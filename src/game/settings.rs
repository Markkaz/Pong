use bevy::prelude::*;

#[derive(Resource, Default, PartialEq, Copy, Clone)]
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