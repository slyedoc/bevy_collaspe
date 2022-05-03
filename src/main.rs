#![allow(warnings)]

mod ascii;
mod cleanup;
mod camera_controller;
mod fadeout;
mod enviroment;
mod tiles;
mod wave;
mod assets;
mod start_menu;
mod overworld;
mod overlay;
mod sudoku;
mod ui;

use bevy_asset_loader::AssetLoader;
use ascii::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_tweening::TweeningPlugin;
use camera_controller::*;
use enviroment::*;
use tiles::Tiles;
use wave::*;
use assets::*;
use fadeout::*;
use start_menu::*;
use overworld::*;
use sudoku::*;
use cleanup::*;
use overlay::*;
use ui::*;

use bevy::{prelude::*, app::AppExit, ui::UiPlugin};
use bevy_editor_pls::prelude::*;
use bevy_text_mesh::*;
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;
const TILE_SIZE: f32 = 1.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    AssetLoading,
    StartMenu,
    Sudoku,
    Overworld,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::AssetLoading)
        .add_plugin(AssetPlugin {
            init_state: GameState::Sudoku,
        })
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        
        // 3rd Party
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(TextMeshPlugin)
        //.add_plugin(EditorPlugin)
        .add_plugin(WorldInspectorPlugin::new())

        

        // Local Plugins
        .add_plugin(EnviromentPlugin)
        .add_plugin(StartMenuPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(FadeoutPlugin)
        .add_plugin(CleanupPlugin)
        //.add_plugin(OverlayPlugin)
        .add_plugin(SudokuPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(UiPlugin)
        //.add_plugin(WavePlugin)

        
        //.add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        //.add_plugin(bevy_transform_gizmo::TransformGizmoPlugin)

        // Global Setup
        .add_startup_system(setup)
        .add_system(update_escape)
        .run();


}

fn setup(mut commands: Commands) {
    // Global UI Camera
    commands.spawn_bundle(UiCameraBundle::default())
    .insert(Name::new("Global UI Camera"));
}

fn update_escape(
    mut commands: Commands,
    mut keys : ResMut<Input<KeyCode>>,
    mut state : Res<State<GameState>>,
    mut fadeout: EventWriter<FadeoutEvent>,
    mut app_exit: EventWriter<AppExit>,
 ) {
     if keys.just_pressed(KeyCode::Escape) {
         if *state.current() == GameState::Sudoku {
            app_exit.send(AppExit);
         } else {
            fadeout.send(FadeoutEvent(None));
         }
     }
 }