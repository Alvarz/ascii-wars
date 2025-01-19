use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::game::{ApplyMove, Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, keyboard_events);
}

fn keyboard_events(
    mut commands: Commands,
    mut evr_kbd: EventReader<KeyboardInput>,
    query: Query<Entity, With<Player>>,
) {
    for ev in evr_kbd.read() {
        match ev.state {
            ButtonState::Pressed => {
                let mut move_dir = Vec3::ZERO;
                match ev.key_code {
                    KeyCode::ArrowUp => {
                        move_dir.y = 1.0;
                    }
                    KeyCode::ArrowDown => {
                        move_dir.y -= 1.0;
                    }
                    KeyCode::ArrowLeft => {
                        move_dir.x -= 1.0;
                    }
                    KeyCode::ArrowRight => {
                        move_dir.x = 1.0;
                    }
                    _ => {}
                }

                for e in &query {
                    commands.entity(e).insert(ApplyMove { move_dir });
                }
            }
            _ => {}
        }
    }
}
