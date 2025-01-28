use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    assets::CharsetAsset,
    enemies::Boss,
    game::{GamePlayEntity, Pool},
    GameState,
};

#[derive(Component)]
pub struct ApplyMove {
    pub move_dir: Vec3,
}

#[derive(Component)]
pub struct Player;

const SPEED: f32 = 250.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, movement.run_if(in_state(GameState::Playing)));
    app.add_systems(Update, rotation.run_if(in_state(GameState::Playing)));
    app.add_systems(
        Update,
        confine_player_movement.run_if(in_state(GameState::Playing)),
    );
}
pub fn spawn_player(commands: &mut Commands, chaset: &CharsetAsset, window: &Window) {
    let spawn_pos_x = window.width() * 0.5;
    let spawn_pos_y = window.height() * 0.5;

    commands.spawn((
        Sprite {
            image: chaset.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: chaset.atlas.clone(),
                index: '>' as usize,
            }),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(spawn_pos_x, spawn_pos_y - 300., 0.),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.3, 1.3, 0.),
        },
        Player {},
        Pool {
            health: 10.,
            max_health: 10.,
            damage: 2.,
            god_mode: false,
        },
        GamePlayEntity,
    ));
}

fn rotation(
    mut player: Single<&mut Transform, (With<Player>, Without<Boss>)>,
    boss: Single<&Transform, (With<Boss>, Without<Player>)>,
) {
    let player_transform = &mut player;

    let to_position = (boss.translation.xy() - player_transform.translation.xy()).normalize();
    let rotation = Quat::from_rotation_arc(Vec3::X, to_position.extend(0.));

    player_transform.rotation = rotation;
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
