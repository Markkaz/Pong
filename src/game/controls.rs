use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum GameAction {
    Up,
    Down,
    Menu,
}

impl GameAction {
    fn default_input_map() -> InputMap<GameAction> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Up, KeyCode::ArrowUp);
        input_map.insert(Self::Down, KeyCode::ArrowDown);
        input_map.insert(Self::Menu, KeyCode::Escape);

        input_map
    }
}

#[derive(Resource, Default, Clone, Copy)]
pub struct ControlRemapping {
    current_action: Option<GameAction>,
    is_listening: bool,
}

impl ControlRemapping {
    pub fn start_remapping(control: GameAction) -> Self {
        Self {
            current_action: Some(control),
            is_listening: true,
        }
    }

    pub fn stop_remapping(&mut self) {
        self.current_action = None;
        self.is_listening = false;
    }
}

pub fn listen_for_keys(
    mut mapping: ResMut<ControlRemapping>,
    mut key_map: ResMut<InputMap<GameAction>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if mapping.is_listening && mapping.current_action.is_some() {
        if let Some(control) = mapping.current_action {
            for key in keys.get_pressed() {
                key_map.clear_action(&control);
                key_map.insert(control, *key);

                mapping.stop_remapping();
            }
        }
    }
}

pub struct GameControlsPlugin;

impl Plugin for GameControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<GameAction>::default())
            .init_resource::<ControlRemapping>()
            .init_resource::<ActionState<GameAction>>()
            .init_resource::<InputMap<GameAction>>()
            .insert_resource(GameAction::default_input_map());
    }
}