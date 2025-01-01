use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin};
use leafwing_input_manager::prelude::*;

use crate::game::{
    states::{
        GameState, PausedState, StartGameSet,
        MainSet, ControlsSet, PlayingSet, PausedSet
    },
    controls::{GameAction, ControlRemapping, listen_for_keys},
    settings::{GameSettings, Difficulty, PlayerType},
};
use crate::game::commands::UpdatePlayerCommand;
use crate::ui::menu::{
    builder::MenuBuilder,
    components::{MenuButton, MenuLabel, MenuLayoutHorizontal, MenuSelectableLabel},
    actions::{ChangeStateMenuAction, QuitMenuAction, UpdateResourceMenuAction, CommandMenuAction}
};

fn main_menu(mut commands: Commands, contexts: EguiContexts) {
    let builder = MenuBuilder::new("Pong!");
    builder.add_component(
        MenuButton::new("Start Game", ChangeStateMenuAction::new(GameState::Startgame)),
    ).add_component(
        MenuButton::new("Controls", ChangeStateMenuAction::new(GameState::Controls)),
    ).add_component(
        MenuButton::new("Quit Game", QuitMenuAction),
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
    let mut builder = MenuBuilder::new("Controls")
        .with_top_spacing(25.)
        .add_component(MenuLabel::new("Player 1:"));

    builder = control_selection_button(&keys, builder, GameAction::Player1Up);
    builder = control_selection_button(&keys, builder, GameAction::Player1Down);
    builder = builder.add_component(MenuLabel::new("Player 2:"));
    builder = control_selection_button(&keys, builder, GameAction::Player2Up);
    builder = control_selection_button(&keys, builder, GameAction::Player2Down);
    builder = builder.add_component(MenuLabel::new(""));

    builder = control_selection_button(&keys, builder, GameAction::Menu);

    builder.add_component(
        MenuButton::new("Back", ChangeStateMenuAction::new(GameState::Main)),
    ).build(contexts, &mut commands);
}

fn control_selection_button(keys: &Res<InputMap<GameAction>>, builder: MenuBuilder, control: GameAction) -> MenuBuilder {
    let current_keys = keys.get(&control)
        .map(|key_set| {
            key_set.iter().filter_map(|key| {
                match key {
                    UserInputWrapper::Button(button) => Some(format!("{:?}", button)),
                    _ => None,
                }
            }).collect::<Vec<String>>().join(", ")
        }).unwrap_or_else(|| "[Not Set]".to_string());

    builder.add_component(
        MenuLayoutHorizontal::new()
            .add_component(MenuLabel::new(format!("{:?}", control)))
            .add_component(MenuButton::new(current_keys, UpdateResourceMenuAction::new(ControlRemapping::start_remapping(control)))),
    )
}

fn init_start_game_menu(mut commands: Commands) {
    commands.init_resource::<GameSettings>();
}

fn start_game_menu(mut commands: Commands, contexts: EguiContexts, settings: ResMut<GameSettings>) {
    MenuBuilder::new("New Game")
        .with_top_spacing(100.)
        .add_component(MenuLabel::new("Player 1"))
        .add_component(MenuLayoutHorizontal::new()
            .add_component(MenuSelectableLabel::new(
                "Human",
                matches!(settings.get_player1(), PlayerType::Human),
                CommandMenuAction::new(UpdatePlayerCommand::new(1, PlayerType::Human))
            ))
            .add_component(MenuSelectableLabel::new(
                "Easy",
                matches!(settings.get_player1(), PlayerType::Computer(difficulty) if difficulty == &Difficulty::Easy),
                CommandMenuAction::new(UpdatePlayerCommand::new(1, PlayerType::Computer(Difficulty::Easy)))
            ))
            .add_component(MenuSelectableLabel::new(
                "Difficult",
                matches!(settings.get_player1(), PlayerType::Computer(difficulty) if difficulty == &Difficulty::Difficult),
                CommandMenuAction::new(UpdatePlayerCommand::new(1, PlayerType::Computer(Difficulty::Difficult)))
            ))
            .add_component(MenuSelectableLabel::new(
                "Impossible",
                matches!(settings.get_player1(), PlayerType::Computer(difficulty) if difficulty == &Difficulty::Impossible),
                CommandMenuAction::new(UpdatePlayerCommand::new(1, PlayerType::Computer(Difficulty::Impossible)))
            ))
        )
        .add_component(MenuLabel::new("Player 2"))
        .add_component(MenuLayoutHorizontal::new()
            .add_component(MenuSelectableLabel::new(
                "Human",
                matches!(settings.get_player2(), PlayerType::Human),
                CommandMenuAction::new(UpdatePlayerCommand::new(2, PlayerType::Human))
            ))
            .add_component(MenuSelectableLabel::new(
                "Easy",
                matches!(settings.get_player2(), PlayerType::Computer(difficulty) if difficulty == &Difficulty::Easy),
                CommandMenuAction::new(UpdatePlayerCommand::new(2, PlayerType::Computer(Difficulty::Easy)))
            ))
            .add_component(MenuSelectableLabel::new(
                "Difficult",
                matches!(settings.get_player2(), PlayerType::Computer(difficulty) if difficulty == &Difficulty::Difficult),
                CommandMenuAction::new(UpdatePlayerCommand::new(2, PlayerType::Computer(Difficulty::Difficult)))
            ))
            .add_component(MenuSelectableLabel::new(
                "Impossible",
                matches!(settings.get_player2(), PlayerType::Computer(difficulty) if difficulty == &Difficulty::Impossible),
                CommandMenuAction::new(UpdatePlayerCommand::new(2, PlayerType::Computer(Difficulty::Impossible)))
            ))
        )
        .add_component(MenuButton::new("Start Game", ChangeStateMenuAction::new(GameState::Playing)))
        .add_component(MenuButton::new("Back", ChangeStateMenuAction::new(GameState::Main)))
        .build(contexts, &mut commands);
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
            .add_systems(OnEnter(GameState::Startgame), init_start_game_menu)
            .add_systems(Update, (
                start_game_menu.in_set(StartGameSet),
                main_menu.in_set(MainSet),
                (controls_menu, listen_for_keys).in_set(ControlsSet),
                toggle_pause_game.in_set(PlayingSet),
                (toggle_pause_game, paused_menu).in_set(PausedSet)
            ));
    }
}