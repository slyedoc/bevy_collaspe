use crate::GameState;
use crate::math::range_lerp;
use crate::physics::{*};
use crate::systems::cleanup_system;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::{ecs::component::Component, prelude::*};
use bevy_inspector_egui::*;
use rand::Rng;

pub struct BreakoutPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BreakoutState {
    Playing,
    Resetting,
}
#[derive(Inspectable, Debug)]
struct BreakoutConfig {
    pub rapier_scale: f32,

    // breakout settings
    pub player_size_half: Vec2,
    pub player_speed: f32,

    pub board_size_half: Vec2,
    pub board_line_size_half: f32,
    pub brick_grid: (u8, u8),
    pub ball_size_half: f32,
    pub ball_init_x_range: (f32, f32),
    pub ball_init_y: f32,
    pub ball_speed: f32,
    pub ball_y_basis: f32,
    pub ball_basis_engage: f32,
}

impl Default for BreakoutConfig {
    fn default() -> Self {
        Self {
            rapier_scale: 50.0,
            player_size_half: Vec2::new(1.0, 0.1),
            player_speed: 0.2,
            board_size_half: Vec2::new(4.0, 6.0),
            board_line_size_half: 0.1,
            brick_grid: (3, 5),
            ball_size_half: 0.1,
            ball_init_x_range: (-2.0, 2.0),
            ball_init_y: 5.0,
            ball_speed: 3.0,
            ball_y_basis: 0.01,
            ball_basis_engage: 0.8,
        }
    }
}

#[derive(Component)]
struct Brick;

#[derive(Component)]
pub struct Player {
    index: usize,
}
struct Score(usize);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Bottom;

#[derive(Component)]
struct Breakout;

pub struct BreakoutAssets {
    pub board: Handle<StandardMaterial>,
    pub brick: Handle<StandardMaterial>,
    pub ball: Handle<StandardMaterial>,
}

impl FromWorld for BreakoutAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let board = materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 0.0),
            unlit: true,
            ..Default::default()
        });

        let brick = materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.3, 0.7),
            unlit: true,
            ..Default::default()
        });

        let ball = materials.add(StandardMaterial {
            base_color: Color::RED,
            unlit: true,
            ..Default::default()
        });

        Self { board, brick, ball }
    }
}

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_plugin(InspectorPlugin::<BreakoutConfig>::new())
        .init_resource::<BreakoutConfig>()
        .init_resource::<BreakoutAssets>()
        .insert_resource(Score(0))
        .add_state(BreakoutState::Playing)
            .add_system_set(
                SystemSet::on_enter(GameState::Breakout)
                    .with_system(setup)
                    .with_system(spawn_player)
                    .with_system(spawn_ball),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Breakout)
                    //.with_system(update_ball)
                    .with_system(brick_collisions)
                    //.with_system(bottom_collisions)
                    .with_system(player_movement)
                    
                    // Old
                    //.with_system(ball_collision)
                    //.with_system(ball_bounds_check),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Breakout).with_system(cleanup_system::<Breakout>),
            );;

    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut rapier_config: ResMut<RapierConfiguration>,
    mut score: ResMut<Score>,
    mut config: ResMut<BreakoutConfig>,
    mut ambient_light: ResMut<AmbientLight>,
    breakout_assets: Res<BreakoutAssets>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::rgb(0.2, 0.2, 0.2);
    //rapier_config.gravity = Vec2::ZERO.into();
    score.0 = 0;

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        //.insert(CameraController::default())
        .insert(Name::new("Camera3d"))
        .insert(Breakout);

    // ambient light
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.02;

    //let board = commands
    // .spawn_bundle(TransformBundle::default())
    // .insert(RigidBody::Fixed)
    // .insert(Name::new("Board"))
    // .id();

    // Left
    create_board_side(
        &mut commands,
        &mut meshes,
        Vec2::new(-config.board_size_half.x, 0.0),
        Vec2::new(config.board_line_size_half, config.board_size_half.y),
        &breakout_assets,
    );
    // Right
    create_board_side(
        &mut commands,
        &mut meshes,
        Vec2::new(config.board_size_half.x, 0.0),
        Vec2::new(config.board_line_size_half, config.board_size_half.y),
        &breakout_assets,
    );

    // Top
    create_board_side(
        &mut commands,
        &mut meshes,
        Vec2::new(0.0, config.board_size_half.y),
        Vec2::new(
            config.board_size_half.x + config.board_line_size_half,
            config.board_line_size_half,
        ),
        &breakout_assets,
    );

    // Bottom
    let bottom = create_board_side(
        &mut commands,
        &mut meshes,
        Vec2::new(0.0, -config.board_size_half.y),
        Vec2::new(
            config.board_size_half.x + config.board_line_size_half,
            config.board_line_size_half,
        ),
        &breakout_assets,
    );
    commands.entity(bottom).insert(Bottom);

    let size_x: f32 = config.board_size_half.x / (config.brick_grid.0 + 2) as f32;
    let size_y: f32 = config.board_size_half.y * 0.5 / (config.brick_grid.1 + 2) as f32;

    // Create Bricks
    for x in 0..config.brick_grid.0 {
        for y in 0..config.brick_grid.1 {
            let pos_x = range_lerp(
                (x + 1) as f32,
                0.0,
                (config.brick_grid.0 + 1) as f32,
                -config.board_size_half.x,
                config.board_size_half.x,
            );
            let pos_y = range_lerp(
                (y + 1) as f32,
                0.0,
                (config.brick_grid.1 + 1) as f32,
                0.0,
                config.board_size_half.y,
            );

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: vec2(size_x, size_y),
                        ..default()
                    })),
                    material: breakout_assets.brick.clone(),
                    transform: Transform::from_xyz(pos_x, pos_y, 0.0),
                    ..default()
                })
                //.insert(RigidBody::Fixed)
                .insert(Collider::cuboid(size_x * 0.5, size_y * 0.5, 1.0))
                //.insert(ColliderMassProperties::Density(2.0))
                //.insert(Friction::coefficient(0.0))
                //.insert(Restitution::coefficient(1.0))
                .insert(Brick)
                .insert(Name::new(format!("Brick {}x{}", x, y)))
                .insert(Breakout);
        }
    }
}

