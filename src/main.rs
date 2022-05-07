#![allow(warnings)]


mod states;
mod systems;
mod assets;

mod math;

mod tiles;
mod ui;
mod wave;
pub mod physics;

use states::*;
use systems::*;
use assets::*;
use bevy_asset_loader::AssetLoader;
use bevy_hanabi::HanabiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;

use bevy_tweening::TweeningPlugin;

use physics::*;


use tiles::Tiles;
use ui::*;
use wave::*;

use bevy::{app::AppExit, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, ui::UiPlugin};
use bevy_editor_pls::prelude::*;
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    AssetLoading,
    StartMenu,
    Sudoku,
    Breakout,
    Overworld,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(GameState::AssetLoading)
        .add_plugin(AssetPlugin {
            init_state: GameState::Overworld,
        })
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        // 3rd Party
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(PhysicsPlugin)
        //.add_plugin(HanabiPlugin)
        .add_plugin(TweeningPlugin)
        //.add_plugin(TextMeshPlugin)
        //.add_plugin(EditorPlugin)
        //.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // Local Plugins
        .add_plugin(EnviromentPlugin)
        .add_plugin(StartMenuPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(FadeoutPlugin)
        .add_plugin(OverlayPlugin)
        .add_plugin(CameraControllerPlugin)
        .add_plugin(UiPlugin)
        // Levels
        .add_plugin(SudokuPlugin)
        .add_plugin(OverworldPlugin)
        .add_plugin(BreakoutPlugin)
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
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Name::new("Global UI Camera"));
}

fn update_escape(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    mut state: Res<State<GameState>>,
    mut fadeout: EventWriter<FadeoutEvent>,
    mut app_exit: EventWriter<AppExit>,
    starting_state: Res<StartupState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if *state.current() == starting_state.0 {
            app_exit.send(AppExit);
        } else {
            fadeout.send(FadeoutEvent(None));
        }
    }
}
