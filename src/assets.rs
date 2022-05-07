use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_editor_pls::default_windows::inspector;
use rand::Rng;

use crate::GameState;

pub struct AssetPlugin {
    pub init_state: GameState,
}

pub struct StartupState(pub GameState);

impl Plugin for AssetPlugin {
    fn build(&self, mut app: &mut App) {
        AssetLoader::new(GameState::AssetLoading)
            .continue_to_state(self.init_state)
            .with_collection::<UiFont>()
            .with_collection::<AsciiAssets>()
            .with_collection::<NatureKitAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<OverlayAssets>()
            .build(&mut app);

        app.insert_resource(StartupState(self.init_state))
            .init_resource::<UiSize>()
            .init_resource::<UiColors>();
    }
}

#[derive(AssetCollection)]
pub struct NatureKitAssets {
    #[asset(path = "models/kenney_nature_kit", folder(typed))]
    folder: Vec<Handle<Gltf>>,
}

#[derive(AssetCollection)]
pub struct OverlayAssets {
    #[asset(path = "images/music.png")]
    pub music: Handle<Image>,

    
}

#[derive(AssetCollection)]
pub struct UiFont {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub base: Handle<Font>,
}
pub struct UiSize {
    pub label: f32,
}



impl Default for UiSize {
    fn default() -> Self {
        UiSize {
            label: 30.0,
        }
    }
}



impl NatureKitAssets {
    pub fn rand_tree(&self) -> Handle<Gltf> {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.folder.len());
        self.folder[index].clone()
    }
}

pub struct UiColors {
    pub normal_button: Color,
    pub hovered_button: Color,
    pub pressed_button: Color,
    pub ui_background: Color,
}

impl Default for UiColors {
    fn default() -> Self {
        Self {
            normal_button: Color::rgb(0.15, 0.15, 0.15),
            hovered_button: Color::rgb(0.25, 0.25, 0.25),
            pressed_button: Color::rgb(0.35, 0.75, 0.35),
            ui_background: Color::rgba(0.1, 0.1, 0.1, 0.05),
        }
    }
}

#[derive(AssetCollection)]
pub struct AsciiAssets {
    #[asset(path = "Ascii.png")]
    pub ascii: Handle<Image>,
}

#[derive(AssetCollection)]
struct AudioAssets {
    //#[asset(path = "walking.ogg")]
//walking: Handle<AudioSource>
}

//     fn from_world(world: &mut World) -> Self {
//         let (font, font2) = world.resource_scope(|_world: &mut World, asset_server: Mut<AssetServer>| {
//                 (
//                     asset_server.load("fonts/FiraSans-Bold.ttf"),
//                     asset_server.load("fonts/FiraMono-Medium.ttf"),
//                 )
//             });
//         let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

//         let normal = materials.add(Color::rgb(0.15, 0.15, 0.15).into());
//         let hovered = materials.add(Color::rgb(0.25, 0.25, 0.25).into());
//         let pressed = materials.add(Color::rgb(0.35, 0.75, 0.35).into());

//         let mut assets = world.get_resource_mut::<AssetServer>().unwrap();

//         UiAssets {
//             font,
//             font2,
//             normal_material: normal,
//             hovered_material: hovered,
//             pressed_material: pressed,
//         }
//     }
// }

// pub struct Text3dAssets {
//     pub font_3d: Handle<TextMeshFont>,
//     pub font_3d_style: TextMeshStyle,
// }

// impl FromWorld for Text3dAssets {
//     fn from_world(world: &mut World) -> Self {
//         let (font_3d) =
//             world.resource_scope(|_world: &mut World, asset_server: Mut<AssetServer>| {
//                 (asset_server.load("fonts/FiraMono-Medium.ttf"))
//             });

//         let font_3d_style = TextMeshStyle {
//             font: font_3d.clone(),
//             font_size: SizeUnit::NonStandard(9.),
//             color: Color::rgb(0.0, 0.0, 0.0),
//             ..Default::default()
//         };

//         Text3dAssets {
//             font_3d,
//             font_3d_style,
//         }
//     }
// }
