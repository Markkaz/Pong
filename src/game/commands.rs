use bevy::prelude::*;
use super::settings::{GameSettings, PlayerType};

#[derive(Clone)]
pub struct UpdatePlayerCommand {
    player_num: usize,
    player_type: PlayerType,
}

impl UpdatePlayerCommand {
    pub fn new(player_num: usize, player_type: PlayerType) -> Self {
        Self {
            player_num,
            player_type,
        }
    }
}

impl Command for UpdatePlayerCommand {
    fn apply(self, world: &mut World) {
        if let Some(mut settings) = world.get_resource_mut::<GameSettings>() {
            settings.update_players(self.player_num, self.player_type);
        }
    }
}