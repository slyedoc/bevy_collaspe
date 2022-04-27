#![allow(warnings)]

mod camera_controller;
mod enviroment;
mod wave;

use camera_controller::*;
use enviroment::*;
use wave::*;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use rand::Rng;

/// An implementation of the classic game "Breakout"
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
enum Cell {
    Solid,
    Scorable,
    Paddle,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(camera_controller::CameraControllerPlugin)
        // .insert_resource(Scoreboard { score: 0 })
        // .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // .add_state(GameState::MainMenu)
        // .add_event::<GameOverEvent>()
        .add_startup_system(setup_cameras)
        .add_startup_system(setup_light)
        .add_startup_system(setup_ground)
        .add_startup_system(setup_wave)
        // .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(ui_system_setup))
        // .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(key_input_system))
        // .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(teardown::<OnUIScreen>))
        // .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
        // .add_system_set(
        //     SystemSet::on_update(GameState::InGame)
        //         .with_system(paddle_movement_system)
        //         .with_system(ball_collision_system)
        //         .with_system(ball_movement_system)
        //         .with_system(scoreboard_system)
        //         .with_system(on_game_over),
        // )
        // .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(teardown::<OnGameScreen>))
        // .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(ui_system_setup))
        // .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(key_input_system))
        // .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown::<OnUIScreen>))
        // .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)))
        // .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_cameras(mut commands: Commands) {
    // cameras
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 5.0)),
            ..Default::default()
        })
        .insert(CameraController::default());

    commands.spawn_bundle(UiCameraBundle::default());
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tiles {
    Sand,
    Grass,
    Water,
    Forest,
}

impl Collapsible<Tiles> for Tiles {
    fn allowed_neighbors(&self) -> Vec<Tiles> {
        match self {
            Tiles::Sand => vec![Tiles::Grass, Tiles::Water],
            Tiles::Grass => vec![Tiles::Sand, Tiles::Forest],
            Tiles::Water => vec![Tiles::Sand],
            Tiles::Forest => vec![Tiles::Grass],
        }
    }

    fn values() -> Vec<Tiles> {
        vec![Tiles::Sand, Tiles::Grass, Tiles::Water, Tiles::Forest]
    }
}

fn setup_wave(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut wave = Wave::<Tiles>::new(4, 4, 50.0);
    wave.seed(0, 0, Tiles::Water);

    wave.print();

    // sphere
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.5,
            ..Default::default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.3, 0.3),
            ..Default::default()
        }),
        ..Default::default()
    });
}
