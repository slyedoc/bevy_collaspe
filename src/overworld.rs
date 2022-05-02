use crate::fadeout::create_fadeout;
use crate::{enviroment::*, cleanup};
use crate::{CameraController, GameState};
use crate::ascii::*;
use bevy::{ecs::system::Command, prelude::*};

pub struct Overworld;

impl Plugin for Overworld {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Overworld)
                .with_system(setup_overworld)
                .with_system(setup_light)
                .with_system(setup_ground),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
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

    clear_color.0 = Color::rgb(0.0, 0.0, 0.0);
    
    // cameras
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 5.0)),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Overworld Camera"))
        .insert(cleanup::Overworld);

    // sphere
    commands.spawn_bundle(PbrBundle {
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
    });
}