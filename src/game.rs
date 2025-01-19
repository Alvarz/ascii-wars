use bevy::{prelude::*, window::PrimaryWindow};

use crate::{assets::CharsetAsset, GameState};

#[derive(Component)]
pub struct ApplyMove {
    pub move_dir: Vec3,
}
#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerEntity {
    pub entity: Entity,
}

const SPEED: f32 = 500.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::NewGame), spawn_player);
    app.add_systems(Update, movement.run_if(in_state(GameState::Playing)));
    app.add_systems(
        Update,
        confine_player_movement.run_if(in_state(GameState::Playing)),
    );
}
fn spawn_player(
    mut commands: Commands,
    chaset: Res<CharsetAsset>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let entity = commands
        .spawn((
            Sprite {
                image: chaset.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: chaset.atlas.clone(),
                    index: '@' as usize,
                }),
                ..Default::default()
            },
            Player {},
        ))
        .id();

    commands.insert_resource(PlayerEntity { entity });
    next_state.set(GameState::Playing);
}

fn movement(
    time: Res<Time>,
    mut commands: Commands,
    mut movers: Query<(Entity, &mut Transform, &ApplyMove)>,
) {
    for (e, mut transform, movement) in &mut movers.iter_mut() {
        transform.translation += movement.move_dir * SPEED * time.delta_secs();
        commands.entity(e).remove::<ApplyMove>();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let half_player_size = 8.0 * 0.5; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
