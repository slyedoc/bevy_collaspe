use crate::{tiles::Tiles};
use bevy::prelude::*;
use rand::Rng;
use std::{fmt::Debug, fmt::Display, marker::PhantomData, process::Output};

use itertools::Itertools;

pub struct Wave {
    width: usize,
    height: usize,
    cell_size: f32,
}

impl FromWorld for Wave {
    fn from_world(world: &mut World) -> Self {
        Wave {
            width: 10,
            height: 10,
            cell_size: 1.0,
        }
    }
}

pub struct WaveCollapseEvent;
pub struct CellUpdateEvent(CellPosition);



#[derive(Component, Eq, PartialEq, Debug, Copy, Clone)]
pub struct CellPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Clone, PartialEq, Eq, Debug)]
pub struct CellPossable(Vec<Tiles>);

impl CellPossable {
    pub fn allowed_neighbors(&self) -> Vec<Tiles> {
        self.0
            .iter()
            .map(|t| t.allowed_neighbors())
            .flatten()
            .dedup()
            .collect::<Vec<_>>()
    }
}

#[derive(Component, Clone, PartialEq, Eq, Debug)]
pub struct CellFixed(Tiles);

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wave>()
            .add_event::<WaveCollapseEvent>()
            .add_event::<CellUpdateEvent>()
            .add_startup_system(spawn_tiles)
            .add_system(collapse_event)
            .add_system(cell_fixed_listener)
            //.add_system(cell_listener)
            .add_system(keyboard_input);
    }
}

fn keyboard_input(
    input: Res<Input<KeyCode>>,
    wave: ResMut<Wave>,
    query: Query<(&mut CellPossable, &mut Transform)>,
    mut collapse_event: EventWriter<WaveCollapseEvent>,
) {
    if input.pressed(KeyCode::Space) {
        collapse_event.send(WaveCollapseEvent);
    }
}


// Listens for the WaveCollapseEvent and collapses one cell in the wave
pub fn collapse_event(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CellPossable, &mut CellPosition, &mut Handle<StandardMaterial>)>,
    mut collapse_events: EventReader<WaveCollapseEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in collapse_events.iter() {
        
        // Search for the cells with the least possable values and make a list
        let mut lowest_possable_count = usize::max_value();
        let mut lowest_possable_enties = Vec::new();

        for (e, possable, pos, material) in query.iter() {
            if possable.0.len() < lowest_possable_count {
                lowest_possable_count = possable.0.len();
                lowest_possable_enties = vec![e];
            } else if possable.0.len() == lowest_possable_count {
                lowest_possable_enties.push(e);
            }
        }

        // from the list, if any, pick one at random and collapse it
        if lowest_possable_enties.len() > 0 {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..lowest_possable_enties.len());
            let e = lowest_possable_enties[index];
            info!("Collapse {:?}", e);
            if let Ok((e, mut possable, pos, mut material)) =
                query.get_mut(lowest_possable_enties[index])
            {
                // Collapse the cell
                let fixed_value = possable.0[rng.gen_range(0..possable.0.len())];

                // Update the cell
                commands.entity(e).insert(materials.add(fixed_value.color().into()));
                
                info!("{:?} e: {:?}", pos, e);
                commands
                    .entity(e)
                    .remove::<CellPossable>()
                    .insert(CellFixed(fixed_value));
            }
        }
    }
}

pub fn cell_fixed_listener(
    mut commands: Commands,
    mut cell_fixed_added: Query<(&CellFixed, &CellPosition), (Added<CellFixed>)>,
    mut cell_update_events: EventWriter<CellUpdateEvent>,
    wave: Res<Wave>,
) {
    for (fixed, pos) in cell_fixed_added.iter() {
        // cell is changed, update neighbors
        let mut neighbors = wave.get_neighbors(pos);
        neighbors.iter().for_each(|neighbor| {
            cell_update_events.send(CellUpdateEvent(*neighbor));
        });
    }
}

