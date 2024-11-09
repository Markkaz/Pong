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
}

use constants::*;

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
struct Player;

#[derive(Component)]
struct Computer;

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

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_event::<ScorePointEvent>()
            .add_systems(Startup, (
                create_camera,
                create_board,
                create_players,
                create_ball,
                create_score,
            ))
            .add_systems(Update, (
                move_players,
                speed_up_ball,
                detect_point,
            ))
            .add_systems(Update, (
                score_point,
                update_score_display,
                reset_ball,
            ));
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

    for (mut player, paddle, paddle_position) in players.iter_mut() {
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
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            PongPlugin,
        ))
        .run();
}