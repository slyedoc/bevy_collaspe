mod colliders;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
pub use colliders::*;

#[derive(Component, Inspectable, Debug)]
pub enum RigidBody {
    Static,
    Dynamic,
}

// TODO: will most likey come back for component based approach
// #[derive(Component, Debug)]
// struct RigidBodyStatic;

// #[derive(Component, Debug)]
// struct RigidBodyDynamic;

#[derive(Component, Inspectable, Debug, Default)]
pub struct LinearVelocity(pub Vec3);

#[derive(Component, Inspectable, Debug, Default)]
pub struct AngularVelocity(pub Vec3);

#[derive(Component, Inspectable, Debug, Default)]
pub struct Elasticity(pub f32); // assumed [0,1]

#[derive(Component, Inspectable, Debug, Default)]
pub struct Friction(pub f32); // assumed [0,1]

#[derive(Component, Inspectable, Debug, Default)]
pub struct Mass(pub f32);

#[derive(Component, Inspectable, Debug, Default)]
pub struct InvMass(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_inspectable::<RigidBody>()
            .register_inspectable::<LinearVelocity>()
            .register_inspectable::<AngularVelocity>()
            .register_inspectable::<Elasticity>()
            .register_inspectable::<Friction>()
            .register_inspectable::<Mass>()
            .register_inspectable::<InvMass>()
            .register_inspectable::<Collider>()
            .add_system(PhysicsPlugin::spawn_components);
    }
}

impl PhysicsPlugin {
    pub fn spawn_components(
        mut commands: Commands,
        mut query: Query<
            (
                Entity,
                &Collider,
                Option<&RigidBody>,
                Option<&Mass>,
                Option<&InvMass>,
                Option<&LinearVelocity>,
                Option<&AngularVelocity>,
                Option<&Elasticity>,
                Option<&Friction>,
            ),
            (Added<Collider>),
        >,
    ) {
        for (e, collider, rigid_body, mass, inv_mass, linear_vel, angular_vel, elasticity, friction) in query.iter() {
            // add rigid body if not already added
            let mut added_rb = false;
            if rigid_body.is_none() {
                commands.entity(e).insert(RigidBody::Static);
                added_rb = true;
            }

            // add inv_mass
            if inv_mass.is_none() {
                if let Some(mass) = mass {
                    commands.entity(e).insert(InvMass(1.0 / mass.0));
                } else {
                    commands.entity(e).insert(InvMass(1.0));
                }
            }

            // add linear velocity
            if linear_vel.is_none() {
                commands.entity(e).insert(LinearVelocity::default());
            }

            // add angular velocity
            if angular_vel.is_none() {
                commands.entity(e).insert(AngularVelocity::default());
            }

            // add elasticity
            if elasticity.is_none() {
                commands.entity(e).insert(Elasticity::default());
            }

            // add friction
            if friction.is_none() {
                commands.entity(e).insert(Friction::default());
            }
        }
    }
}
