use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use leafwing_input_manager::prelude::*;

use crate::game::{
    states::{
        GameState, PausedState,
        MainSet, SettingsSet, ControlsSet, PlayingSet, PausedSet
    },
    controls::{GameAction, ControlRemapping, listen_for_keys},
    settings::Difficulty,
};

use crate::ui::menu::{
    builder::MenuBuilder,
    components::{MenuButton, MenuLabel, MenuLayoutHorizontal, MenuSelectableLabel},
    actions::{ChangeStateMenuAction, QuitMenuAction, UpdateResourceMenuAction}
};

fn main_menu(mut commands: Commands, contexts: EguiContexts) {
    let builder = MenuBuilder::new("Pong!");
    builder.add_component(
        MenuButton::new("Start Game", ChangeStateMenuAction::new(GameState::Playing)),
    ).add_component(
        MenuButton::new("Settings", ChangeStateMenuAction::new(GameState::Settings)),
    ).add_component(
        MenuButton::new("Quit Game", QuitMenuAction),
    ).build(contexts, &mut commands);
}

fn settings_menu(
    mut commands: Commands,
    contexts: EguiContexts,
    difficulty: ResMut<Difficulty>,
) {
    let builder = MenuBuilder::new("Settings");
    builder.add_component(
        MenuLayoutHorizontal::new()
            .add_component(MenuLabel::new("Difficulty: "))
            .add_component(
                MenuSelectableLabel::new(
                    "Easy",
                    difficulty.as_ref() == &Difficulty::Easy,
                    UpdateResourceMenuAction::new(Difficulty::Easy)
                )
            )
            .add_component(
                MenuSelectableLabel::new(
                    "Difficult",
                    difficulty.as_ref() == &Difficulty::Difficult,
                    UpdateResourceMenuAction::new(Difficulty::Difficult)
                )
            )
            .add_component(
                MenuSelectableLabel::new(
                    "Impossible",
                    difficulty.as_ref() == &Difficulty::Impossible,
                    UpdateResourceMenuAction::new(Difficulty::Impossible)
                )
            )
    ).add_component(
        MenuButton::new("Controls", ChangeStateMenuAction::new(GameState::Controls)),
    ).add_component(
        MenuButton::new("Back", ChangeStateMenuAction::new(GameState::Main)),
    ).build(contexts, &mut commands);
}

fn init_controls_menu(mut commands: Commands) {
    commands.insert_resource(ControlRemapping::default());
}

fn destroy_controls_menu(mut commands: Commands) {
    commands.remove_resource::<ControlRemapping>();
}

fn controls_menu(
    mut commands: Commands,
    contexts: EguiContexts,
    keys: Res<InputMap<GameAction>>,
) {
    let editable_controls = [GameAction::Up, GameAction::Down, GameAction::Menu];

    let mut builder = MenuBuilder::new("Controls");

    for control in editable_controls {
        let current_keys = keys.get(&control)
            .map(|key_set| {
                key_set.iter().filter_map(|key| {
                    match key {
                        UserInputWrapper::Button(button) => Some(format!("{:?}", button)),
                        _ => None,
                    }
                }).collect::<Vec<String>>().join(", ")
            }).unwrap_or_else(|| "[Not Set]".to_string());

        builder = builder.add_component(
            MenuLayoutHorizontal::new()
                .add_component(MenuLabel::new(format!("{:?}", control)))
                .add_component(MenuButton::new(current_keys, UpdateResourceMenuAction::new(ControlRemapping::start_remapping(control)))),
        );
    }

    builder.add_component(
        MenuButton::new("Back", ChangeStateMenuAction::new(GameState::Settings)),
    ).build(contexts, &mut commands);
}

fn toggle_pause_game(
    keys: Res<ActionState<GameAction>>,
    state: Res<State<PausedState>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    if keys.just_pressed(&GameAction::Menu) {
        match state.get() {
            PausedState::Playing => next_state.set(PausedState::Paused),
            PausedState::Paused => next_state.set(PausedState::Playing),
        }
    }
}

fn paused_menu(mut commands: Commands, contexts: EguiContexts)  {
    let builder = MenuBuilder::new("Paused");
    builder.add_component(
        MenuButton::new("Resume", ChangeStateMenuAction::new(PausedState::Playing))
    ).add_component(
        MenuButton::new("Main Menu", ChangeStateMenuAction::new(GameState::Main)),
    ).build(contexts, &mut commands);
}

pub struct MenuSystemsPlugin;

impl Plugin for MenuSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(OnEnter(GameState::Controls), init_controls_menu)
            .add_systems(OnExit(GameState::Controls), destroy_controls_menu)
            .add_systems(Update, (
                main_menu.in_set(MainSet),
                settings_menu.in_set(SettingsSet),
                (controls_menu, listen_for_keys).in_set(ControlsSet),
                toggle_pause_game.in_set(PlayingSet),
                (toggle_pause_game, paused_menu).in_set(PausedSet)
            ));
    }
}