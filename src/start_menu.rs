use crate::{
    assets::{UiColors, UiFont},
    enviroment::*,
    CameraController,
};
use bevy::{app::AppExit, math::vec3, prelude::*, ui::FocusPolicy};

use crate::{ascii::AsciiSheet, cleanup, fadeout::FadeoutEvent, GameState};

pub struct StartMenuPlugin;

#[derive(Component)]
enum Button {
    Start,
    Exit,
}

impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(setup_menu))
            .add_system_set(SystemSet::on_resume(GameState::StartMenu).with_system(setup_menu))
            .add_startup_system(setup_light)
            .add_startup_system(setup_ground)
            .add_system(button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Button, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut ui_colors: Res<UiColors>,
    mut fadeout: EventWriter<FadeoutEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color, btn, children) in interaction_query.iter_mut() {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                //text.sections[0].value = "Press".to_string();
                match btn {
                    Button::Start => {
                        fadeout.send(FadeoutEvent(Some(GameState::Overworld)));
                    }
                    Button::Exit => {
                        exit.send(AppExit);
                    }
                }
                *color = ui_colors.normal_button.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();

                *color = ui_colors.hovered_button.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = ui_colors.pressed_button.into();
            }
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ui_font: Res<UiFont>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera"))
        .insert(cleanup::StartMenu);

    // setup box
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Background"))
        .insert(cleanup::StartMenu);

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
            // left vertical fill (border)
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
                .insert(Name::new("Left Border"))
                .with_children(|parent| {
                    // left vertical fill (content)
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
                                        "Towles Family Adventure",
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
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                                        margin: Rect::all(Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_self: AlignSelf::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Button",
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
                                .insert(Button::Start)
                                .insert(Name::new("Start Button"));

                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                                        margin: Rect::all(Val::Auto),
                                        justify_content: JustifyContent::Center,
                                        align_self: AlignSelf::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                //.insert(ButtonActive(true))
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Exit",
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
                                .insert(Button::Exit)
                                .insert(Name::new("Exit Button"));
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
                .with_children(|parent| {
                    

                });
            // // absolute positioning
            // parent
            //     .spawn_bundle(NodeBundle {
            //         style: Style {
            //             size: Size::new(Val::Px(200.0), Val::Px(200.0)),
            //             position_type: PositionType::Absolute,
            //             position: Rect {
            //                 left: Val::Px(210.0),
            //                 bottom: Val::Px(10.0),
            //                 ..Default::default()
            //             },
            //             border: Rect::all(Val::Px(20.0)),
            //             ..Default::default()
            //         },
            //         color: Color::rgb(0.4, 0.4, 1.0).into(),
            //         ..Default::default()
            //     })
            //     .with_children(|parent| {
            //         parent.spawn_bundle(NodeBundle {
            //             style: Style {
            //                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            //                 ..Default::default()
            //             },
            //             color: Color::rgb(0.8, 0.8, 1.0).into(),
            //             ..Default::default()
            //         });
            //     });
        });

    // Exit
}
