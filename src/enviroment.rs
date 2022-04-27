use bevy::prelude::*;

pub fn setup_light(mut commands: Commands) {
    const HALF_SIZE: f32 = 50.0;
    commands
        .spawn_bundle(DirectionalLightBundle {
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
        .insert(Name::new("Light"));
}


pub fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // ground
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..Default::default()
        }),
        ..Default::default()
    });
}
