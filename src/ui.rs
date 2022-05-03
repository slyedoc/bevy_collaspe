use bevy::prelude::*;

use crate::assets::UiColors;

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
