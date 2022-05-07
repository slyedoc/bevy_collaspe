
use std::fmt::Debug;

use bevy::prelude::*;

use crate::assets::{UiColors, UiFont};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_system);
    }
}

fn create_button_system(
    mut interaction_query: Query<(&mut UiColor), (Added<UiColor>, With<Button>)>,
    mut ui_colors: Res<UiColors>,
) {
    for (mut color) in interaction_query.iter_mut() {
        if color.0 == Color::WHITE {
            *color = UiColor(ui_colors.normal_button);
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Button, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut ui_colors: Res<UiColors>,
) {
    for (interaction, mut color, btn, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = ui_colors.pressed_button.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Hover".to_string();
                *color = ui_colors.hovered_button.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Button".to_string();
                *color = ui_colors.normal_button.into();
            }
        }
    }
}


pub fn create_button(
    btn: impl Component + Clone + Debug,
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
        .insert(btn.clone())
        .insert(Name::new(format!("{:?} Button", btn.clone())));
}
