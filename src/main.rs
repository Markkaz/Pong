mod menu;
mod pong;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::pong::PongPlugin;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Main,
    Settings,
    Controls,
    Playing,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PausedState {
    #[default]
    Playing,
    Paused,
}

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
enum Controls {
    Up,
    Down,
    Menu,
}

impl Controls {
    fn default_input_map() -> InputMap<Controls> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::Up, KeyCode::ArrowUp);
        input_map.insert(Self::Down, KeyCode::ArrowDown);
        input_map.insert(Self::Menu, KeyCode::Escape);

        input_map
    }
}

#[derive(Resource, Default, PartialEq, Copy, Clone)]
enum Difficulty {
    #[default]
    Easy,
    Difficult,
    Impossible,
}

impl Difficulty {
    fn speed(&self) -> f32 {
        match self {
            Difficulty::Easy => 2.,
            Difficulty::Difficult => 4.,
            Difficulty::Impossible => 6.,
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MainSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct SettingsSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ControlsSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlayingSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PausedSet;

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn main_menu(mut commands: Commands, contexts: EguiContexts) {
    let builder = menu::MenuBuilder::new("Pong!");
    builder.add_component(
        menu::MenuButton::new("Start Game", menu::ChangeStateMenuAction::new(GameState::Playing)),
    ).add_component(
        menu::MenuButton::new("Settings", menu::ChangeStateMenuAction::new(GameState::Settings)),
    ).add_component(
        menu::MenuButton::new("Quit Game", menu::QuitMenuAction),
    ).build(contexts, &mut commands);
}

fn settings_menu(
    mut commands: Commands,
    contexts: EguiContexts,
    difficulty: ResMut<Difficulty>,
) {

    let builder = menu::MenuBuilder::new("Settings");
    builder.add_component(
        menu::MenuOptions::new("Difficulty")
            .add_option(
                "Easy",
                difficulty.as_ref() == &Difficulty::Easy,
                menu::UpdateResourceMenuAction::new(Difficulty::Easy)
            )
            .add_option(
                "Difficult",
                difficulty.as_ref() == &Difficulty::Difficult,
                menu::UpdateResourceMenuAction::new(Difficulty::Difficult)
            )
            .add_option(
                "Impossible",
                difficulty.as_ref() == &Difficulty::Impossible,
                menu::UpdateResourceMenuAction::new(Difficulty::Impossible)
            )
    ).add_component(
        menu::MenuButton::new("Back", menu::ChangeStateMenuAction::new(GameState::Main)),
    ).build(contexts, &mut commands);
}

fn toggle_pause_game(
    keys: Res<ActionState<Controls>>,
    state: Res<State<PausedState>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    if keys.just_pressed(&Controls::Menu) {
        match state.get() {
            PausedState::Playing => next_state.set(PausedState::Paused),
            PausedState::Paused => next_state.set(PausedState::Playing),
        }
    }
}

fn paused_menu(mut commands: Commands, contexts: EguiContexts)  {
    let builder = menu::MenuBuilder::new("Paused");
    builder.add_component(
        menu::MenuButton::new("Resume", menu::ChangeStateMenuAction::new(PausedState::Playing))
    ).add_component(
        menu::MenuButton::new("Main Menu", menu::ChangeStateMenuAction::new(GameState::Main)),
    ).build(contexts, &mut commands);
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(
                        Window {
                            present_mode: PresentMode::AutoNoVsync,
                            ..default()
                        }
                    ),
                    ..default()
                }
            )
        )
        .add_plugins((
            EguiPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            InputManagerPlugin::<Controls>::default(),
            PongPlugin,
        ))
        .add_plugins(RapierDebugRenderPlugin::default())
        .configure_sets(PostUpdate, (
            PhysicsSet::StepSimulation,
        ).run_if(in_state(GameState::Playing)).run_if(in_state(PausedState::Playing)))
        .configure_sets(Update, (
            MainSet.run_if(in_state(GameState::Main)),
            SettingsSet.run_if(in_state(GameState::Settings)),
            ControlsSet.run_if(in_state(GameState::Controls)),
            PlayingSet.run_if(in_state(GameState::Playing)).run_if(in_state(PausedState::Playing)),
            PausedSet.run_if(in_state(GameState::Playing)).run_if(in_state(PausedState::Paused)),
        ))
        .init_state::<GameState>()
        .init_state::<PausedState>()
        .init_resource::<InputMap<Controls>>()
        .init_resource::<ActionState<Controls>>()
        .insert_resource(Controls::default_input_map())
        .insert_resource(Difficulty::default())
        .add_systems(Startup, create_camera)
        .add_systems(Update, (
            main_menu.in_set(MainSet),
            settings_menu.in_set(SettingsSet),
            toggle_pause_game.in_set(PlayingSet),
            toggle_pause_game.in_set(PausedSet),
            paused_menu.in_set(PausedSet),
        ))
        .run();
}