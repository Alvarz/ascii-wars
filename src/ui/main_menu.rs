use bevy::prelude::*;

use crate::ui::ui_commons::{
    exit_button_system, spawn_box, spawn_button, spawn_container, spawn_text, ExitButton,
};
use crate::ui::ui_style::{
    BOX_BG_COLOR, BOX_BORDER_COLOR, COLOR_TEXT_BUTTON, HOVERED_BUTTON, HOVER_TEXT_COLOR,
    MAIN_TEXT_COLOR, NORMAL_BUTTON, PRESSED_BUTTON,
};
use crate::GameState;

#[derive(Resource, Clone)]
pub struct MainMenu {
    pub entity: Entity,
}

#[derive(Component, Clone)]
pub struct PlayButton;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), main_menu);
    app.add_systems(
        Update,
        play_button_system.run_if(in_state(GameState::MainMenu)),
    );

    app.add_systems(
        Update,
        exit_button_system.run_if(in_state(GameState::MainMenu)),
    );
    app.add_systems(OnExit(GameState::MainMenu), clear_main_menu);
}

fn main_menu(mut commands: Commands) {
    let container = spawn_container(&mut commands);
    commands.insert_resource(MainMenu { entity: container });
    let menu_box = spawn_box(
        &mut commands,
        container,
        Val::Percent(60.),
        Val::Percent(60.),
    );
    spawn_text(&mut commands, menu_box, "ASCII Wars!", 50.0);
    spawn_button(&mut commands, menu_box, "Play".to_string(), PlayButton, 32.);
    spawn_button(&mut commands, menu_box, "Exit".to_string(), ExitButton, 32.);
}

fn clear_main_menu(mut commands: Commands, menu: Res<MainMenu>) {
    commands.entity(menu.entity).despawn_recursive();
    commands.remove_resource::<MainMenu>();
}

fn play_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &PlayButton,
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
                next_state.set(GameState::NewGame)
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
