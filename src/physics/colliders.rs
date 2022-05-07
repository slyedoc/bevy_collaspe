
use bevy::{prelude::*, math::vec3};
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Debug)]
pub enum Collider {
    Sphere { radius: f32 },
    Cuboid { size: Vec3 },
}

impl Collider {
    pub fn sphere(radius: f32) -> Self {
        Collider::Sphere { radius }
    }

    pub fn cuboid(x: f32, y: f32, z: f32) -> Self {
        Collider::Cuboid { size: vec3(x, y, z) }
    }
}
