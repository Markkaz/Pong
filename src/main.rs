use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

mod constants {
    pub const WALL_THICKNESS: f32 = 10.0;
    pub const TOP_BUFFER: f32 = 100.0;

    pub mod paddle {
        pub const WIDTH: f32 = 10.0;
        pub const HEIGHT: f32 = 100.0;
        pub const BUFFER: f32 = 40.0;
        pub const SPEED: f32 = 400.0;
    }

    pub mod ball {
        pub const RADIUS: f32 = 10.0;
        pub const INITIAL_VELOCITY: (f32, f32) = (200.0, 100.0);
        pub const SPEED_INCREASE: f32 = 1.1;
    }

    pub mod menu {
        use bevy::color::Color;

        pub const NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
        pub const HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
        pub const PRESSED: Color = Color::srgb(0.35, 0.35, 0.35);
    }
}

use constants::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Settings,
    Playing,
    Paused,
    Exit,
}

#[derive(Resource)]
struct RootEntity(Entity);

struct MenuButton {
    text: String,
    action: GameState,
    style: Option<Style>,
    enabled: bool,
}

#[derive(Component)]
struct ButtonAction(GameState);

#[derive(Default)]
struct MenuBuilder {
    style: Style,
    background_color: Option<Color>,
    buttons: Vec<MenuButton>,
    title: Option<String>,
    spacing: f32,
}

impl MenuBuilder {
    fn new() -> Self {
        Self {
            style: Style {
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
        style: Style,
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
        let root = commands
            .spawn(NodeBundle {
                style: self.style,
                background_color: self.background_color.map(|c| c.into()).unwrap_or_default(),
                ..default()
            })
            .with_children(|parent| {
                if let Some(title) = self.title {
                    parent.spawn(TextBundle::from_section(
                        title,
                        TextStyle {
                            font_size: 48.,
                            color: Color::WHITE,
                            ..default()
                        }
                    ));

                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(self.spacing),
                            ..default()
                        },
                        ..default()
                    });
                }

                for button in self.buttons {
                    let button_style = button.style.unwrap_or(Style {
                        width: Val::Px(200.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    });
                    parent.spawn(ButtonBundle {
                        style: button_style,
                        background_color: if button.enabled {
                            Color::srgb(0.25, 0.25, 0.25).into()
                        } else {
                            Color::srgb(0.5, 0.5, 0.5).into()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            button.text,
                            TextStyle {
                                font_size: 24.,
                                color: if button.enabled {
                                    Color::WHITE
                                } else {
                                    Color::srgb(0.5, 0.5, 0.5)
                                },
                                ..default()
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

fn despawn_state(mut commands: Commands, root: Res<RootEntity>) {
    commands.entity(root.0).despawn_recursive();
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

fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

#[derive(Component)]
enum ScoreField {
    Left,
    Right,
}

#[derive(Component)]
enum Paddle {
    Player,
    Computer,
}

#[derive(Component)]
struct Ball;

#[derive(Resource)]
struct Score {
    player: u32,
    computer: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            player: 0,
            computer: 0,
        }
    }
}
#[derive(Event)]
struct ScorePointEvent(Entity);

pub struct MenuPlugin;
pub struct PongPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PongUpdateSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MenuUpdateSet;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MenuUpdateSet.run_if(
            in_state(GameState::Menu)
                .or_else(in_state(GameState::Settings))
        ));
        app.add_systems(OnEnter(GameState::Menu), spawn_main_menu);
        app.add_systems(OnExit(GameState::Menu), despawn_state);
        app.add_systems(OnEnter(GameState::Exit), quit_game);
        app.add_systems(OnEnter(GameState::Settings), spawn_settings_menu);
        app.add_systems(OnExit(GameState::Settings), despawn_state);
        app.add_systems(Update, update_menu.in_set(MenuUpdateSet));
    }
}

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .configure_sets(Update, PongUpdateSet.run_if(in_state(GameState::Playing)))
            .add_event::<ScorePointEvent>()
            .add_systems(OnEnter(GameState::Playing), (
                create_board,
                create_players,
                create_ball,
                create_score,
            ))
            .add_systems(Update, (
                move_players,
                speed_up_ball,
                detect_point,
            ).in_set(PongUpdateSet))
            .add_systems(Update, (
                score_point,
                update_score_display,
                reset_ball,
            ).in_set(PongUpdateSet));
    }
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

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    width: f32,
    height: f32,
    transform: Transform
) {
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(width, height)));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(Color::WHITE),
            transform,
            ..default()
        },
        Collider::cuboid(width / 2.0, height / 2.0),
        RigidBody::Fixed,
    ));
}

fn create_board(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();
    let (screen_width, screen_height) = (window.resolution.width(), window.resolution.height());

    // Create horizontal walls
    for y_pos in [
        screen_height / 2.0 - WALL_THICKNESS - TOP_BUFFER,
        screen_height / -2.0 + WALL_THICKNESS,
    ] {
        create_wall(
            &mut commands,
            &mut meshes,
            &mut materials,
            screen_width,
            WALL_THICKNESS,
            Transform::from_xyz(0.0, y_pos, 0.0),
        );
    }

    // Create scoring sensors
    let sensor_height = screen_height - TOP_BUFFER - WALL_THICKNESS;
    for (x_pos, score_field) in [
        (screen_width / -2.0 + WALL_THICKNESS, ScoreField::Left),
        (screen_width / 2.0 - WALL_THICKNESS, ScoreField::Right),
    ] {
        commands.spawn((
            TransformBundle::from(Transform::from_xyz(
                x_pos,
                TOP_BUFFER / -2.0,
                0.0,
            )),
            Collider::cuboid(WALL_THICKNESS, sensor_height / 2.0),
            Sensor,
            score_field,
        ));
    }
}

