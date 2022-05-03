use crate::{
    assets::{UiColors, UiFont},
    enviroment::*,
    CameraController,
};
use bevy::{app::AppExit, math::vec3, prelude::*, ui::FocusPolicy};

use crate::{ascii::AsciiSheet, cleanup, fadeout::FadeoutEvent, GameState};

pub struct StartMenuPlugin;

#[derive(Debug, Component, Clone, Copy)]
enum Button {
    Start,
    Sudoku,
    Exit,
}

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::StartMenu)
                .with_system(setup_menu_world)
                .with_system(setup_menu_ui),
        )
        .add_system_set(
            SystemSet::on_resume(GameState::StartMenu)
                .with_system(setup_menu_world)
                .with_system(setup_menu_ui),
        )
        .add_system(button_events);
    }
}

fn button_events(
    mut interaction_query: Query<
        (&Interaction, &Button, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut fadeout: EventWriter<FadeoutEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, btn, children) in interaction_query.iter_mut() {
        if Interaction::Clicked == *interaction {
            match btn {
                Button::Start => fadeout.send(FadeoutEvent(Some(GameState::Overworld))),
                Button::Sudoku => fadeout.send(FadeoutEvent(Some(GameState::Sudoku))),
                Button::Exit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}

fn setup_menu_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ui_font: Res<UiFont>,
) {
    // Add UI
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(cleanup::StartMenu)
        .insert(Name::new("Ui"))
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .insert(Name::new("Menu"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexStart,
                                ..Default::default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Left fill content"))
                        .with_children(|parent| {
                            // Title
                            parent
                                .spawn_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(5.0)),
                                        ..Default::default()
                                    },
                                    text: Text::with_section(
                                        "Testing - TwinGames",
                                        TextStyle {
                                            font: ui_font.base.clone(),
                                            font_size: 30.0,
                                            color: Color::WHITE,
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                })
                                .insert(Name::new("Title"));

                            // start button
                            create_menu_button(Button::Start, "Start", parent, &ui_font);
                            create_menu_button(Button::Sudoku, "Sudoku", parent, &ui_font);
                            create_menu_button(Button::Exit, "Exit", parent, &ui_font);
                        });
                });

            // right vertical fill
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {});
        });

    // Exit
}


fn setup_menu_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera3d"))
        .insert(cleanup::StartMenu);

    // light
    commands.spawn().insert(Sun).insert(cleanup::StartMenu);

    // Ground
    commands.spawn().insert(Ground).insert(cleanup::StartMenu);

    // setup box
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Cube"))
        .insert(cleanup::StartMenu);
}

fn create_menu_button(
    btn: Button,
    text: impl Into<String>,
    parent: &mut ChildBuilder,
    ui_font: &UiFont,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        //.insert(ButtonActive(true))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: ui_font.base.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(btn)
        .insert(Name::new(format!("{:?} Button", btn.clone())));
}