fn create_board_side(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    pos: Vec2,
    size_half: Vec2,
    assets: &BreakoutAssets,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: size_half * 2.0,
                ..default()
            })),
            material: assets.board.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        })
        //.insert(RigidBody::Fixed)
        //.insert(ColliderMassProperties::Density(2.0))
        .insert(Collider::cuboid(size_half.x, size_half.y, 1.0))
        //.insert(Friction::coefficient(0.0))
        //.insert(Restitution::coefficient(1.0))
        .insert(Breakout)
        .insert(Name::new("Board Side"))
        .id()
}

fn spawn_player(
    mut commands: Commands,
    config: Res<BreakoutConfig>,
    breakout_assets: Res<BreakoutAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: config.player_size_half * 2.0,
                ..default()
            })),
            material: breakout_assets.board.clone(),
            transform: Transform::from_xyz(
                0.0,
                -config.board_size_half.y + (config.board_size_half.y * 0.1),
                0.0,
            ),
            ..default()
        })
        //.insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            config.player_size_half.x,
            config.player_size_half.y,
         1.0))
        //.insert(ColliderMassProperties::Density(2.0))
        //.insert(Friction::coefficient(0.0))
        //.insert(Restitution::coefficient(1.0))
        .insert(Player { index: 0 })
        .insert(Name::new("Player"))
        .insert(Breakout);
}

fn spawn_ball(
    mut commands: Commands,
    config: Res<BreakoutConfig>,
    breakout_assets: Res<BreakoutAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut rnd = rand::thread_rng();
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: config.ball_size_half,
                ..default()
            })),
            material: breakout_assets.board.clone(),
            transform: Transform::from_xyz(
                0.0,
                -config.board_size_half.y + (config.board_size_half.y * 0.2),
                0.0,
            ),
            ..default()
        })
        .insert(RigidBody)
        .insert(LinearVelocity(
            vec3(
                rnd.gen_range(config.ball_init_x_range.0..config.ball_init_x_range.1), 
                config.ball_init_y,
            0.0)))
        .insert(Friction(0.0))
        .insert(Elasticity(1.0))
        //.insert(ActiveEvents::COLLISION_EVENTS)
        //.insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::sphere(config.ball_size_half))
        //.insert(ColliderMassProperties::Density(2.0))
        .insert(Ball)
        .insert(Name::new("Ball"))
        .insert(Breakout);
}

