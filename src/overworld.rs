use crate::ascii::*;
use crate::fadeout::create_fadeout;
use crate::{cleanup, enviroment::*};
use crate::{CameraController, GameState};
use bevy::utils::Duration;
use bevy::{ecs::system::Command, prelude::*};
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::*;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Overworld)
                .with_system(setup_overworld));
    }
}

fn setup_overworld(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    info!("Setting up overworld");

    //clear_color.0 = Color::rgb(0.1, 0.3, 0.3);

    // cameras
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera3d"))
        .insert(cleanup::Overworld);

    // light
    commands.spawn().insert(Sun).insert(cleanup::Overworld);

    // Ground
    commands.spawn().insert(Ground).insert(cleanup::Overworld);

    // Create a single animation (tween) to move an entity.
    let tween = Tween::new(
        // Use a quadratic easing on both endpoints.
        EaseFunction::QuadraticInOut,
        // Loop animation back and forth.
        TweeningType::PingPong,
        // Animation time (one way only; for ping-pong it takes 2 seconds
        // to come back to start).
        Duration::from_secs(1),
        // The lens gives the Animator access to the Transform component,
        // to animate it. It also contains the start and end values associated
        // with the animation ratios 0. and 1.
        TransformPositionLens {
            start: Vec3::new(0., 0., 0.),
            end: Vec3::new(1., 2., -4.),
        },
    );

    // sphere
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.3, 0.3),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Animator::new(tween))
        .insert(Name::new("Sphere"))
        .insert(cleanup::Overworld);
}
