use std::f32::consts::PI;

use bevy::{math::vec2, prelude::*, render::camera::Camera2d};
use bevy_mod_picking::PickingCameraBundle;

use crate::{camera_controller::CameraController, cleanup, GameState};

pub struct SudokuPlugin;

impl Plugin for SudokuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SudokuState>()
            .init_resource::<SudokuAssets>()
            .add_system_set(SystemSet::on_enter(GameState::Sudoku).with_system(setup_sudoku));
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Number(u8),
}

pub struct SudokuAssets {
    pub line: Handle<StandardMaterial>,
    pub empty: Handle<StandardMaterial>,
    pub red: Handle<StandardMaterial>,
}

impl FromWorld for SudokuAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let line = materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 0.0, 0.0),
            unlit: true,
            ..Default::default()
        });

        let empty = materials.add(StandardMaterial {
            base_color: Color::rgb(0.0, 1.0, 0.1),
            unlit: true,
            ..Default::default()
        });

        let red = materials.add(StandardMaterial {
            base_color: Color::RED,
            unlit: true,
            ..Default::default()
        });

        Self { line, empty, red }
    }
}

struct SudokuState {
    pub cell_size_half: f32,
    pub line_size: f32,
    pub board: [[Tile; 9]; 9],
    pub selected_cell: Option<(usize, usize)>,
    pub solved: bool,
}

impl Default for SudokuState {
    fn default() -> Self {
        let mut board = [[Tile::Empty; 9]; 9];
        Self {
            cell_size_half: 0.5,
            line_size: 0.05,
            board,
            selected_cell: None,
            solved: false,
        }
    }
}

fn setup_sudoku(
    mut commands: Commands,
    mut state: ResMut<SudokuState>,
    mut query: Query<(&OrthographicProjection)>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut clear_color: ResMut<ClearColor>,
    sudoku_assets: Res<SudokuAssets>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera3d"))
        .insert_bundle(PickingCameraBundle::default())
        .insert(cleanup::Sudoku);

    clear_color.0 = Color::WHITE;

    let board = create_board(&mut commands, &state, &mut meshes, &sudoku_assets);

    
}

fn create_board(
    commands: &mut Commands,
    state: &SudokuState,
    meshes: &mut Assets<Mesh>,
    sudoku_assets: &SudokuAssets,
) -> Entity {
    let board = commands
        .spawn_bundle(TransformBundle::default())
        .insert(cleanup::Sudoku)
        .insert(Name::new("Board"))
        .id();

    let side = state.cell_size_half * 2.0 * 9.0;

    // draw grid
    for row in 0..=9 {
        commands.entity(board).with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_translation(Vec3::new(
                        row as f32 * (state.cell_size_half * 2.0) - side / 2.0,
                        0.0,
                        0.0,
                    )),
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(
                            match row % 3 == 0 {
                                true => state.line_size * 2.0,
                                false => state.line_size,
                            },
                            side + (state.line_size * 2.0),
                        ),
                        flip: false,
                    })),
                    material: sudoku_assets.line.clone(),
                    ..Default::default()
                })
                .insert(Name::new(format!("Row {row}")));
        });
    }
    for col in 0..=9 {
        commands.entity(board).with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        col as f32 * (state.cell_size_half * 2.0) - side / 2.0,
                        0.0,
                    )),
                    mesh: meshes.add(Mesh::from(shape::Quad {
                        size: Vec2::new(
                            side + (state.line_size * 2.0),
                            match col % 3 == 0 {
                                true => state.line_size * 2.0,
                                false => state.line_size,
                            },
                        ),
                        flip: false,
                    })),
                    material: sudoku_assets.line.clone(),
                    ..Default::default()
                })
                .insert(Name::new(format!("Col {col}")));
        });
    }

    // Create Tiles
    for i in 0..9 {
        for j in 0..9 {
            let cell = state.board[i][j];

            commands.entity(board).with_children(|parent| {
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane {
                        size: state.cell_size_half * 2.0 * 0.8,
                    })),
                    material: match cell {
                        Tile::Empty => sudoku_assets.empty.clone(),
                        Tile::Number(_) => sudoku_assets.red.clone(),
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            (i as f32 * state.cell_size_half * 2.0) - (side * 0.5) + state.cell_size_half,
                        (j as f32 * state.cell_size_half * 2.0) - (side * 0.5) + state.cell_size_half,
                        0.0,
                        ),
                        rotation: Quat::from_rotation_x(PI / 2.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Name::new(format!("Cell ({},{})", i, j)));
            });
        }
    }
    board
}
