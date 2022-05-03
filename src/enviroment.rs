use bevy::{
    pbr::SpecializedMaterial,
    prelude::*,
    render::{primitives::Frustum, view::VisibleEntities},
};

pub struct EnviromentPlugin;
impl Plugin for EnviromentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_sun).add_system(create_ground);
    }
}

#[derive(Component)]
pub struct Sun;

fn create_sun(mut commands: Commands, mut query: Query<Entity, Added<Sun>>) {
    const HALF_SIZE: f32 = 50.0;

    for e in query.iter() {
        commands.entity(e).insert_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 10000.0,
                // Configure the projection to better fit the scene
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 100.0 * HALF_SIZE,
                    ..Default::default()
                },
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(10.0, 2.0, 10.0),
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(Name::new("Sun"));
    }
}

#[derive(Component)]
pub struct Ground;

fn create_ground(
    mut commands: Commands,
    mut query: Query<Entity, Added<Ground>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for e in query.iter() {
        info!("Creating ground");
        commands
            .entity(e)
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(Name::new("Ground"));
    }
}
