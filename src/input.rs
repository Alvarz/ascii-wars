use bevy::prelude::*;

use crate::{
    game::{ApplyMove, Player},
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, keyboard_events.run_if(in_state(GameState::Playing)));
}

fn keyboard_events(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Player>>,
) {
    let mut move_dir = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        move_dir.y = 1.0;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        move_dir.y -= 1.0;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
        move_dir.x -= 1.0;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        move_dir.x = 1.0;
    }

    if move_dir != Vec3::ZERO {
        for e in &query {
            commands.entity(e).insert(ApplyMove {
                move_dir: move_dir.normalize(),
            });
        }
    }
}
