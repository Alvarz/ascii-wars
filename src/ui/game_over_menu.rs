use bevy::prelude::*;

use crate::ui::ui_commons::{
    exit_button_system, play_again_button_system, spawn_box, spawn_button, spawn_container,
    spawn_text, ExitButton, PlayAgainButton,
};
use crate::GameState;

#[derive(Resource, Clone)]
pub struct PauseMenu {
    entity: Entity,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), game_over_menu);
    app.add_systems(
        Update,
        play_again_button_system.run_if(in_state(GameState::GameOver)),
    );

    app.add_systems(
        Update,
        exit_button_system.run_if(in_state(GameState::GameOver)),
    );
    app.add_systems(OnExit(GameState::GameOver), clear_game_over_menu);
}

fn game_over_menu(mut commands: Commands) {
    let container = spawn_container(&mut commands);
    commands.insert_resource(PauseMenu { entity: container });

    let menu_box = spawn_box(
        &mut commands,
        container,
        Val::Percent(40.),
        Val::Percent(60.),
    );

    spawn_text(&mut commands, menu_box, "Game Over!", 25.0);
    spawn_button(
        &mut commands,
        menu_box,
        "Play Again".to_string(),
        PlayAgainButton,
        20.,
    );
    spawn_button(&mut commands, menu_box, "Exit".to_string(), ExitButton, 20.);
}

fn clear_game_over_menu(mut commands: Commands, menu: Res<PauseMenu>) {
    commands.entity(menu.entity).despawn_recursive();
    commands.remove_resource::<PauseMenu>();
}
