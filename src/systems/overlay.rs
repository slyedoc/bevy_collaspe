use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    assets::{OverlayAssets, UiColors, UiFont, UiSize},
    GameState,
};

const OVERLAY_INDEX: usize = 10;
pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(GameState::AssetLoading).with_system(setup_overlay))
            .add_system(update_gamestate)
            .add_system(update_fps);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct GameStateText;

fn setup_overlay(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    ui_colors: Res<UiColors>,
    ui_size: Res<UiSize>,
    over_assets: Res<OverlayAssets>,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..Default::default()
                },

                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Game State: ".to_string(),
                        style: TextStyle {
                            font: ui_font.base.clone(),
                            font_size: ui_size.label,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: ui_font.base.clone(),
                            font_size: ui_size.label,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("GameState"))
        .insert(GameStateText);

    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: ui_font.base.clone(),
                            font_size: ui_size.label,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: ui_font.base.clone(),
                            font_size: ui_size.label,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("FPS"))
        .insert(FpsText);

    // // top menu
    // commands
    //     .spawn_bundle(NodeBundle {
    //         style: Style {
    //             size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
    //             position_type: PositionType::Absolute,
    //             position: Rect {
    //                 top: Val::Px(10.0),
    //                 ..Default::default()
    //             },
    //             border: Rect::all(Val::Px(20.0)),
    //             ..Default::default()
    //         },
    //         color: ui_colors.ui_background.into(),
    //         ..Default::default()
    //     })
    //     .insert(Name::new("Overlay Top Menu"))
    //     .with_children(|parent| {
    //         parent
    //             .spawn_bundle(ButtonBundle {
    //                 style: Style {
    //                     align_self: AlignSelf::FlexEnd,
    //                     position_type: PositionType::Absolute,
    //                     position: Rect {
    //                         bottom: Val::Px(5.0),
    //                         right: Val::Px(15.0),
    //                         ..Default::default()
    //                     },
    //                     ..Default::default()
    //                 },
    //                 image: over_assets.music.clone().into(),
    //                 ..Default::default()
    //             })
    //             .insert(Name::new("State: <Something>"));
    //     });
}

fn update_gamestate(
    mut text_query: Query<&mut Text, With<GameStateText>>,
    game_state: Res<State<GameState>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[1].value = format!("{:?}", game_state.current());
    }
}

fn update_fps(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.0}", average);
                text.sections[1].style.color = match average {
                    x if x >= 50.0 => Color::GREEN,
                    x if x > 40.0 && x < 50.0 => Color::YELLOW,
                    x if x <= 40.0 => Color::RED,
                    _ => Color::WHITE,
                };
            }
        }
    }
}
