use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct StartMenu;

#[derive(Component)]
pub struct Overworld;

pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_pause(GameState::StartMenu).with_system(cleanup_system::<StartMenu>),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Overworld).with_system(cleanup_system::<Overworld>),
        );
    }
}

fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