pub fn cell_listener(
    mut commands: Commands,
    mut cell_possable: Query<(
        Entity,
        &mut CellPossable,
        &CellPosition,
        Changed<CellPossable>,
    )>,
    wave: Res<Wave>,
) {
    let changed = cell_possable
        .iter()
        .filter(|(_, _, _, c)| *c)
        .map(|(e, possable, pos, _)| e)
        .collect::<Vec<_>>();

    // Look for cell_possable changes cells
    for (e, mut possable, pos, changed) in cell_possable.iter() {
        // cell is changed, update neighbors
        let mut neighbors = wave.get_neighbors(pos);
        if changed {
            info!("cell changed")
            // Update neighbors if needed

            //    .filter( |(_, possable, pos)| neighbors.contains(pos) )
            //    .for_each(|(e, mut possable, pos)| {
            //             let allowed = possable.allowed_neighbors();
            //             possable.0.retain(|t| allowed.contains(t));
            //             info!("{:?} {:?}", pos, possable.0);
            //       });
        }
    }
}

fn spawn_tiles(
    mut commands: Commands,
    wave: Res<Wave>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in 0..wave.width {
        for y in 0..wave.height {
            commands
                // Note: TextMesh doesnt expose TextMeshState, so have to add it this way
                // .spawn_bundle(TextMeshBundle {
                //     text_mesh:  TextMesh {
                //         text: format!("({x},{y})"),
                //         style: style.font_3d_style.clone(),
                //         ..Default::default()
                //     },
                //     transform: Transform::from_xyz(x as f32 * wave.cell_size,0.25,-(y as f32 * wave.cell_size)),
                //     ..default()
                // })
                .spawn_bundle((
                    meshes.add(shape::Cube::new(0.9).into()),
                    materials.add(Color::BLACK.into()),
                    Transform::from_xyz(x as f32 * wave.cell_size,0.25,-(y as f32 * wave.cell_size)),
                    GlobalTransform::default(),
                    Visibility::default(),
                    ComputedVisibility::default(),
                    CellPosition { x, y },
                    CellPossable(Tiles::values())
                ));
        }
    }
}

impl Wave {

    // fn propagate_cell(&mut self, x: usize, y: usize) {
    //     // get our current cell values
    //     let current = match &self.grid[x][y] {
    //         Cell::Possable(list) => list.clone(),
    //         Cell::Fixed(c) => vec![*c],
    //     };

    //     // get possable allowed values for neighbors
    //     let allowed_neigbors = current
    //         .iter()
    //         .flat_map(|c| T::allowed_neighbors(c))
    //         .collect::<Vec<_>>();

    //     // for each neighbor, check if it is allowed, remove it and add to changed list
    //     for (nx, ny) in self.get_neighbors(x, y) {
    //         let cell = self.grid[nx][ny].clone();
    //         if let Cell::Possable(mut neighbor_values) = cell {
    //             let remove = neighbor_values
    //                 .iter()
    //                 .filter(|n| !allowed_neigbors.contains(n))
    //                 .map(|n| *n)
    //                 .collect::<Vec<_>>();

    //             neighbor_values.retain(|x| remove.contains(x));
    //         }
    //     }
    // }

    pub fn get_neighbors(&self, pos: &CellPosition) -> Vec<CellPosition> {
        let mut neighbors = Vec::new();
        for x_offset in -1..2 {
            for y_offset in -1..2 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }

                let x_neighbor = pos.x as i32 + x_offset;
                let y_neighbor = pos.y as i32 + y_offset;
                if x_neighbor >= 0
                    && x_neighbor < self.width as i32
                    && y_neighbor >= 0
                    && y_neighbor < self.height as i32
                {
                    neighbors.push(CellPosition {
                        x: x_neighbor as usize,
                        y: y_neighbor as usize,
                    });
                }
            }
        }
        neighbors
    }
}
