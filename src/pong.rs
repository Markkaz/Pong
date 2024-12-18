use crate::{Controls, Difficulty, GameState, PausedState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use leafwing_input_manager::prelude::ActionState;

mod constants {
    pub const WALL_THICKNESS: f32 = 10.0;
    pub const TOP_BUFFER: f32 = 100.0;

    pub mod paddle {
        pub const WIDTH: f32 = 10.0;
        pub const HEIGHT: f32 = 100.0;
        pub const BUFFER: f32 = 40.0;
        pub const SPEED: f32 = 6.;
    }

    pub mod ball {
        pub const RADIUS: f32 = 10.0;
        pub const INITIAL_VELOCITY: (f32, f32) = (200.0, 100.0);
        pub const SPEED_INCREASE: f32 = 1.1;
    }
}

use constants::*;

#[derive(Component)]
struct Pong;

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

fn setup_game(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<PausedState>>,
) {

    score.player = 0;
    score.computer = 0;
    next_state.set(PausedState::Playing);

    let window = windows.single();
    let (window_width, window_height) = (window.resolution.width(), window.resolution.height());

    commands.spawn((
        Pong,
        Transform::from_xyz(0., 0., 0.),
        Visibility::default(),
    )).with_children(|builder| {
        create_board(
            builder,
            window_width,
            window_height,
            &mut meshes,
            &mut materials,
        );

        create_players(
            builder,
            window_width,
            &mut meshes,
            &mut materials,
        );

        spawn_ball(
            builder,
            &mut meshes,
            &mut materials,
        );

        create_score(builder, window_height);
    });

}

fn create_wall(
    commands: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    width: f32,
    height: f32,
    transform: Transform
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(width, height))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        transform,
        Collider::cuboid(width / 2.0, height / 2.0),
        RigidBody::Fixed,
    ));
}

fn create_board(
    builder: &mut ChildBuilder,
    screen_width: f32,
    screen_height: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Create horizontal walls
    for y_pos in [
        screen_height / 2.0 - WALL_THICKNESS - TOP_BUFFER,
        screen_height / -2.0 + WALL_THICKNESS,
    ] {
        create_wall(
            builder,
            meshes,
            materials,
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
        builder.spawn((
            Transform::from_xyz(
                x_pos,
                TOP_BUFFER / -2.0,
                0.0,
            ),
            Collider::cuboid(WALL_THICKNESS, sensor_height / 2.0),
            Sensor,
            score_field,
        ));
    }
}

fn create_paddle(
    builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    transform: Transform,
    paddle: Paddle,
) {
    builder.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            paddle::WIDTH,
            paddle::HEIGHT,
        ))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        transform,
        Collider::cuboid(paddle::WIDTH / 2.0, paddle::HEIGHT / 2.0),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController::default(),
        paddle,
    ));
}

fn create_players(
    builder: &mut ChildBuilder,
    screen_width: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    for (x_offset, paddle_type) in [
        (screen_width / -2.0 + paddle::BUFFER, Paddle::Player),
        (screen_width / 2.0 - paddle::BUFFER, Paddle::Computer),
    ] {
        create_paddle(
            builder,
            meshes,
            materials,
            Transform::from_xyz(x_offset, TOP_BUFFER / -2.0, 0.0),
            paddle_type,
        );
    }
}

fn create_score(builder: &mut ChildBuilder, window_height: f32) {
    builder.spawn((
        Text2d::new("0 - 0"),
        TextColor(Color::WHITE),
        TextFont { font_size: 100., ..default() },
        Transform::from_translation((window_height / 2.0 - 50.) * Vec3::Y),
    ));
}

fn spawn_ball(
    builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    builder.spawn((
        Mesh2d(meshes.add(Circle::new(ball::RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Ball,
        RigidBody::Dynamic,
        Ccd::enabled(),
        Velocity {
            linvel: Vec2::new(ball::INITIAL_VELOCITY.0, ball::INITIAL_VELOCITY.1),
            angvel: 0.,
        },
        GravityScale(0.),
        Sleeping::disabled(),
        Collider::ball(ball::RADIUS),
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        },
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn move_players(
    keys: Res<ActionState<Controls>>,
    difficulty: Res<Difficulty>,
    mut players: Query<(&mut KinematicCharacterController, &Paddle, &Transform)>,
    balls: Query<&Transform, With<Ball>>,
) {
    let ball = balls.single();

    for (player, paddle, paddle_position) in players.iter_mut() {
        match paddle {
            Paddle::Player => move_player(player, &keys),
            Paddle::Computer => move_computer(player, paddle_position, ball, *difficulty),
        }
    }
}

fn move_player(
    mut player: Mut<KinematicCharacterController>,
    keys: &Res<ActionState<Controls>>,
) {
    let mut direction = Vec2::ZERO;

    if keys.pressed(&Controls::Up) {
        direction.y += 1.0;
    }
    if keys.pressed(&Controls::Down) {
        direction.y -= 1.0;
    }

    player.translation = Some(direction.normalize_or_zero() * paddle::SPEED);
}

fn move_computer(
    mut player: Mut<KinematicCharacterController>,
    paddle_position: &Transform,
    ball: &Transform,
    difficulty: Difficulty,
) {
    let direction = Vec2::new(
        0.0,
        ball.translation.y - paddle_position.translation.y,
    );

    player.translation = Some(
        direction.clamp_length_max(difficulty.speed()),
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

fn update_score_display(score: Res<Score>, mut score_text: Query<&mut Text2d>) {
    for mut text in &mut score_text {
        text.0 = format!("{} - {}", score.player, score.computer);
    }
}

fn reset_ball(
    mut score_point_event: EventReader<ScorePointEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ball_entity: Query<Entity, With<Ball>>,
    pong_entity: Query<Entity, With<Pong>>,
) {
    for _ in score_point_event.read() {
        commands.entity(ball_entity.single()).despawn();

        let pong = pong_entity.single();
        commands.entity(pong).with_children(|parent| {
            spawn_ball(parent, &mut meshes, &mut materials);
        });
    }
}

fn cleanup_game(mut commands: Commands, pong: Query<Entity, With<Pong>>) {
    for entity in pong.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct PongPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PongUpdateSet;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .configure_sets(Update, PongUpdateSet.run_if(in_state(GameState::Playing)))
            .configure_sets(FixedUpdate, PongUpdateSet.run_if(in_state(GameState::Playing)))
            .add_event::<ScorePointEvent>()
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), cleanup_game)
            .add_systems(Update, (
                move_players,
                speed_up_ball,
                detect_point,
                score_point,
                update_score_display,
                reset_ball,
            ).in_set(PongUpdateSet));
    }
}