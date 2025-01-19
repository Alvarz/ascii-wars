use bevy::prelude::*;

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

const SPEED: f32 = 10.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::NewGame), spawn_player);
    app.add_systems(Update, movement.run_if(in_state(GameState::Playing)));
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