fn create_players(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let screen_width = windows.single().resolution.width();

    for (x_offset, paddle_type) in [
        (screen_width / -2.0 + paddle::BUFFER, Paddle::Player),
        (screen_width / 2.0 - paddle::BUFFER, Paddle::Computer),
    ] {
        create_paddle(
            &mut commands,
            &mut meshes,
            &mut materials,
            Transform::from_xyz(x_offset, TOP_BUFFER / -2.0, 0.0),
            paddle_type,
        );
    }
}

fn create_paddle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
    paddle: Paddle,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                paddle::WIDTH,
                paddle::HEIGHT,
            ))),
            material: materials.add(Color::WHITE),
            transform,
            ..default()
        },
        Collider::cuboid(paddle::WIDTH / 2.0, paddle::HEIGHT / 2.0),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        paddle,
    ));
}

fn create_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_ball(&mut commands, &mut meshes, &mut materials);
}

fn create_score(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0 - 0",
            TextStyle { font_size: 100.0, ..default() },
        )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                width: Val::Percent(100.0),
                ..default()
            }),
    ));
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(ball::RADIUS))),
        material: materials.add(Color::WHITE),
        ..default()
    })
    .insert((
        Ball,
        RigidBody::Dynamic,
        Ccd::enabled(),
        Velocity {
            linvel: Vec2::new(ball::INITIAL_VELOCITY.0, ball::INITIAL_VELOCITY.1),
            angvel: 0.0,
        },
        GravityScale(0.0),
        Sleeping::disabled(),
        Collider::ball(ball::RADIUS),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn move_players(
    keys: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut KinematicCharacterController, &Paddle, &Transform)>,
    balls: Query<&Transform, With<Ball>>,
    time: Res<Time>,
) {
    let ball = balls.single();

    for (player, paddle, paddle_position) in players.iter_mut() {
        match paddle {
            Paddle::Player => move_player(player, &keys, &time),
            Paddle::Computer => move_computer(player, paddle_position, ball, &time),
        }
    }
}

fn move_player(
    mut player: Mut<KinematicCharacterController>,
    keys: &Res<ButtonInput<KeyCode>>,
    time: &Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    player.translation = Some(direction.normalize_or_zero() * paddle::SPEED * time.delta_seconds());
}

fn move_computer(
    mut player: Mut<KinematicCharacterController>,
    paddle_position: &Transform,
    ball: &Transform,
    time: &Res<Time>,
) {
    let direction = Vec2::new(
        0.0,
        ball.translation.y - paddle_position.translation.y,
    );

    player.translation = Some(
        direction.clamp_length_max(paddle::SPEED) * time.delta_seconds(),
    );
}

fn speed_up_ball(
    mut collision_events: EventReader<CollisionEvent>,
    mut velocities: Query<&mut Velocity>,
) {
    for entities in collision_events.read().filter_map(|event| {
        match event {
            CollisionEvent::Started(entity1, entity2, _) => Some((*entity1, *entity2)),
            _ => None,
        }
    }) {
        if let Ok(mut velocity) = velocities.get_mut(entities.0) {
            velocity.linvel.y *= ball::SPEED_INCREASE;
        } else if let Ok(mut velocity) = velocities.get_mut(entities.1) {
            velocity.linvel.y *= ball::SPEED_INCREASE;
        }
    }
}

fn detect_point(
    mut collision_events: EventReader<CollisionEvent>,
    mut ev_score_point: EventWriter<ScorePointEvent>
) {
    collision_events.read()
        .filter(|event| matches!(
            event,
            CollisionEvent::Started(_, _, flags) if *flags & CollisionEventFlags::SENSOR != CollisionEventFlags::empty()
        ))
        .for_each(|event| {
            match event {
                CollisionEvent::Started(entity, _, _) => {
                    ev_score_point.send(ScorePointEvent(*entity));
                },
                _ => (),
            }
        })
}

fn score_point(
    mut score: ResMut<Score>,
    walls: Query<&ScoreField>,
    mut point_events: EventReader<ScorePointEvent>
) {
    for event in point_events.read() {
        if let Ok(wall) = walls.get(event.0) {
            match wall {
                ScoreField::Right => score.player += 1,
                ScoreField::Left => score.computer += 1,
            }
        }
    }
}

fn update_score_display(score: Res<Score>, mut score_text: Query<&mut Text>) {
    for mut text in &mut score_text {
        text.sections[0].value = format!("{} - {}", score.player, score.computer);
    }
}

fn reset_ball(
    mut score_point_event: EventReader<ScorePointEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<Ball>>,
) {
    for _ in score_point_event.read() {
        commands.entity(query.single()).despawn();
        spawn_ball(&mut commands, &mut meshes, &mut materials);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .init_state::<GameState>()
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            MenuPlugin,
            PongPlugin,
        ))
        .add_systems(Startup, create_camera)
        .run();
}