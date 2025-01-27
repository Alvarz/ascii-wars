use bevy::prelude::*;

use crate::{
    enemies::Boss,
    player::{ApplyMove, Player},
    shoot::WantToShoot,
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, keyboard_events.run_if(in_state(GameState::Playing)));
}

fn keyboard_events(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    boss: Single<&Transform, (With<Boss>, Without<Player>)>,
) {
    let mut move_dir = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        move_dir.y = 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        move_dir.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        move_dir.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        move_dir.x = 1.0;
    }

    if move_dir != Vec3::ZERO {
        for e in &query {
            commands.entity(e).insert(ApplyMove {
                move_dir: move_dir.normalize(),
            });

            let mut directions = Vec::new();
            directions.push(boss.translation);
            commands.entity(e).insert(WantToShoot {
                dir: directions,
                entity: e,
            });
        }
    }

    if keyboard_input.pressed(KeyCode::Escape) {
        next_state.set(GameState::PauseMenu)
    }
}
