use bevy::prelude::*;
use crate::{despawn_state, quit_game, GameState, RootEntity};

mod constants {
    pub mod menu {
        use bevy::color::Color;

        pub const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
        pub const HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
        pub const PRESSED: Color = Color::srgb(0.35, 0.35, 0.35);
    }
}

use constants::*;

struct MenuButton {
    text: String,
    action: GameState,
    style: Option<Node>,
    enabled: bool,
}

#[derive(Component)]
struct ButtonAction(GameState);

#[derive(Default)]
struct MenuBuilder {
    style: Node,
    background_color: Option<Color>,
    buttons: Vec<MenuButton>,
    title: Option<String>,
    spacing: f32,
}

impl MenuBuilder {
    fn new() -> Self {
        Self {
            style: Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
    }

    fn with_background(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    fn add_button(
        mut self,
        text: impl Into<String>,
        action: GameState,
        enabled: bool,
    ) -> Self {
        self.buttons.push(MenuButton {
            text: text.into(),
            action,
            style: None,
            enabled,
        });
        self
    }

    fn add_styled_button(
        mut self,
        text: impl Into<String>,
        action: GameState,
        style: Node,
        enabled: bool,
    ) -> Self {
        self.buttons.push(MenuButton {
            text: text.into(),
            action,
            style: Some(style),
            enabled,
        });
        self
    }

    fn build(self, commands: &mut Commands) -> Entity {

        let root = commands.spawn((
            self.style,
            BackgroundColor(self.background_color.map(|c| c.into()).unwrap_or_default()),
        )).with_children(|parent| {
                if let Some(title) = self.title {
                    parent.spawn((
                        Text::new(title),
                        TextFont {
                            font_size: 48.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn(Node {
                        height: Val::Px(self.spacing),
                        ..default()
                    });
                }

                for button in self.buttons {
                    let button_style = button.style.unwrap_or(Node {
                        width: Val::Px(200.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    });

                    parent.spawn((
                        Button,
                        button_style,
                        if button.enabled {
                            BackgroundColor(Color::srgb(0.25, 0.25, 0.25))
                        } else {
                            BackgroundColor(Color::srgb(0.5, 0.5, 0.5))
                        }
                    )).with_children(|parent| {
                            parent.spawn((
                                Text(button.text),
                                TextFont {
                                    font_size: 24.,
                                    ..default()
                                },
                                if button.enabled {
                                    TextColor(Color::WHITE)
                                } else {
                                    TextColor(Color::srgb(0.5, 0.5, 0.5).into())
                                }
                            ));
                        })
                        .insert(ButtonAction(button.action));
                }
            })
            .id();
        root
    }
}

fn spawn_main_menu(mut commands: Commands) {
    let entity = MenuBuilder::new()
        .with_title("Pong!")
        .with_spacing(20.)
        .with_background(Color::srgb(0., 0., 0.,))
        .add_button("Play", GameState::Playing, true)
        .add_button("Settings", GameState::Settings, true)
        .add_button("Exit", GameState::Exit, true)
        .build(&mut commands);
    commands.insert_resource(RootEntity(entity));
}

fn spawn_settings_menu(mut commands: Commands) {
    let entity = MenuBuilder::new()
        .with_title("Settings")
        .with_spacing(20.)
        .with_background(Color::srgb(0., 0., 0.))
        .add_button("Back", GameState::Menu, true)
        .build(&mut commands);
    commands.insert_resource(RootEntity(entity));
}

fn update_menu(
    mut interaction_query: Query<
        (&ButtonAction, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (action, interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = menu::PRESSED.into();
                game_state.set(action.0.clone());
            },
            Interaction::Hovered => *color = menu::HOVERED.into(),
            Interaction::None => *color = menu::NORMAL.into(),
        }
    }
}

pub struct MenuPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MenuUpdateSet;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MenuUpdateSet.run_if(
            in_state(GameState::Menu)
                .or(in_state(GameState::Settings))
        ));
        app.add_systems(OnEnter(GameState::Menu), spawn_main_menu);
        app.add_systems(OnExit(GameState::Menu), despawn_state);
        app.add_systems(OnEnter(GameState::Exit), quit_game);
        app.add_systems(OnEnter(GameState::Settings), spawn_settings_menu);
        app.add_systems(OnExit(GameState::Settings), despawn_state);
        app.add_systems(Update, update_menu.in_set(MenuUpdateSet));
    }
}