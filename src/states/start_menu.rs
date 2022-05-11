use crate::{
    assets::{UiColors, UiFont, UiSize},
    systems::{cleanup_system, CameraController, FadeoutEvent, Ground, Sun},
    GameState,
};
use bevy::{
    app::AppExit,
    math::{vec2, vec3},
    prelude::*,
    ui::FocusPolicy,
};

pub struct StartMenuPlugin;
#[derive(Component)]
struct StartMenu;

#[derive(Debug, Component, Clone, Copy)]
enum Button {
    Overworld,
    Sudoku,
    Tetris,
    Breakout,
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
        .add_system_set(
            SystemSet::on_pause(GameState::StartMenu).with_system(cleanup_system::<StartMenu>),
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
                Button::Overworld => fadeout.send(FadeoutEvent(Some(GameState::Overworld))),
                Button::Breakout => fadeout.send(FadeoutEvent(Some(GameState::Breakout))),
                Button::Sudoku => fadeout.send(FadeoutEvent(Some(GameState::Sudoku))),
                Button::Tetris => fadeout.send(FadeoutEvent(Some(GameState::Tetris))),
                Button::Exit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup_menu_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ui_font: Res<UiFont>,
    ui_size: Res<UiSize>,
    ui_colors: Res<UiColors>,
) {
    // Title Bar
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                position: Rect {
                    top: Val::Percent(10.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            color: ui_colors.ui_background.into(),
            ..Default::default()
        })
        .insert(Name::new("Title Bar"))
        .insert(StartMenu)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Sly Games",
                        TextStyle {
                            font: ui_font.base.clone(),
                            //ui_font.base.clone(),
                            font_size: 90.0,
                            color: Color::GOLD,
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(Name::new("Title"));

            // dev tag
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    color: Color::RED.into(),
                    ..Default::default()
                })
                .insert(Name::new("Dev Tag"))
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "from TwinGames",
                                TextStyle {
                                    font: ui_font.base.clone(),
                                    //ui_font.base.clone(),
                                    font_size: ui_size.label,
                                    color: Color::GOLD,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(Name::new("Dev"));
                });
        });

    // Menu
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(40.0),
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
        .insert(StartMenu)
        .with_children(|parent| {
            create_menu_button(Button::Overworld, "Overworld", parent, &ui_font);
            create_menu_button(Button::Breakout, "Breakout", parent, &ui_font);
            create_menu_button(Button::Sudoku, "Sudoku", parent, &ui_font);
            create_menu_button(Button::Tetris, "Tetris", parent, &ui_font);
            create_menu_button(Button::Exit, "Exit", parent, &ui_font);
        });
}

fn setup_menu_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::rgb(0.21, 0.36, 0.43) * 2.0;

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController::default())
        .insert(Name::new("Camera3d"))
        .insert(StartMenu);

    // light
    commands.spawn().insert(Sun).insert(StartMenu);

    // Ground
    commands.spawn().insert(Ground).insert(StartMenu);

    // setup box
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Cube"))
        .insert(StartMenu);
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
                margin: Rect::all(Val::Px(10.0)),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
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
