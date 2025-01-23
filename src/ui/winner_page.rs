use bevy::prelude::*;

use crate::ui::ui_commons::{
    exit_button_system, play_again_button_system, spawn_box, spawn_button, spawn_container,
    spawn_text, ExitButton, PlayAgainButton,
};
use crate::ui::ui_style::{
    BOX_BG_COLOR, BOX_BORDER_COLOR, COLOR_TEXT_BUTTON, HOVERED_BUTTON, HOVER_TEXT_COLOR,
    MAIN_TEXT_COLOR, NORMAL_BUTTON, PRESSED_BUTTON,
};
use crate::GameState;

#[derive(Resource, Clone)]
pub struct WinnerPage {
    entity: Entity,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::WinGame), win_game_page);
    app.add_systems(
        Update,
        play_again_button_system.run_if(in_state(GameState::WinGame)),
    );

    app.add_systems(
        Update,
        exit_button_system.run_if(in_state(GameState::WinGame)),
    );
    app.add_systems(OnExit(GameState::WinGame), clear_win_game_page);
}

fn win_game_page(mut commands: Commands) {
    let container = spawn_container(&mut commands);

    commands.insert_resource(WinnerPage { entity: container });
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
        "Play Again".to_string(),
        PlayAgainButton,
        18.,
    );
    spawn_button(&mut commands, menu_box, "Exit".to_string(), ExitButton, 18.);
}

fn clear_win_game_page(mut commands: Commands, menu: Res<WinnerPage>) {
    commands.entity(menu.entity).despawn_recursive();
    commands.remove_resource::<WinnerPage>();
}