// Keep the ball speed somewhat constant and  avoid getting stuck by back and forth
fn update_ball(mut balls: Query<&mut LinearVelocity, With<Ball>>, config: Res<BreakoutConfig>) {
    for mut rb_vel in balls.iter_mut() {
        // Normalize ball speed, currently picked at random
        // let mag = rb_vel.linvel.length();
        // let speed = config.ball_speed;
        // if mag != speed {
        //     rb_vel.linvel *= speed / mag;
        // }

        // // This will curve that ball up when its going more left to right that up and down
        // // so it can't get stuck, relies on the speed normalizing above
        // if rb_vel.linvel[0].abs() > config.ball_basis_engage * speed {
        //     rb_vel.linvel[1] += if rb_vel.linvel[1].is_sign_positive() {
        //         config.ball_y_basis
        //     } else {
        //         -config.ball_y_basis
        //     };
        // }
    }
}

// The ball can get away using the paddle to force it though a wall, this checks for that
// fn ball_bounds_check(
//     balls: Query<&RigidBodyPosition, With<Ball>>,
//     config: Res<BreakoutConfig>,
//     mut state: ResMut<State<BreakoutState>>,
// ) {
//     for rb_pos in balls.iter() {
//         if rb_pos.position.translation.x.abs() > config.board_size_half.x
//             || rb_pos.position.translation.y.abs() > config.board_size_half.y
//         {
//             state.set(BreakoutState::Resetting).unwrap()
//         }
//     }
// }

/* A system that displays the events. */
fn brick_collisions(
    mut commands: Commands,
   // mut collision_events: EventReader<CollisionEvent>,
    bricks: Query<Entity, With<Brick>>,
    mut score: ResMut<Score>,
    mut state: ResMut<State<BreakoutState>>,
) {
    // for e in collision_events.iter() {
    //     info!("Collision event: {:?}", e);
    //     if let CollisionEvent::Stopped(a, b, _) = e {
    //         // remove bricks
    //         if let Ok(bricks) = bricks.get(*a) {
    //             commands.entity(*a).despawn_recursive();
    //             score.0 += 1;
    //         }
    //         if let Ok(bricks) = bricks.get(*b) {
    //             commands.entity(*b).despawn_recursive();
    //             score.0 += 1;
    //         }
    //     }
    // }
}

fn bottom_collisions(
    mut commands: Commands,
    //mut collision_events: EventReader<CollisionEvent>,
    bottom: Query<Entity, With<Bottom>>,
    mut state: ResMut<State<BreakoutState>>,
) {
    // for e in collision_events.iter() {
    //     if let CollisionEvent::Stopped(a, b, _) = e {
    //         // remove ball and restart
    //         if let Ok(c) = bottom.get(*a) {
    //             commands.entity(*b).despawn_recursive();
    //             state.set(BreakoutState::Resetting).unwrap();
    //         }
    //         if let Ok(c) = bottom.get(*b) {
    //             commands.entity(*a).despawn_recursive();
    //             state.set(BreakoutState::Resetting).unwrap();
    //         }
    //     }
    // }
}

fn other_keyboard_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<BreakoutState>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        state.set(BreakoutState::Resetting).unwrap();
        // TODO: You get stuck in a loop without updating keyboard
        // https://github.com/bevyengine/bevy/issues/1700
        keyboard_input.reset(KeyCode::Escape);
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<&mut Transform, With<Player>>,
    config: Res<BreakoutConfig>,
) {
    let movement = config.player_speed;
    let limit = config.board_size_half.x - config.player_size_half.x - config.board_line_size_half;
    for mut trans in players.iter_mut() {
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x: f32 = if left {
            trans.translation.x - movement
        } else if right {
            trans.translation.x + movement
        } else {
            0.0
        };
        if x != 0.0 {
            trans.translation.x = x.clamp(-limit, limit);
        }
    }
}
