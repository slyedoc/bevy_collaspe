// TODO: See for sain way to handle rotations
// https://github.com/mikejquinn/rust-tetris/blob/master/src/main.rs
use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::camera::Camera2d,
    utils::HashMap,
};
use bevy_mod_picking::{PickableBundle, PickingCameraBundle, PickingEvent};

use crate::{
    assets::{UiColors, UiFont},
    systems::{cleanup_system, CameraController},
    ui::{self, create_button},
    GameState,
};

pub struct TetrisPlugin;

#[derive(Component)]
struct Tetris;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<TetrisAssets>()
            .add_system_set(SystemSet::on_enter(GameState::Tetris).with_system(setup_tetris))
            .add_system_set(
                SystemSet::on_exit(GameState::Tetris).with_system(cleanup_system::<Tetris>),
            );
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum Peice {
    O,
    S,
    Z,
    T,
    L,
    J,
    I,
}

const PEICES: [Peice; 7] = [
    // Enumiter replacement
    Peice::O,
    Peice::S,
    Peice::Z,
    Peice::T,
    Peice::L,
    Peice::J,
    Peice::I,
];

impl Peice {
    pub fn color(&self) -> Color {
        match self {
            Peice::O => Color::YELLOW,
            Peice::S => Color::GREEN,
            Peice::Z => Color::RED,
            Peice::T => Color::PURPLE,
            Peice::L => Color::ORANGE,
            Peice::J => Color::BLUE,
            Peice::I => Color::AQUAMARINE,
        }
    }

    pub fn positions(&self) -> [Vec3; 4] {
        match self {
            Peice::O => [
                vec3(-BRICK_SIZE, 0.0, 0.0),
                vec3(-BRICK_SIZE, -BRICK_SIZE, 0.0),
                Vec3::ZERO,
                vec3(0.0, -BRICK_SIZE, 0.0),
            ],
            Peice::S => [
                Vec3::ZERO,
                vec3(BRICK_SIZE, 0.0, 0.0),
                vec3(0.0, -BRICK_SIZE, 0.0),
                vec3(-BRICK_SIZE, -BRICK_SIZE, 0.0),
            ],
            Peice::Z => [
                Vec3::ZERO,
                vec3(BRICK_SIZE, 0.0, 0.0),
                vec3(0.0, -BRICK_SIZE, 0.0),
                vec3(-BRICK_SIZE, -BRICK_SIZE, 0.0),
            ],
            Peice::T => [
                Vec3::ZERO,
                vec3(-BRICK_SIZE, -BRICK_SIZE, 0.0),
                vec3(0.0, -BRICK_SIZE, 0.0),
                vec3(BRICK_SIZE, -BRICK_SIZE, 0.0),
            ],
            Peice::L => [
                vec3(-BRICK_SIZE * 2.0, -BRICK_SIZE, 0.0),
                vec3(-BRICK_SIZE, -BRICK_SIZE, 0.0),
                vec3(0.0, -BRICK_SIZE, 0.0),
                vec3(0.0, 0.0, 0.0),
            ],
            Peice::J => [
                vec3(-BRICK_SIZE, 0.0, 0.0),
                vec3(-BRICK_SIZE, -BRICK_SIZE, 0.0),
                vec3(0.0, -BRICK_SIZE, 0.0),
                vec3(BRICK_SIZE, -BRICK_SIZE, 0.0),
            ],
            Peice::I => [
                vec3(-BRICK_SIZE * 2.0, 0.0, 0.0),
                vec3(-BRICK_SIZE, 0.0, 0.0),
                vec3(0.0, 0.0, 0.0),
                vec3(BRICK_SIZE, 0.0, 0.0),
            ],
        }
    }
}

enum Block {
    Empty,
    Color(),
}

const BRICK_SIZE: f32 = 1.0;

pub struct TetrisAssets {
    pub brick: Handle<Mesh>,
    pub peices: HashMap<Peice, Handle<StandardMaterial>>,
}

impl FromWorld for TetrisAssets {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let mut asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let block_img = asset_server.load("images/tetris_block.png");

        let mut peices = HashMap::with_capacity(PEICES.len());

        for p in PEICES {
            let color = p.color();
            peices.insert(
                p,
                materials.add(StandardMaterial {
                    base_color: color,
                    unlit: true,
                    base_color_texture: Some(block_img.clone()),
                    ..Default::default()
                }),
            );
        }

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let brick_mesh = meshes.add(Mesh::from(shape::Quad::new(vec2(BRICK_SIZE, BRICK_SIZE))));

        Self {
            peices,
            brick: brick_mesh,
        }
    }
}

struct Board(u8, u8);

impl Default for Board {
    fn default() -> Self {
        Self(8, 40)
    }
}

fn setup_tetris(
    mut commands: Commands,
    mut query: Query<(&OrthographicProjection)>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut clear_color: ResMut<ClearColor>,
    tetris_assets: Res<TetrisAssets>,
) {
    info!("Tetris");

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera3d"))
        .insert_bundle(PickingCameraBundle::default())
        .insert(Tetris);

    clear_color.0 = Color::WHITE;

    let mut pos = Vec3::ZERO;
    for p in PEICES {
        spawn_peice(p, pos, &mut commands, &tetris_assets);
        pos.x += 4.0;
    }
}

fn spawn_peice(
    peice: Peice,
    pos: Vec3,
    commands: &mut Commands,
    assets: &TetrisAssets,
) -> Vec<Entity> {
    peice
        .positions()
        .iter()
        .map(|p| spawn_brick(&peice, *p + pos, commands, assets))
        .collect::<Vec<Entity>>()
}

fn spawn_brick(p: &Peice, pos: Vec3, commands: &mut Commands, assets: &TetrisAssets) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(pos),
            mesh: assets.brick.clone(),
            material: assets.peices[&p].clone(),
            ..Default::default()
        })
        .insert(Tetris)
        .insert(Name::new("Brick"))
        .id()
}
#[derive(Component, Debug, Clone)]
enum Button {
    Show,
    New,
    Solve,
    Clear,
}

fn create_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    ui_colors: Res<UiColors>,
    ui_font: Res<UiFont>,
) {
    // Menu
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Percent(10.0),
                    left: Val::Percent(30.0),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(40.0), Val::Percent(40.0)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Stretch,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: ui_colors.ui_background.into(),
            ..Default::default()
        })
        .insert(Name::new("Menu"))
        .insert(Tetris)
        .with_children(|parent| {
            create_button(Button::Show, "Show", parent, &ui_font);
        });
}
