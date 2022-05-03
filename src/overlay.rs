use bevy::prelude::*;

use crate::{
    assets::{OverlayAssets, UiColors, UiFont},
    GameState,
};

const OVERLAY_INDEX: usize = 10;
pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(GameState::AssetLoading).with_system(setup_overlay))
            .add_system(update_gamestate);
    }
}

fn setup_overlay(
    mut commands: Commands,
    ui_font: Res<UiFont>,
    ui_colors: Res<UiColors>,
    over_assets: Res<OverlayAssets>,
) {
    // Bottom Right
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(100.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                border: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            color: ui_colors.ui_background.into(),
            transform: Transform::from_xyz(0.0, 0.0, OVERLAY_INDEX as f32),
            ..Default::default()
        })
        .insert(Name::new("Overlay Bottom Right"))
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(5.0),
                            right: Val::Px(15.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        // Accepts a `String` or any type that converts into a `String`, such as `&str`
                        "GameState",
                        TextStyle {
                            font: ui_font.base.clone(),
                            font_size: 100.0,
                            color: Color::WHITE,
                        },
                        // Note: You can use `Default::default()` in place of the `TextAlignment`
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(Name::new("GameState"));
        });

    // top menu
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                border: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            color: ui_colors.ui_background.into(),
            ..Default::default()
        })
        .insert(Name::new("Overlay Top Menu"))
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(5.0),
                            right: Val::Px(15.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    image: over_assets.music.clone().into(),
                    ..Default::default()
                })
                .insert(Name::new("State: <Something>"));
        });
}

#[derive(Component)]
struct GameStateText;

fn update_gamestate(
    mut text_query: Query<&mut Text, With<GameStateText>>,
    game_state: Res<State<GameState>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("state: {:?}", game_state.current());
    }
}
