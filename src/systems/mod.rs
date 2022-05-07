mod ascii;
mod camera_controller;
mod enviroment;
mod fadeout;
mod overlay;

pub use ascii::*;
use bevy::prelude::*;

pub use camera_controller::*;
pub use enviroment::*;
pub use fadeout::*;
pub use overlay::*;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
