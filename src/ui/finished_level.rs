use bevy::prelude::*;

use crate::ui::ui_commons::{spawn_box, spawn_button, spawn_container, spawn_text};
use crate::ui::ui_style::{
    BOX_BORDER_COLOR, HOVERED_BUTTON, HOVER_TEXT_COLOR, MAIN_TEXT_COLOR, NORMAL_BUTTON,
    PRESSED_BUTTON,
};
use crate::GameState;

#[derive(Resource, Clone)]
pub struct FinishedLevel {
    entity: Entity,
}

#[derive(Component, Clone)]
pub struct NextLevelButton;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::FinishedLevel), finished_level);
    app.add_systems(
        Update,
        continue_button_system.run_if(in_state(GameState::FinishedLevel)),
    );

    app.add_systems(OnExit(GameState::FinishedLevel), clear_finished_level);
}

fn finished_level(mut commands: Commands) {
    let container = spawn_container(&mut commands);
    commands.insert_resource(FinishedLevel { entity: container });

    let menu_box = spawn_box(
        &mut commands,
        container,
        Val::Percent(40.),
        Val::Percent(60.),
    );

    spawn_text(&mut commands, menu_box, "You Won!", 20.0);
    spawn_button(
        &mut commands,
        menu_box,
        "Continue Next Level".to_string(),
        NextLevelButton,
        18.,
    );
}

fn clear_finished_level(mut commands: Commands, menu: Res<FinishedLevel>) {
    commands.entity(menu.entity).despawn_recursive();
    commands.remove_resource::<FinishedLevel>();
}

fn continue_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &NextLevelButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_color_query: Query<&mut TextColor>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color, children, _) in &mut interaction_query {
        let mut text_color = text_color_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = HOVER_TEXT_COLOR.into();
                next_state.set(GameState::NextLevel);
            }

            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = HOVER_TEXT_COLOR.into()
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = BOX_BORDER_COLOR.into();
                *text_color = MAIN_TEXT_COLOR.into()
                // border_color.0 = NORMAL_BUTTON.into();
            }
        }
    }
}
