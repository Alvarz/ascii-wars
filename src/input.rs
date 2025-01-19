use bevy::{prelude::*, state::commands, window::PrimaryWindow};

use crate::{
    camera::MainCamera,
    player::{ApplyMove, ApplyRotation, Player},
    shoot::WantToShoot,
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, keyboard_events.run_if(in_state(GameState::Playing)));
    app.add_systems(Update, mouse_movement.run_if(in_state(GameState::Playing)));
    app.add_systems(
        Update,
        mouse_click_input.run_if(in_state(GameState::Playing)),
    );
}

fn keyboard_events(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Player>>,
) {
    let mut move_dir = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        move_dir.y = 1.0;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        move_dir.y -= 1.0;
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        move_dir.x -= 1.0;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
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

fn mouse_movement(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    players: Query<Entity, With<Player>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = cameras.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = windows.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        let e = players.single();
        commands.entity(e).insert(ApplyRotation {
            rotate_to: Vec3::new(world_position.x, world_position.y, 0.0),
        });
    }
}

fn mouse_click_input(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    players: Query<Entity, With<Player>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // info!("start firing ");
    }

    if mouse_button_input.pressed(MouseButton::Left) {
        // info!("firing ");
        // Continue firing bullets
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = cameras.single();

        // There is only one primary window, so we can similarly get it from the query:
        let window = windows.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
            .map(|ray| ray.unwrap().origin.truncate())
        {
            info!("Shoot click");
            let e = players.single();
            commands.entity(e).insert(WantToShoot {
                dir: Vec3::new(world_position.x, world_position.y, 0.0),
            });
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        // Stop firing bullets
        info!("stop firing ");
    }
}
